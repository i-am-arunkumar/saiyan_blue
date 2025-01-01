<script lang="ts">
	import type { AdapterInfo, DeviceInfo, TargetButtonEvent } from '$lib/models';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Device from './Device.svelte';

	let devices: DeviceInfo[] = [];
	onMount(() => {
		invoke('known_devices').then((result) => {
			devices = result as DeviceInfo[];
		});

		listen('devices_update', (event) => {
			const devices = event.payload as DeviceInfo[];
			console.log('device', event.payload);
		});
	});

	function scan(event: TargetButtonEvent) {
		invoke('discover_devices', {
			timeout: 10
		});
	}

	function cancel_scan(event: TargetButtonEvent) {
		invoke('cancel_discovering');
	}

	export let adapterInfo: AdapterInfo;
</script>

<div class="block">
	<div class="has-text-centered">
		<button
			class="button is-primary"
			on:click={scan}
			disabled={adapterInfo.discovering}
			class:is-loading={adapterInfo.discovering}>Scan</button
		>
		{#if adapterInfo.discovering}
			<button class="button is-error" on:click={cancel_scan}>Cancel</button>
		{/if}
	</div>
	{#each devices as device}
		<Device {device} />
	{/each}
</div>
