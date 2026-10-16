#[macro_use]
extern crate log;

use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

use anyhow::anyhow;
use channel::ChannelMessage;
#[cfg(all(feature = "experimental", target_os = "linux"))]
use hdl::BleAdvertiser;
use hdl::MDnsDiscovery;
use once_cell::sync::Lazy;
use rand::Rng;
use rand::distr::Alphanumeric;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, watch};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

#[cfg(all(feature = "experimental", target_os = "linux"))]
use crate::hdl::BleListener;
use crate::hdl::MDnsServer;
use crate::manager::TcpServer;

pub mod channel;
pub mod errors;
pub mod hdl;
pub mod manager;
pub mod utils;

pub use hdl::{EndpointInfo, OutboundPayload, TransferState, Visibility};
pub use manager::SendInfo;
pub use utils::DeviceType;

pub mod sharing_nearby {
    include!(concat!(env!("OUT_DIR"), "/sharing.nearby.rs"));
}

pub mod securemessage {
    include!(concat!(env!("OUT_DIR"), "/securemessage.rs"));
}

pub mod securegcm {
    include!(concat!(env!("OUT_DIR"), "/securegcm.rs"));
}

pub mod location_nearby_connections {
    include!(concat!(env!("OUT_DIR"), "/location.nearby.connections.rs"));
}

static CUSTOM_DOWNLOAD: Lazy<RwLock<Option<PathBuf>>> = Lazy::new(|| RwLock::new(None));
static DEVICE_NAME: Lazy<RwLock<String>> =
    Lazy::new(|| RwLock::new(sys_metrics::host::get_hostname().unwrap_or("Unknown device".into())));

/// Name shown to the other device (for sending without an `RQS` instance).
pub fn set_device_name(name: &str) {
    *DEVICE_NAME.write().unwrap() = name.to_string();
}

#[derive(Debug)]
pub struct RQS {
    tracker: Option<TaskTracker>,
    ctoken: Option<CancellationToken>,
    // Discovery token is different than ctoken because he is on his own
    // - can be cancelled while the ctoken is still active
    discovery_ctk: Option<CancellationToken>,

    // Used to trigger a change in the mDNS visibility (and later on, BLE)
    pub visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    visibility_receiver: watch::Receiver<Visibility>,

    // Only used to send the info "a nearby device is sharing"
    ble_sender: broadcast::Sender<()>,

    pub port_number: Option<u32>,

    pub message_sender: broadcast::Sender<ChannelMessage>,
}

impl Default for RQS {
    fn default() -> Self {
        Self::new(
            Visibility::Visible,
            None,
            None,
            Some(sys_metrics::host::get_hostname().unwrap_or("Unknown device".into())),
        )
    }
}

impl RQS {
    pub fn new(
        visibility: Visibility,
        port_number: Option<u32>,
        download_path: Option<PathBuf>,
        device_name: Option<String>,
    ) -> Self {
        {
            let mut guard = CUSTOM_DOWNLOAD.write().unwrap();
            *guard = download_path;
        }
        {
            let mut guard = DEVICE_NAME.write().unwrap();
            if let Some(device_name) = device_name {
                *guard = device_name.clone();
            }
        }

        let (message_sender, _) = broadcast::channel(50);
        let (ble_sender, _) = broadcast::channel(5);

        // Define default visibility as per the args inside the new()
        let (visibility_sender, visibility_receiver) = watch::channel(Visibility::Invisible);
        let _ = visibility_sender.send(visibility);

        Self {
            tracker: None,
            ctoken: None,
            discovery_ctk: None,
            visibility_sender: Arc::new(Mutex::new(visibility_sender)),
            visibility_receiver,
            ble_sender,
            port_number,
            message_sender,
        }
    }

