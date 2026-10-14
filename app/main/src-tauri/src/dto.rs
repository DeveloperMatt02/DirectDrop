//! DTO layer: unico wire format GUI <-> core.
//!
//! `core_lib` non deriva `Serialize/Deserialize` su nessun tipo di dominio,
//! quindi nessun tipo core attraversa mai il bridge Tauri tal quale.
//! Tutto il JSON `invoke` / `emit` passa da questi DTO (serde) con mapping
//! `From` esplicito. Fonte speculare: `app/main/src/types/qsd.ts`.
//!
//! Regola di confine (`GUI_ARCHITECTURE.md` §1.1): da `rqs_lib` si importa solo
//! quanto serve al bridge; tutto il resto del core resta off-limits alla GUI.

use std::time::Duration;

/// Versione del wire format. Duplicata in `app/main/src/types/qsd.ts`;
/// il test `dto_version_matches_ts` fallisce se divergono.
pub const DTO_VERSION: u32 = 1;

/// Throttle progress `qsd://transfer` (§2.4): emit solo se cambio di `state`,
/// oppure delta `ack_bytes` >= 256 KiB, oppure >= 500 ms dall'ultimo emit
/// per quell'`id`. Lo stato terminale e' sempre emesso.
pub const PROGRESS_BYTES_STEP: u64 = 256 * 1024;
pub const PROGRESS_MIN_INTERVAL: Duration = Duration::from_millis(500);

/// Eventi BE -> FE (rinominano i vecchi `rs2js_*` / `visibility_updated`).
pub const EVT_TRANSFER: &str = "qsd://transfer";
pub const EVT_ENDPOINTS: &str = "qsd://endpoints";
pub const EVT_VISIBILITY: &str = "qsd://visibility";
pub const EVT_BLE_PRESENCE: &str = "qsd://ble-presence";

/// Timeout di consenso di default (parita' CLI `QSD_CONSENT_TIMEOUT`, 60 s).
pub const CONSENT_TIMEOUT_DEFAULT_SECS: u64 = 60;

/// Throttle dell'evento `qsd://ble-presence` (stessa finestra di `main.rs`: 120 s).
pub const BLE_PRESENCE_THROTTLE: Duration = Duration::from_secs(120);

/// Legge `QSD_CONSENT_TIMEOUT` (default 60 s). La GUI non implementa mai
/// il timer da se': il bridge lo usa per l'auto-decline.
pub fn consent_timeout_secs() -> u64 {
    std::env::var("QSD_CONSENT_TIMEOUT")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(CONSENT_TIMEOUT_DEFAULT_SECS)
}

// ---------------------------------------------------------------------------
// Enum DTO (serializzate come stringhe)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VisibilityDto {
    Visible,
    Invisible,
    Temporarily,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransferKindDto {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransferActionDto {
    ConsentAccept,
    ConsentDecline,
    TransferCancel,
}

/// 18 varianti di `rqs_lib::TransferState` (`hdl/mod.rs`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransferStateDto {
    Initial,
    ReceivedConnectionRequest,
    SentUkeyServerInit,
    SentUkeyClientInit,
    SentUkeyClientFinish,
    SentPairedKeyEncryption,
    ReceivedUkeyClientFinish,
    SentConnectionResponse,
    SentPairedKeyResult,
    SentIntroduction,
    ReceivedPairedKeyResult,
    WaitingForUserConsent,
    ReceivingFiles,
    SendingFiles,
    Disconnected,
    Rejected,
    Cancelled,
    Finished,
}

