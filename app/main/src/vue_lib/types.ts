// Legacy re-exports kept for incremental migration. New code imports from
// ../types/qsd.ts directly. stateToDisplay maps to the DTO display filter (§4.2).
export * from '../types/qsd';

import type { TransferStateDto } from '../types/qsd';

export const autostartKey = 'autostart';
export const realcloseKey = 'realclose';
export const startminimizedKey = 'startminimized';
export const visibilityKey = 'visibility';
export const downloadPathKey = 'download_path';

/** Displayed states (handshake transients hidden, §4.2). */
export const stateToDisplay: ReadonlyArray<TransferStateDto> = [
	'ReceivedPairedKeyResult',
	'WaitingForUserConsent',
	'ReceivingFiles',
	'Disconnected',
	'Finished',
	'SentIntroduction',
	'SendingFiles',
	'Cancelled',
	'Rejected',
];

export interface Toast {
	id: number;
	type: ToastType;
	message: string;
}

export enum ToastType {
	Success = 'SUCCESS',
	Error = 'ERROR',
	Info = 'INFO',
}
