// Tipi TS speculari a `app/main/src-tauri/src/dto.rs` (unica fonte del wire
// format). Sostituisce `@martichou/core_lib/bindings` (stale, ts-rs rimosso).
// Il test Rust `dto_version_matches_ts` fallisce se DTO_VERSION diverge.

export const DTO_VERSION = 1;

export const QSD_EVENTS = {
  transfer: 'qsd://transfer',
  endpoints: 'qsd://endpoints',
  visibility: 'qsd://visibility',
  blePresence: 'qsd://ble-presence',
} as const;

export type VisibilityDto = 'Visible' | 'Invisible' | 'Temporarily';

export type TransferKindDto = 'Inbound' | 'Outbound';

export type TransferActionDto = 'ConsentAccept' | 'ConsentDecline' | 'TransferCancel';

export type TransferStateDto =
  | 'Initial'
  | 'ReceivedConnectionRequest'
  | 'SentUkeyServerInit'
  | 'SentUkeyClientInit'
  | 'SentUkeyClientFinish'
  | 'SentPairedKeyEncryption'
  | 'ReceivedUkeyClientFinish'
  | 'SentConnectionResponse'
  | 'SentPairedKeyResult'
  | 'SentIntroduction'
  | 'ReceivedPairedKeyResult'
  | 'WaitingForUserConsent'
  | 'ReceivingFiles'
  | 'SendingFiles'
  | 'Disconnected'
  | 'Rejected'
  | 'Cancelled'
  | 'Finished';

export type PayloadKindDto = 'Files' | 'Text' | 'Url' | 'WiFi';

export interface RemoteDeviceDto {
  name: string;
  device_type: string;
}

export interface TransferMetadataDto {
  id: string;
  source: RemoteDeviceDto | null;
  pin_code: string | null;
  payload_kind: PayloadKindDto;
  payload_preview: string | null;
  files: string[] | null;
  text: string | null;
  url: string | null;
  wifi_ssid: string | null;
  total_bytes: number;
  ack_bytes: number;
}

/** Evento qsd://transfer (da Message::Client). */
export interface TransferDto {
  id: string;
  kind: TransferKindDto;
  state: TransferStateDto;
  metadata: TransferMetadataDto | null;
}

/** Evento qsd://endpoints (da EndpointInfo). */
export interface EndpointDto {
  id: string;
  fullname: string;
  name: string | null;
  ip: string | null;
  port: string | null;
  device_type: string | null;
  present: boolean;
}

/** Input qsd_send_lan. `addr` = `ip:port` da EndpointDto. */
export interface SendPayloadDto {
  id: string;
  name: string;
  addr: string;
  files: string[];
}

/** Input qsd_consent. */
export interface ConsentDto {
  id: string;
  action: TransferActionDto;
}

export interface BleReceiverDto {
  address: string;
  name: string | null;
  endpoint_id: string;
}

export interface SnapshotDto {
  dto_version: number;
  visibility: VisibilityDto;
  device_name: string;
  download_path: string | null;
  port: number | null;
  discovery_running: boolean;
}

export interface VisibilityEvent {
  visibility: VisibilityDto;
}

export interface BlePresenceEvent {
  at: number;
}

export type GuiErrorCode = 'CORE' | 'VALIDATION' | 'NOT_RUNNING' | 'IO' | 'TIMEOUT';

export interface GuiError {
  code: GuiErrorCode | string;
  message: string;
}

/** Stati terminali: dopo questi l'`id` puo' essere rimosso dalla store. */
export const TERMINAL_STATES: ReadonlySet<TransferStateDto> = new Set([
  'Finished',
  'Rejected',
  'Cancelled',
  'Disconnected',
]);

export function isTerminalState(state: TransferStateDto): boolean {
  return TERMINAL_STATES.has(state);
}