impl TransferStateDto {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            TransferStateDto::Finished
                | TransferStateDto::Rejected
                | TransferStateDto::Cancelled
                | TransferStateDto::Disconnected
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PayloadKindDto {
    Files,
    Text,
    Url,
    WiFi,
}

// ---------------------------------------------------------------------------
// Struct DTO
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RemoteDeviceDto {
    pub name: String,
    pub device_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TransferMetadataDto {
    pub id: String,
    pub source: Option<RemoteDeviceDto>,
    pub pin_code: Option<String>,
    pub payload_kind: PayloadKindDto,
    pub payload_preview: Option<String>,
    pub files: Option<Vec<String>>,
    pub text: Option<String>,
    pub url: Option<String>,
    pub wifi_ssid: Option<String>,
    pub total_bytes: u64,
    pub ack_bytes: u64,
}

/// Evento `qsd://transfer` (da `Message::Client`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TransferDto {
    pub id: String,
    pub kind: TransferKindDto,
    pub state: TransferStateDto,
    pub metadata: Option<TransferMetadataDto>,
}

/// Evento `qsd://endpoints` (da `EndpointInfo`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EndpointDto {
    pub id: String,
    pub fullname: String,
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub device_type: Option<String>,
    pub present: bool,
}

/// Input `qsd_send_lan` (-> `SendInfo{ob: Files}`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SendPayloadDto {
    pub id: String,
    pub name: String,
    pub addr: String,
    pub files: Vec<String>,
}

/// Input `qsd_consent` (-> `ChannelMessage{id, msg: Lib{action}}`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ConsentDto {
    pub id: String,
    pub action: TransferActionDto,
}

/// Input `qsd_scan_ble` (one-shot BLE scan, §1.1).
/// `timeout_s` range 3–30, default 30; `settle` fisso 3 s.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ScanBleInput {
    pub name: Option<String>,
    pub timeout_s: Option<u64>,
}

/// Input `qsd_send_ble` (token opaco da `qsd_scan_ble`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SendBleInput {
    pub receiver_id: String,
    pub files: Vec<String>,
    pub send_id: Option<String>,
}

/// Fase 2 (BLE): mantenuto qui perche' parte del wire versionato.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BleReceiverDto {
    pub address: String,
    pub name: Option<String>,
    pub endpoint_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotDto {
    pub dto_version: u32,
    pub visibility: VisibilityDto,
    pub device_name: String,
    pub download_path: Option<String>,
    pub port: Option<u32>,
    pub discovery_running: bool,
}

/// Payload `qsd://visibility`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct VisibilityEvent {
    pub visibility: VisibilityDto,
}

/// Payload `qsd://ble-presence` (`at` = unix millis).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BlePresenceEvent {
    pub at: u64,
}

/// Errore di comando (`code`: CORE|VALIDATION|NOT_RUNNING|IO|TIMEOUT).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GuiError {
    pub code: String,
    pub message: String,
}

impl GuiError {
    fn new(code: &str, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into() }
    }
    pub fn core(message: impl Into<String>) -> Self {
        Self::new("CORE", message)
    }
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new("VALIDATION", message)
    }
    pub fn not_running(message: impl Into<String>) -> Self {
        Self::new("NOT_RUNNING", message)
    }
    pub fn io(message: impl Into<String>) -> Self {
        Self::new("IO", message)
    }
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::new("TIMEOUT", message)
    }
}

impl std::fmt::Display for GuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for GuiError {}

impl From<anyhow::Error> for GuiError {
    fn from(e: anyhow::Error) -> Self {
        Self::core(e.to_string())
    }
}

// ---------------------------------------------------------------------------
// Mapping core -> DTO (mai nel core)
// ---------------------------------------------------------------------------

use rqs_lib::channel::{ChannelMessage, Message, MessageClient, TransferAction, TransferKind};
use rqs_lib::hdl::info::{TransferMetadata, TransferPayload, TransferPayloadKind};
use rqs_lib::{DeviceType, EndpointInfo, OutboundPayload, SendInfo, TransferState, Visibility};
use rqs_lib::utils::RemoteDeviceInfo;

pub fn device_type_to_string(t: &DeviceType) -> String {
    match t {
        DeviceType::Unknown => "Unknown",
        DeviceType::Phone => "Phone",
        DeviceType::Tablet => "Tablet",
        DeviceType::Laptop => "Laptop",
    }
    .to_string()
}

