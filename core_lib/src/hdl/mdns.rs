use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::anyhow;
use mdns_sd::{AddrType, ServiceDaemon, ServiceInfo};
use tokio::sync::broadcast::Receiver;
use tokio::sync::watch;
use tokio::time::{Instant, interval_at};
use tokio_util::sync::CancellationToken;

use crate::DEVICE_NAME;
use crate::utils::{DeviceType, gen_mdns_endpoint_info, gen_mdns_name};

const INNER_NAME: &str = "MDnsServer";
const TICK_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visibility {
    Visible = 0,
    Invisible = 1,
    Temporarily = 2,
}

#[allow(dead_code)]
impl Visibility {
    pub fn from_raw_value(value: u64) -> Self {
        match value {
            0 => Visibility::Visible,
            1 => Visibility::Invisible,
            2 => Visibility::Temporarily,
            _ => unreachable!(),
        }
    }
}

pub struct MDnsServer {
    daemon: ServiceDaemon,
    service_info: ServiceInfo,
    ble_receiver: Receiver<()>,
    visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    visibility_receiver: watch::Receiver<Visibility>,
}

impl MDnsServer {
    pub fn new(
        endpoint_id: [u8; 4],
        service_port: u16,
        ble_receiver: Receiver<()>,
        visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
        visibility_receiver: watch::Receiver<Visibility>,
    ) -> Result<Self, anyhow::Error> {
        let service_info = Self::build_service(endpoint_id, service_port, DeviceType::Laptop)?;

        Ok(Self {
            daemon: ServiceDaemon::new()?,
            service_info,
            ble_receiver,
            visibility_sender,
            visibility_receiver,
        })
    }

    /// Runs until `ctk` is cancelled. Returns an error when the mDNS daemon is gone (its thread
    /// panics on some malformed packets, see `dns_parser.rs` in mdns-sd: unchecked slicing on
    /// RDLENGTH): the caller then starts over with a new `MDnsServer`.
    pub async fn run(&mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");
        let monitor = self.daemon.monitor()?;
        // Mark the current value as seen and apply it here, so a restarted server announces too.
        let mut visibility = *self.visibility_receiver.borrow_and_update();
        if visibility != Visibility::Invisible {
            self.announce();
        }
        let mut interval = interval_at(Instant::now() + TICK_INTERVAL, TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                r = monitor.recv_async() => {
                    match r {
                        Ok(_) => continue,
                        Err(_) => return Err(anyhow!("mDNS daemon stopped")),
                    }
                },
                _ = self.visibility_receiver.changed() => {
                    visibility = *self.visibility_receiver.borrow_and_update();

                    debug!("{INNER_NAME}: visibility changed: {visibility:?}");
                    if visibility == Visibility::Visible {
                        self.announce();
                    } else if visibility == Visibility::Invisible {
                        self.withdraw().await;
                    } else if visibility == Visibility::Temporarily {
                        self.announce();
                        interval.reset();
                    }
                }
                _ = self.ble_receiver.recv() => {
                    if visibility == Visibility::Invisible {
                        continue;
                    }

                    debug!("{INNER_NAME}: ble_receiver: got event");
                    if visibility == Visibility::Visible || visibility == Visibility::Temporarily {
                        // Android can sometime not see the mDNS service if the service
                        // was running BEFORE Android started the Discovery phase for QuickShare.
                        // So resend a broadcast if there's a android device sending.
                        let fullname = self.service_info.get_fullname();
                        if let Err(e) = self.daemon.register_resend(fullname) {
                            warn!("{INNER_NAME}: register_resend failed: {e}");
                        }
                    } else {
                        self.announce();
                    }
                },
                _ = interval.tick() => {
                    if visibility != Visibility::Temporarily {
                        continue;
                    }

                    self.withdraw().await;
                    let _ = self.visibility_sender.lock().unwrap().send(Visibility::Invisible);
                }
            }
        }

        // Unregister the mDNS service - we're shutting down
        self.withdraw().await;

        Ok(())
    }

    /// (Re-)announces the service: `register` reads the current interface addresses again and
    /// sends the announcement on every interface. Errors are logged: the next event retries.
    fn announce(&self) {
        if let Err(e) = self.daemon.register(self.service_info.clone()) {
            warn!("{INNER_NAME}: register failed: {e}");
        }
    }

    async fn withdraw(&self) {
        match self.daemon.unregister(self.service_info.get_fullname()) {
            Ok(receiver) => {
                if let Ok(event) = receiver.recv_async().await {
                    info!("{INNER_NAME}: service unregistered: {:?}", &event);
                }
            }
            Err(e) => warn!("{INNER_NAME}: unregister failed: {e}"),
        }
    }

    fn build_service(
        endpoint_id: [u8; 4],
        service_port: u16,
        device_type: DeviceType,
    ) -> Result<ServiceInfo, anyhow::Error> {
        // This `name` is going to be random every time RQS service restarts.
        // If that is not desired, derive host_name, etc. via some other means
        let name = gen_mdns_name(endpoint_id);
        let device_name = DEVICE_NAME.read().unwrap().clone();
        info!("Broadcasting with: device_name={device_name}, host_name={name}");
        let endpoint_info = gen_mdns_endpoint_info(device_type as u8, &device_name);

        let properties = [("n", endpoint_info)];
        let si = ServiceInfo::new(
            "_FC9F5ED42C8A._tcp.local.",
            &name,
            &name, // Needs to be ASCII?
            "",
            service_port,
            &properties[..],
        )?
        .enable_addr_auto(AddrType::V4);

        Ok(si)
    }
}

impl Drop for MDnsServer {
    // Without this the daemon thread outlives the server (after a restart, or `RQS::stop`).
    fn drop(&mut self) {
        let _ = self.daemon.shutdown();
    }
}
