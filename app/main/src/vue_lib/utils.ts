import { invoke } from '@tauri-apps/api/core';
import { getVersion } from '@tauri-apps/api/app';
import { gt } from 'semver';
import { useToastStore } from './stores/useToastStore';
import { ToastType } from './types';
import { baseName, describeTransfer, humanSize, validateSendFiles } from '../types/qsd';
import type { EndpointDto, TransferView } from '../types/qsd';
import { useDevicesStore } from '../stores/devices';
import { useSessionStore } from '../stores/session';
import { useTransfersStore } from '../stores/transfers';

/** Progress ring style var (--progress: 0-100). Guards NaN/zero totals. */
export function getProgress(item: Pick<TransferView, 'metadata'>): string {
	const ack = item.metadata?.ack_bytes ?? 0;
	const total = item.metadata?.total_bytes ?? 0;
	if (!total || total <= 0) return '--progress: 0';
	const value = Math.min(100, Math.max(0, (ack / total) * 100));
	return `--progress: ${value}`;
}

/** File rows: basename display, full path in title. */
export function displayNames(files: string[] | null | undefined): string[] {
	return (files ?? []).map(baseName);
}

export { baseName, describeTransfer, humanSize, validateSendFiles };

/** Outbound file pick validation (folders rejected, CLI parity). Throws with toast. */
export function assertSendableFiles(files: string[]): void {
	const v = validateSendFiles(files);
	if (!v.ok) {
		useToastStore().addToast(v.error ?? 'Invalid selection', ToastType.Error);
		throw new Error(v.error ?? 'Invalid selection');
	}
}

/** Send staged files to an mDNS endpoint (F2, qsd_send_lan). */
export async function sendToEndpoint(endpointId: string, files: string[]): Promise<void> {
	const toasts = useToastStore();
	assertSendableFiles(files);
	const devices = useDevicesStore();
	const ep: EndpointDto | undefined = devices.endpoints.get(endpointId);
	if (!ep?.ip || !ep?.port) {
		toasts.addToast('Device address unknown — wait for discovery', ToastType.Error);
		throw new Error('endpoint address unknown');
	}
	const id = crypto.randomUUID();
	await useTransfersStore().sendLan({
		id,
		name: ep.name ?? 'Unknown',
		addr: `${ep.ip}:${ep.port}`,
		files,
	});
}

/** Session actions previously on TauriVM (settings/autostart/visibility). */
export async function setAutoStart(autostart: boolean): Promise<void> {
	const { enable, disable } = await import('@tauri-apps/plugin-autostart');
	if (autostart) await enable();
	else await disable();
	const { getStore } = await import('@tauri-apps/plugin-store');
	const store = (await getStore('.settings.json'))!;
	await store.set('autostart', autostart);
	await store.save();
}

export async function applyAutoStart(): Promise<boolean> {
	const { enable, disable } = await import('@tauri-apps/plugin-autostart');
	const { getStore } = await import('@tauri-apps/plugin-store');
	const store = (await getStore('.settings.json'))!;
	const v = ((await store.get<boolean>('autostart')) ?? false) as boolean;
	if (v) await enable();
	else await disable();
	return v;
}

export async function invertVisibility(): Promise<void> {
	await useSessionStore().invertVisibility();
}

export async function clearSending(): Promise<void> {
	try {
		await invoke('qsd_stop_discovery');
	} finally {
		useDevicesStore().clear();
		useSessionStore().discoveryRunning = false;
	}
}

export async function getLatestVersion(currentVersion: string | null): Promise<string | null> {
	try {
		const response = await fetch('https://api.github.com/repos/martichou/rquickshare/releases/latest');
		if (!response.ok) throw new Error(`Error: ${response.status} ${response.statusText}`);
		const data = (await response.json()) as { tag_name?: string };
		const tag = data.tag_name ?? '';
		const next = tag.startsWith('v') ? tag.substring(1) : tag;
		if (currentVersion && next && gt(next, currentVersion)) return next;
		return null;
	} catch (err) {
		console.error(err);
		return null;
	}
}

export async function loadAppVersion(): Promise<string> {
	return getVersion();
}

export function blured() {
	(document.activeElement as HTMLElement | null)?.blur?.();
}

export const utils = {
	getProgress,
	displayNames,
	assertSendableFiles,
	sendToEndpoint,
	setAutoStart,
	applyAutoStart,
	invertVisibility,
	clearSending,
	getLatestVersion,
	loadAppVersion,
	blured,
	describeTransfer,
	humanSize,
	validateSendFiles,
};
export type UtilsType = typeof utils;