impl From<Visibility> for VisibilityDto {
    fn from(v: Visibility) -> Self {
        match v {
            Visibility::Visible => VisibilityDto::Visible,
            Visibility::Invisible => VisibilityDto::Invisible,
            Visibility::Temporarily => VisibilityDto::Temporarily,
        }
    }
}

impl From<VisibilityDto> for Visibility {
    fn from(v: VisibilityDto) -> Self {
        match v {
            VisibilityDto::Visible => Visibility::Visible,
            VisibilityDto::Invisible => Visibility::Invisible,
            VisibilityDto::Temporarily => Visibility::Temporarily,
        }
    }
}

impl From<TransferKind> for TransferKindDto {
    fn from(k: TransferKind) -> Self {
        match k {
            TransferKind::Inbound => TransferKindDto::Inbound,
            TransferKind::Outbound => TransferKindDto::Outbound,
        }
    }
}

impl From<TransferAction> for TransferActionDto {
    fn from(a: TransferAction) -> Self {
        match a {
            TransferAction::ConsentAccept => TransferActionDto::ConsentAccept,
            TransferAction::ConsentDecline => TransferActionDto::ConsentDecline,
            TransferAction::TransferCancel => TransferActionDto::TransferCancel,
        }
    }
}

impl From<TransferActionDto> for TransferAction {
    fn from(a: TransferActionDto) -> Self {
        match a {
            TransferActionDto::ConsentAccept => TransferAction::ConsentAccept,
            TransferActionDto::ConsentDecline => TransferAction::ConsentDecline,
            TransferActionDto::TransferCancel => TransferAction::TransferCancel,
        }
    }
}

impl From<TransferState> for TransferStateDto {
    fn from(s: TransferState) -> Self {
        match s {
            TransferState::Initial => TransferStateDto::Initial,
            TransferState::ReceivedConnectionRequest => {
                TransferStateDto::ReceivedConnectionRequest
            }
            TransferState::SentUkeyServerInit => TransferStateDto::SentUkeyServerInit,
            TransferState::SentUkeyClientInit => TransferStateDto::SentUkeyClientInit,
            TransferState::SentUkeyClientFinish => TransferStateDto::SentUkeyClientFinish,
            TransferState::SentPairedKeyEncryption => {
                TransferStateDto::SentPairedKeyEncryption
            }
            TransferState::ReceivedUkeyClientFinish => {
                TransferStateDto::ReceivedUkeyClientFinish
            }
            TransferState::SentConnectionResponse => {
                TransferStateDto::SentConnectionResponse
            }
            TransferState::SentPairedKeyResult => TransferStateDto::SentPairedKeyResult,
            TransferState::SentIntroduction => TransferStateDto::SentIntroduction,
            TransferState::ReceivedPairedKeyResult => {
                TransferStateDto::ReceivedPairedKeyResult
            }
            TransferState::WaitingForUserConsent => TransferStateDto::WaitingForUserConsent,
            TransferState::ReceivingFiles => TransferStateDto::ReceivingFiles,
            TransferState::SendingFiles => TransferStateDto::SendingFiles,
            TransferState::Disconnected => TransferStateDto::Disconnected,
            TransferState::Rejected => TransferStateDto::Rejected,
            TransferState::Cancelled => TransferStateDto::Cancelled,
            TransferState::Finished => TransferStateDto::Finished,
        }
    }
}

impl From<TransferPayloadKind> for PayloadKindDto {
    fn from(k: TransferPayloadKind) -> Self {
        match k {
            TransferPayloadKind::Files => PayloadKindDto::Files,
            TransferPayloadKind::Text => PayloadKindDto::Text,
            TransferPayloadKind::Url => PayloadKindDto::Url,
            TransferPayloadKind::WiFi => PayloadKindDto::WiFi,
        }
    }
}

impl From<&RemoteDeviceInfo> for RemoteDeviceDto {
    fn from(r: &RemoteDeviceInfo) -> Self {
        Self { name: r.name.clone(), device_type: device_type_to_string(&r.device_type) }
    }
}

