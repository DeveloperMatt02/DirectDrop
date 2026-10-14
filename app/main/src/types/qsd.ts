// DTO mirror of app/main/src-tauri/src/dto.rs (GUI_ARCHITECTURE.md §2.3).
// Source of truth for the Tauri wire format is dto.rs: core types don't derive
// Serialize/Deserialize, only these DTOs cross invoke/events. Never import
// @martichou/core_lib here (stale bindings, ts-rs removed).
// The Rust test `dto_version_matches_ts` fails if DTO_VERSION diverges.
export const DTO_VERSION = 1;

/** Consent timeout, ms (PRODUCT.md: QSD_CONSENT_TIMEOUT default 60 s). */
export const QSD_CONSENT_TIMEOUT_MS = 60_000;

export const QSD_EVENTS = {
	transfer: 'qsd://transfer',
	endpoints: 'qsd://endpoints',
	visibility: 'qsd://visibility',
	blePresence: 'qsd://ble-presence',
} as const;

export const QSD_TRANSFER_EVENT = 'qsd://transfer';
export const QSD_ENDPOINTS_EVENT = 'qsd://endpoints';
export const QSD_VISIBILITY_EVENT = 'qsd://visibility';
export const QSD_BLE_PRESENCE_EVENT = 'qsd://ble-presence';

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

/** Event qsd://transfer (from Message::Client). */
export interface TransferDto {
	id: string;
	kind: TransferKindDto;
	state: TransferStateDto;
	metadata: TransferMetadataDto | null;
}

/** Event qsd://endpoints (from EndpointInfo). */
export interface EndpointDto {
	id: string;
	fullname: string;
	name: string | null;
	ip: string | null;
	port: string | null;
	device_type: string | null;
	present: boolean;
}

/** Input qsd_send_lan. `addr` = `ip:port` from EndpointDto. */
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

/** Payload qsd://visibility. */
export interface VisibilityEvent {
	visibility: VisibilityDto;
}

/** Payload qsd://ble-presence (`at` = unix millis). */
export interface BlePresenceEvent {
	at: number;
}

export type GuiErrorCode = 'CORE' | 'VALIDATION' | 'NOT_RUNNING' | 'IO' | 'TIMEOUT';

export interface GuiError {
	code: GuiErrorCode | string;
	message: string;
}

/** UI-only extension: TransferDto + consent deadline / expiry flag. */
export interface TransferView extends TransferDto {
	ui: {
		consent_deadline: number | null;
		expired: boolean;
	};
}

/** Terminal states: after these the `id` may be dropped from the store. */
export const TERMINAL_STATES: ReadonlySet<TransferStateDto> = new Set([
	'Finished',
	'Rejected',
	'Cancelled',
	'Disconnected',
]);

export const ACTIVE_PROGRESS_STATES: ReadonlySet<TransferStateDto> = new Set([
	'SentIntroduction',
	'SendingFiles',
	'ReceivingFiles',
]);

/** Handshake transients: compact "Connecting…" row, no actions (§4.2). */
export const HANDSHAKE_STATES: ReadonlySet<TransferStateDto> = new Set([
	'Initial',
	'ReceivedConnectionRequest',
	'SentUkeyServerInit',
	'SentUkeyClientInit',
	'SentUkeyClientFinish',
	'SentPairedKeyEncryption',
	'ReceivedUkeyClientFinish',
	'SentConnectionResponse',
	'SentPairedKeyResult',
	'ReceivedPairedKeyResult',
]);

export function isTerminalState(state: TransferStateDto): boolean {
	return TERMINAL_STATES.has(state);
}

export function humanSize(bytes: number): string {
	if (!Number.isFinite(bytes) || bytes <= 0) return '0 B';
	const units = ['B', 'KB', 'MB', 'GB', 'TB'];
	let v = bytes;
	let i = 0;
	while (v >= 1024 && i < units.length - 1) {
		v /= 1024;
		i += 1;
	}
	return `${v >= 100 ? Math.round(v) : Math.round(v * 10) / 10} ${units[i]}`;
}

/**
 * Port of CLI describe() (quickshare-direct.rs): factual one-line payload summary.
 * Files: up to 2 names + count + size. Text/Url/WiFi: short label. No payload: size only.
 */
export function describeTransfer(meta: TransferMetadataDto | null): string {
	if (!meta) return '';
	const size = humanSize(meta.total_bytes);
	const files = meta.files ?? [];
	if (files.length > 0) {
		const names =
			files.length <= 3 ? files.join(', ') : `${files.slice(0, 2).join(', ')}, … (${files.length} files)`;
		return `${names} (${size})`;
	}
	switch (meta.payload_kind) {
	case 'Text':
		return meta.payload_preview ? `Text: ${meta.payload_preview}` : 'Text';
	case 'Url':
		return meta.url ?? meta.payload_preview ?? 'Link';
	case 'WiFi':
		return meta.wifi_ssid ? `Wi-Fi network ${meta.wifi_ssid}` : 'Wi-Fi network';
	case 'Files':
	default:
		return size;
	}
}

/** Basename helper for file rows (keeps full path in title attr). */
export function baseName(path: string): string {
	const parts = path.split('/');
	return parts[parts.length - 1] || path;
}

/**
 * Send-side validation (PRODUCT.md §Capabilities): only regular files.
 * The dialog/drag layer only yields paths; folders can't be detected reliably
 * from the browser, so the backend rejects them with VALIDATION — but we fail
 * fast on obvious cases (empty selection, trailing slash = directory pick).
 */
export function validateSendFiles(files: string[]): { ok: boolean; error?: string } {
	if (files.length === 0) return { ok: false, error: 'No files selected' };
	const dirs = files.filter((f) => f.endsWith('/'));
	if (dirs.length > 0) return { ok: false, error: `${dirs[0]}: not a regular file (folders aren't supported)` };
	return { ok: true };
}
