// Which link a transfer runs on, and whether it is slower than what this machine could offer
// (a fallback). Reported to clients in `MessageClient::transport` whenever the link changes.

use crate::location_nearby_connections::bandwidth_upgrade_negotiation_frame::upgrade_path_info::Medium;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportMedium {
    /// BLE GATT/weave socket (or Bluetooth Classic RFCOMM).
    Ble,
    /// BLE L2CAP channel (sending).
    BleL2cap,
    WifiLan,
    WifiHotspot,
    WifiDirect,
}

impl TransportMedium {
    /// Higher is faster. Measured (docs/DEV_NOTES.md): BLE weave ~5 KB/s, BLE L2CAP 38-46 KB/s,
    /// Wi-Fi LAN through the router 2.4 MB/s, our hotspot ~4-5 MB/s, Wi-Fi Direct VHT80
    /// 33-37 MB/s.
    pub fn rank(self) -> u8 {
        match self {
            TransportMedium::Ble => 0,
            TransportMedium::BleL2cap => 1,
            TransportMedium::WifiLan => 2,
            TransportMedium::WifiHotspot => 3,
            TransportMedium::WifiDirect => 4,
        }
    }

    pub fn is_bluetooth(self) -> bool {
        matches!(self, TransportMedium::Ble | TransportMedium::BleL2cap)
    }

    /// The medium of a bandwidth upgrade path (only the ones we can use).
    pub fn from_upgrade(m: Medium) -> Option<Self> {
        match m {
            Medium::WifiLan => Some(TransportMedium::WifiLan),
            Medium::WifiHotspot => Some(TransportMedium::WifiHotspot),
            Medium::WifiDirect => Some(TransportMedium::WifiDirect),
            _ => None,
        }
    }

    /// The upgrade path medium for a Wi-Fi link.
    pub fn to_upgrade(self) -> Option<Medium> {
        match self {
            TransportMedium::WifiLan => Some(Medium::WifiLan),
            TransportMedium::WifiHotspot => Some(Medium::WifiHotspot),
            TransportMedium::WifiDirect => Some(Medium::WifiDirect),
            TransportMedium::Ble | TransportMedium::BleL2cap => None,
        }
    }

    /// The fastest of a list of upgrade mediums (raw proto values), if any is usable.
    pub fn best_of(mediums: &[i32]) -> Option<Self> {
        mediums
            .iter()
            .filter_map(|m| Medium::try_from(*m).ok())
            .filter_map(Self::from_upgrade)
            .max_by_key(|m| m.rank())
    }
}

/// Why the transfer is not (or not only) on the expected medium.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportReason {
    /// The root helper (receiving) or join helper (sending) isn't installed.
    HelperMissing,
    /// The Wi-Fi link couldn't be brought up on this side.
    LinkStartFailed(String),
    /// The phone couldn't join the link we offered.
    PeerJoinFailed,
    /// The phone didn't connect over the new link in time.
    Timeout,
    /// The phone made no upgrade offer (sending).
    NoOffer,
    /// The phone doesn't support the link we offered, or offered one we can't use.
    PeerUnsupported,
}

impl TransportReason {
    /// Stable identifier for clients.
    pub fn code(&self) -> &'static str {
        match self {
            TransportReason::HelperMissing => "helper_missing",
            TransportReason::LinkStartFailed(_) => "link_start_failed",
            TransportReason::PeerJoinFailed => "peer_join_failed",
            TransportReason::Timeout => "timeout",
            TransportReason::NoOffer => "no_offer",
            TransportReason::PeerUnsupported => "peer_unsupported",
        }
    }
}

impl std::fmt::Display for TransportReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportReason::HelperMissing => write!(f, "the Wi-Fi helper isn't installed"),
            TransportReason::LinkStartFailed(e) => write!(f, "couldn't start the Wi-Fi link ({e})"),
            TransportReason::PeerJoinFailed => write!(f, "the phone couldn't join the Wi-Fi link"),
            TransportReason::Timeout => write!(f, "the phone didn't connect in time"),
            TransportReason::NoOffer => write!(f, "the phone offered no Wi-Fi link"),
            TransportReason::PeerUnsupported => write!(f, "no Wi-Fi link supported by both devices"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportInfo {
    /// The link the transfer runs on now.
    pub medium: TransportMedium,
    /// The best link this machine could offer for this transfer.
    pub expected: TransportMedium,
    /// `medium` is slower than `expected`.
    pub is_fallback: bool,
    pub reason: Option<TransportReason>,
}

impl TransportInfo {
    pub fn new(
        medium: TransportMedium,
        expected: TransportMedium,
        reason: Option<TransportReason>,
    ) -> Self {
        Self {
            medium,
            expected,
            is_fallback: medium.rank() < expected.rank(),
            reason,
        }
    }

    /// The link a session starts on, before any upgrade (never a fallback).
    pub fn initial(medium: TransportMedium) -> Self {
        Self::new(medium, medium, None)
    }

    pub fn log(&self) {
        match &self.reason {
            Some(r) if self.is_fallback => {
                warn!("transport: {:?} instead of {:?}: {r}", self.medium, self.expected)
            }
            Some(r) => info!("transport: {:?} ({r})", self.medium),
            None => info!("transport: {:?}", self.medium),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TransportMedium::*;

    #[test]
    fn transport_rank_follows_measured_speed() {
        let order = [Ble, BleL2cap, WifiLan, WifiHotspot, WifiDirect];
        for w in order.windows(2) {
            assert!(w[0].rank() < w[1].rank(), "{:?} should rank below {:?}", w[0], w[1]);
        }
    }

    #[test]
    fn transport_fallback_only_below_expected() {
        assert!(TransportInfo::new(Ble, WifiDirect, Some(TransportReason::Timeout)).is_fallback);
        assert!(TransportInfo::new(WifiHotspot, WifiDirect, None).is_fallback);
        assert!(TransportInfo::new(Ble, WifiLan, Some(TransportReason::NoOffer)).is_fallback);
        assert!(!TransportInfo::new(WifiDirect, WifiDirect, None).is_fallback);
        // Without the helper the shared Wi-Fi is the best we can do: not a fallback.
        let lan = TransportInfo::new(WifiLan, WifiLan, Some(TransportReason::HelperMissing));
        assert!(!lan.is_fallback);
        assert!(!TransportInfo::initial(Ble).is_fallback);
        assert!(!TransportInfo::initial(BleL2cap).is_fallback);
    }

    #[test]
    fn transport_best_of_upgrade_mediums() {
        let all = [Medium::WifiLan as i32, Medium::WifiDirect as i32, Medium::WifiHotspot as i32];
        assert_eq!(TransportMedium::best_of(&all), Some(WifiDirect));
        assert_eq!(TransportMedium::best_of(&[Medium::WifiLan as i32]), Some(WifiLan));
        assert_eq!(TransportMedium::best_of(&[Medium::Bluetooth as i32, 999]), None);
        assert_eq!(TransportMedium::best_of(&[]), None);
    }

    #[test]
    fn transport_reason_codes() {
        assert_eq!(TransportReason::LinkStartFailed("x".into()).code(), "link_start_failed");
        assert_eq!(TransportReason::HelperMissing.code(), "helper_missing");
        assert_eq!(
            TransportReason::LinkStartFailed("no channel".into()).to_string(),
            "couldn't start the Wi-Fi link (no channel)"
        );
    }
}