impl From<&TransferMetadata> for TransferMetadataDto {
    fn from(m: &TransferMetadata) -> Self {
        let (files, text, url, wifi_ssid) = match &m.payload {
            Some(TransferPayload::Files(f)) => (Some(f.clone()), None, None, None),
            Some(TransferPayload::Text(t)) => (None, Some(t.clone()), None, None),
            Some(TransferPayload::Url(u)) => (None, None, Some(u.clone()), None),
            Some(TransferPayload::Wifi { ssid, .. }) => {
                (None, None, None, Some(ssid.clone()))
            }
            None => (None, None, None, None),
        };
        Self {
            id: m.id.clone(),
            source: m.source.as_ref().map(RemoteDeviceDto::from),
            pin_code: m.pin_code.clone(),
            payload_kind: m.payload_kind.clone().into(),
            payload_preview: m.payload_preview.clone(),
            files,
            text,
            url,
            wifi_ssid,
            total_bytes: m.total_bytes,
            ack_bytes: m.ack_bytes,
        }
    }
}

impl TransferDto {
    /// `None` per `Message::Lib` (il core emette solo `Client`; `Lib` si
    /// costruisce solo in ingresso da `ConsentDto`).
    pub fn from_channel(cm: &ChannelMessage) -> Option<Self> {
        let mc: &MessageClient = match &cm.msg {
            Message::Client(mc) => mc,
            Message::Lib { .. } => return None,
        };
        Some(Self {
            id: cm.id.clone(),
            kind: mc.kind.clone().into(),
            state: mc.state.clone().map(TransferStateDto::from).unwrap_or(TransferStateDto::Initial),
            metadata: mc.metadata.as_ref().map(TransferMetadataDto::from),
        })
    }
}

impl From<&EndpointInfo> for EndpointDto {
    fn from(e: &EndpointInfo) -> Self {
        Self {
            id: e.id.clone(),
            fullname: e.fullname.clone(),
            name: e.name.clone(),
            ip: e.ip.clone(),
            port: e.port.clone(),
            device_type: e.rtype.as_ref().map(device_type_to_string),
            // `None` (rimozione mDNS) -> `false` per semplificare Vue.
            present: e.present.unwrap_or(false),
        }
    }
}

impl ConsentDto {
    pub fn into_channel_message(self) -> ChannelMessage {
        ChannelMessage { id: self.id, msg: Message::Lib { action: self.action.into() } }
    }
}

impl SendPayloadDto {
    pub fn into_send_info(self) -> SendInfo {
        SendInfo {
            id: self.id,
            name: self.name,
            addr: self.addr,
            ob: OutboundPayload::Files(self.files),
        }
    }
}