    pub async fn run(
        &mut self,
    ) -> Result<(mpsc::Sender<SendInfo>, broadcast::Receiver<()>), anyhow::Error> {
        let tracker = TaskTracker::new();
        let ctoken = CancellationToken::new();
        self.tracker = Some(tracker.clone());
        self.ctoken = Some(ctoken.clone());

        let endpoint_id: Vec<u8> = rand::rng()
            .sample_iter(Alphanumeric)
            .take(4)
            .map(u8::from)
            .collect();
        let tcp_listener =
            TcpListener::bind(format!("0.0.0.0:{}", self.port_number.unwrap_or(0))).await?;
        let binded_addr = tcp_listener.local_addr()?;
        info!("TcpListener on: {}", binded_addr);

        // So the random port can be accessed from the user if needed.
        // This does have a difference in behaviour however when port_number is Some.
        // .stop() and .run() will reuse the port number instead of generating a new one.
        self.port_number = Some(binded_addr.port() as u32);

        // MPSC for the TcpServer
        let send_channel = mpsc::channel(10);
        // Start TcpServer in own "task"
        let mut server = TcpServer::new(
            endpoint_id[..4].try_into()?,
            tcp_listener,
            self.message_sender.clone(),
            send_channel.1,
        )?;
        let ctk = ctoken.clone();
        tracker.spawn(async move { server.run(ctk).await });

        #[cfg(all(feature = "experimental", target_os = "linux"))]
        {
            // QSD_NO_BLE_SCAN=1: no sender detection at all (see hdl/ble.rs; QSD_BLE_SCAN picks
            // how it watches).
            if std::env::var_os("QSD_NO_BLE_SCAN").is_some() {
                info!("BleListener: disabled (QSD_NO_BLE_SCAN)");
            } else if let Ok(ble) = BleListener::new(self.ble_sender.clone())
                .await
                .inspect_err(|err| warn!("BleListener: {}", err))
            {
                let ctk = ctoken.clone();
                tracker.spawn(async move { ble.run(ctk).await });
            }
        }

        // Start MDnsServer in own "task", restarted (with a new mDNS daemon) whenever it stops:
        // the daemon thread panics on some malformed packets from other hosts (see hdl/mdns.rs).
        let mdns_endpoint_id: [u8; 4] = endpoint_id[..4].try_into()?;
        let mdns_port = binded_addr.port();
        let mdns_ble_sender = self.ble_sender.clone();
        let mdns_visibility_sender = self.visibility_sender.clone();
        let mdns_visibility_receiver = self.visibility_receiver.clone();
        let ctk = ctoken.clone();
        tracker.spawn(supervise(
            "MDnsServer",
            ctk.clone(),
            RESTART_BACKOFF_MIN,
            RESTART_BACKOFF_MAX,
            move || {
                let mdns = MDnsServer::new(
                    mdns_endpoint_id,
                    mdns_port,
                    mdns_ble_sender.subscribe(),
                    mdns_visibility_sender.clone(),
                    mdns_visibility_receiver.clone(),
                );
                let ctk = ctk.clone();
                async move { mdns?.run(ctk).await }
            },
        ));

        // Advertise as a discoverable QuickShare receiver over BLE (0xFEF3) so a
        // phone that drops off Wi-Fi during its browse phase (the Pixel "AirDrop
        // update" behaviour) can still list us as a target. Uses the SAME
        // endpoint_id as mDNS, so once selected the phone reconnects to Wi-Fi and
        // completes the transfer over the normal Wi-Fi-LAN (mDNS + TCP) path.
        #[cfg(all(feature = "experimental", target_os = "linux"))]
        {
            if *self.visibility_receiver.borrow() != Visibility::Invisible {
                let rx_endpoint_id: [u8; 4] = endpoint_id[..4].try_into()?;
                let rx_device_name = DEVICE_NAME.read().unwrap().clone();
                // With our Bluetooth address in the advertisement, the phone's first connection
                // attempt is Bluetooth Classic (RFCOMM) to it; with an all-zero address it skips
                // Classic and goes straight to BLE. Classic doesn't connect reliably yet (see
                // docs/DEV_NOTES.md) and its retries delay BLE by ~5 s, during which a Pixel may
                // rejoin Wi-Fi and cancel the attempt, so it is opt-in: QSD_BT_CLASSIC=1.
                let bt_classic = std::env::var_os("QSD_BT_CLASSIC").is_some();
                let bt_mac = if !bt_classic {
                    [0u8; 6]
                } else {
                    match bluer::Session::new().await {
                        Ok(s) => match s.default_adapter().await {
                            Ok(a) => a.address().await.map(|a| a.0).unwrap_or_default(),
                            Err(_) => [0u8; 6],
                        },
                        Err(_) => [0u8; 6],
                    }
                };
                // Same advertisement bytes are used for the BLE advert and served
                // over GATT slot 0, so the phone reads a consistent endpoint.
                let advert = crate::hdl::receiver_service_data(
                    rx_endpoint_id,
                    crate::utils::DeviceType::Laptop as u8,
                    &rx_device_name,
                    bt_mac,
                );

                // Bluetooth Classic (RFCOMM): the phone's preferred first connection.
                let bt_sender = self.message_sender.clone();
                let bt_tcp_port = binded_addr.port();
                let bctk = ctoken.clone();
                if bt_classic {
                    tracker.spawn(async move {
                        match crate::hdl::RfcommServer::new(bt_sender, bt_tcp_port).await {
                            Ok(srv) => {
                                if let Err(e) = srv.run(bctk).await {
                                    error!("RfcommServer: {}", e);
                                }
                            }
                            Err(e) => error!("Couldn't init RfcommServer: {}", e),
                        }
                    });
                }

                // GATT server: when the phone selects us it opens a GATT connection,
                // reads slot 0 for our advertisement, then drives the weave data
                // socket which we bridge to the inbound handshake.
                let gatt_advert = advert.clone();
                let gatt_sender = self.message_sender.clone();
                let gatt_tcp_port = binded_addr.port();
                // Both BLE tasks are restarted when they stop or can't start (Bluetooth off at
                // startup, adapter gone after a resume or a bluetoothd restart).
                let gctk = ctoken.clone();
                tracker.spawn(supervise(
                    "ReceiverGattServer",
                    gctk.clone(),
                    RESTART_BACKOFF_MIN,
                    RESTART_BACKOFF_MAX,
                    move || {
                        let (gatt_advert, gatt_sender) = (gatt_advert.clone(), gatt_sender.clone());
                        let gctk = gctk.clone();
                        async move {
                            crate::hdl::ReceiverGattServer::new(
                                gatt_advert,
                                gatt_sender,
                                gatt_tcp_port,
                            )
                            .await
                            .map_err(|e| anyhow!("couldn't init: {e}"))?
                            .run(gctk)
                            .await
                        }
                    },
                ));

                // BLE receiver advertisement (0xFEF3).
                let ctk = ctoken.clone();
                tracker.spawn(supervise(
                    "ReceiverAdvertiser",
                    ctk.clone(),
                    RESTART_BACKOFF_MIN,
                    RESTART_BACKOFF_MAX,
                    move || {
                        let rx_device_name = rx_device_name.clone();
                        let ctk = ctk.clone();
                        async move {
                            crate::hdl::ReceiverAdvertiser::new(
                                rx_endpoint_id,
                                crate::utils::DeviceType::Laptop as u8,
                                &rx_device_name,
                            )
                            .await
                            .map_err(|e| anyhow!("couldn't init: {e}"))?
                            .run(ctk)
                            .await
                        }
                    },
                ));
            }
        }

        tracker.close();

        Ok((send_channel.0, self.ble_sender.subscribe()))
    }