// ---------------------------------------------------------------------------
// Test: mapping DTO <-> fixture JSON, sync DTO_VERSION con qsd.ts
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dto_version_matches_ts() {
        let ts = include_str!("../../src/types/qsd.ts");
        assert!(
            ts.contains(&format!("DTO_VERSION = {}", DTO_VERSION)),
            "dto.rs DTO_VERSION={} divergente da app/main/src/types/qsd.ts",
            DTO_VERSION
        );
    }

    #[test]
    fn transfer_state_all_variants_roundtrip() {
        let all = [
            (TransferState::Initial, "Initial"),
            (TransferState::ReceivedConnectionRequest, "ReceivedConnectionRequest"),
            (TransferState::SentUkeyServerInit, "SentUkeyServerInit"),
            (TransferState::SentUkeyClientInit, "SentUkeyClientInit"),
            (TransferState::SentUkeyClientFinish, "SentUkeyClientFinish"),
            (TransferState::SentPairedKeyEncryption, "SentPairedKeyEncryption"),
            (TransferState::ReceivedUkeyClientFinish, "ReceivedUkeyClientFinish"),
            (TransferState::SentConnectionResponse, "SentConnectionResponse"),
            (TransferState::SentPairedKeyResult, "SentPairedKeyResult"),
            (TransferState::SentIntroduction, "SentIntroduction"),
            (TransferState::ReceivedPairedKeyResult, "ReceivedPairedKeyResult"),
            (TransferState::WaitingForUserConsent, "WaitingForUserConsent"),
            (TransferState::ReceivingFiles, "ReceivingFiles"),
            (TransferState::SendingFiles, "SendingFiles"),
            (TransferState::Disconnected, "Disconnected"),
            (TransferState::Rejected, "Rejected"),
            (TransferState::Cancelled, "Cancelled"),
            (TransferState::Finished, "Finished"),
        ];
        assert_eq!(all.len(), 18);
        for (core, wire) in all {
            let dto = TransferStateDto::from(core);
            let json = serde_json::to_string(&dto).unwrap();
            assert_eq!(json, format!("\"{wire}\""));
            let back: TransferStateDto = serde_json::from_str(&json).unwrap();
            assert_eq!(back, dto);
        }
    }

    #[test]
    fn endpoint_removal_maps_present_false() {
        let info = EndpointInfo {
            id: "192.168.1.20:46257".into(),
            ..Default::default()
        };
        let dto = EndpointDto::from(&info);
        assert!(!dto.present);
        assert_eq!(dto.name, None);
        let json = serde_json::to_string(&dto).unwrap();
        let back: EndpointDto = serde_json::from_str(&json).unwrap();
        assert_eq!(back, dto);
    }

    #[test]
    fn scan_ble_send_ble_roundtrip() {
        let scan = ScanBleInput { name: Some("Pixel".into()), timeout_s: Some(30) };
        let json = serde_json::to_string(&scan).unwrap();
        let back: ScanBleInput = serde_json::from_str(&json).unwrap();
        assert_eq!(back, scan);
        // Default wire: campi assenti -> None.
        let empty: ScanBleInput = serde_json::from_str("{}").unwrap();
        assert_eq!(empty, ScanBleInput { name: None, timeout_s: None });

        let send = SendBleInput {
            receiver_id: "abcd-AA:BB:CC:DD:EE:FF".into(),
            files: vec!["/tmp/a.pdf".into()],
            send_id: Some("ble-123".into()),
        };
        let json = serde_json::to_string(&send).unwrap();
        let back: SendBleInput = serde_json::from_str(&json).unwrap();
        assert_eq!(back, send);
        let no_id: SendBleInput =
            serde_json::from_str(r#"{"receiver_id":"x","files":[]}"#).unwrap();
        assert_eq!(no_id.send_id, None);
    }

    #[test]
    fn transfer_consent_wire_example() {
        // Esempio wire §3.3: consenso in arrivo + accept.
        let dto = TransferDto {
            id: "192.168.1.5:51234".into(),
            kind: TransferKindDto::Inbound,
            state: TransferStateDto::WaitingForUserConsent,
            metadata: Some(TransferMetadataDto {
                id: "192.168.1.5:51234".into(),
                source: Some(RemoteDeviceDto {
                    name: "Pixel 10".into(),
                    device_type: "Phone".into(),
                }),
                pin_code: Some("4821".into()),
                payload_kind: PayloadKindDto::Files,
                payload_preview: None,
                files: Some(vec!["IMG_001.jpg".into(), "video.mp4".into()]),
                text: None,
                url: None,
                wifi_ssid: None,
                total_bytes: 38210432,
                ack_bytes: 0,
            }),
        };
        let json = serde_json::to_value(&dto).unwrap();
        assert_eq!(json["pin_code"].as_str(), None);
        assert_eq!(
            json["metadata"]["pin_code"].as_str(),
            Some("4821")
        );
        let consent = ConsentDto {
            id: "192.168.1.5:51234".into(),
            action: TransferActionDto::ConsentAccept,
        };
        let cm = consent.into_channel_message();
        assert_eq!(cm.id, "192.168.1.5:51234");
        assert!(matches!(
            cm.msg,
            Message::Lib { action: TransferAction::ConsentAccept }
        ));
    }
}