    pub fn discovery(
        &mut self,
        sender: broadcast::Sender<EndpointInfo>,
    ) -> Result<(), anyhow::Error> {
        let tracker = self
            .tracker
            .as_ref()
            .ok_or_else(|| anyhow!("The service wasn't first started"))?;

        let ctk = CancellationToken::new();
        self.discovery_ctk = Some(ctk.clone());

        // QSD_NO_FASTINIT=1: skip the 0xFE2C "I want to send" advertisement (diagnostics: it makes
        // some phones restart their Quick Share advertising, and their address, every few seconds).
        #[cfg(all(feature = "experimental", target_os = "linux"))]
        {
            if std::env::var_os("QSD_NO_FASTINIT").is_some() {
                info!("BleAdvertiser: disabled (QSD_NO_FASTINIT)");
            } else {
                let ctk_blea = ctk.clone();
                tracker.spawn(async move {
                    let blea = match BleAdvertiser::new().await {
                        Ok(b) => b,
                        Err(e) => {
                            error!("Couldn't init BleAdvertiser: {}", e);
                            return;
                        }
                    };

                    if let Err(e) = blea.run(ctk_blea).await {
                        error!("Couldn't start BleAdvertiser: {}", e);
                    }
                });
            }
        }

        let discovery = MDnsDiscovery::new(sender)?;
        tracker.spawn(async move { discovery.run(ctk.clone()).await });

        Ok(())
    }

    pub fn stop_discovery(&mut self) {
        if let Some(discovert_ctk) = &self.discovery_ctk {
            discovert_ctk.cancel();
            self.discovery_ctk = None;
        }
    }

    pub fn change_visibility(&mut self, nv: Visibility) {
        self.visibility_sender
            .lock()
            .unwrap()
            .send_modify(|state| *state = nv);
    }

    pub async fn stop(&mut self) {
        self.stop_discovery();

        if let Some(ctoken) = &self.ctoken {
            ctoken.cancel();
        }

        if let Some(tracker) = &self.tracker {
            // Inorder for TaskTracker::wait to return, close() must be called
            // and the count of tasks being watched should be 0 (i.e. they've all closed).
            //
            // If not, the TaskTracker may forever wait if task count is 0 when wait() was called
            tracker.close();
            tracker.wait().await;
        }

        self.ctoken = None;
        self.tracker = None;
    }

    // Setting None here will resume the default settings
    pub fn set_download_path(&self, p: Option<PathBuf>) {
        debug!("Setting the download path to {:?}", p);
        let mut guard = CUSTOM_DOWNLOAD.write().unwrap();
        *guard = p;
    }

    /// For this to properly take effect,
    /// `MdnsServer` would need to be reset which is done by `RQS::stop` followed by `RQS::run`.
    ///
    /// So only do this when no data transfer is going on.
    pub fn set_device_name(&self, name: String) {
        debug!("Setting the device name {:?}", name);
        let mut guard = DEVICE_NAME.write().unwrap();
        *guard = name;
    }

    pub fn get_device_name(&self) -> String {
        DEVICE_NAME.read().unwrap().clone()
    }
}

const RESTART_BACKOFF_MIN: std::time::Duration = std::time::Duration::from_secs(1);
const RESTART_BACKOFF_MAX: std::time::Duration = std::time::Duration::from_secs(30);

/// Delay before the next restart of a supervised task: doubles up to `max`, and starts again
/// from `min` when the task had been running for at least `max` (a fresh failure, not a loop).
fn next_backoff(
    delay: std::time::Duration,
    ran_for: std::time::Duration,
    min: std::time::Duration,
    max: std::time::Duration,
) -> std::time::Duration {
    if ran_for >= max {
        min
    } else {
        (delay * 2).min(max)
    }
}

/// Runs the task built by `make` until `ctk` is cancelled, restarting it with a backoff
/// (`min` doubling up to `max`) whenever it returns, with an error or not, before that. The
/// announcement tasks (mDNS, BLE) run under this: a dead one leaves the laptop invisible until
/// the app restarts, and before they ended silently (the spawned task's result was dropped).
async fn supervise<F, Fut>(
    name: &'static str,
    ctk: CancellationToken,
    min: std::time::Duration,
    max: std::time::Duration,
    mut make: F,
) where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<(), anyhow::Error>>,
{
    let mut delay = min;
    loop {
        // Not raced against `ctk`: the task handles cancellation itself (mDNS sends its goodbye).
        let started = tokio::time::Instant::now();
        let res = make().await;
        if ctk.is_cancelled() {
            return;
        }

        delay = next_backoff(delay, started.elapsed(), min, max);
        match res {
            Ok(()) => warn!("{name}: stopped, restarting in {delay:?}"),
            Err(e) => warn!("{name}: {e:#}; restarting in {delay:?}"),
        }
        tokio::select! {
            _ = ctk.cancelled() => return,
            _ = tokio::time::sleep(delay) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use super::*;

    const MS: Duration = Duration::from_millis(1);

    #[test]
    fn backoff_doubles_up_to_max_and_resets_after_a_long_run() {
        let (min, max) = (Duration::from_secs(1), Duration::from_secs(30));
        let mut delay = min;
        let mut seen = vec![];
        for _ in 0..7 {
            delay = next_backoff(delay, Duration::ZERO, min, max);
            seen.push(delay.as_secs());
        }
        assert_eq!(seen, [2, 4, 8, 16, 30, 30, 30]);
        assert_eq!(next_backoff(max, Duration::from_secs(3600), min, max), min);
    }

    #[tokio::test]
    async fn supervise_restarts_a_failing_task_until_it_keeps_running() {
        let ctk = CancellationToken::new();
        let runs = Arc::new(AtomicUsize::new(0));
        let (r, c) = (runs.clone(), ctk.clone());
        let sup = tokio::spawn(supervise("test", ctk.clone(), MS, 4 * MS, move || {
            let (r, c) = (r.clone(), c.clone());
            async move {
                // Fails three times (error, early return, error), then runs until cancelled.
                match r.fetch_add(1, Ordering::SeqCst) {
                    0 | 2 => Err(anyhow!("boom")),
                    1 => Ok(()),
                    _ => {
                        c.cancelled().await;
                        Ok(())
                    }
                }
            }
        }));

        tokio::time::timeout(Duration::from_secs(5), async {
            while runs.load(Ordering::SeqCst) < 4 {
                tokio::time::sleep(MS).await;
            }
        })
        .await
        .expect("the task was not restarted");
        ctk.cancel();
        tokio::time::timeout(Duration::from_secs(5), sup)
            .await
            .expect("supervise did not stop on cancel")
            .unwrap();
        assert_eq!(runs.load(Ordering::SeqCst), 4);
    }

    #[tokio::test]
    async fn supervise_stops_during_the_backoff_when_cancelled() {
        let ctk = CancellationToken::new();
        let runs = Arc::new(AtomicUsize::new(0));
        let r = runs.clone();
        let sup = tokio::spawn(supervise(
            "test",
            ctk.clone(),
            Duration::from_secs(3600),
            Duration::from_secs(3600),
            move || {
                r.fetch_add(1, Ordering::SeqCst);
                async { Err(anyhow!("boom")) }
            },
        ));

        while runs.load(Ordering::SeqCst) < 1 {
            tokio::time::sleep(MS).await;
        }
        ctk.cancel();
        tokio::time::timeout(Duration::from_secs(5), sup)
            .await
            .expect("supervise kept sleeping after cancel")
            .unwrap();
        assert_eq!(runs.load(Ordering::SeqCst), 1);
    }
}
