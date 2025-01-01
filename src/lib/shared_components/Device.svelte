<script lang="ts">
	import type { DeviceInfo, TargetButtonEvent } from '$lib/models';
	import { invoke } from '@tauri-apps/api/tauri';

	export let device: DeviceInfo;

	let loading = false;
	let error = false;
	let errorMsg = '';

	function connect(event: TargetButtonEvent) {
		loading = true;
		invoke('connect', {
			address: device.address
		})
			.then(() => {
				loading = false;
				error = false;
			})
			.catch((err) => {
				loading = false;
			});
	}

	function disconnect(event: TargetButtonEvent) {
		loading = true;
		invoke('disconnect', {
			address: device.address
		})
			.then(() => {
				loading = false;
				error = false;
			})
			.catch((err) => {
				loading = false;
			});
	}
</script>

<div class="my-4 p-4">
	<div class="my-1 p-4">name : {device.name}</div>
	<div class="my-1 p-4">alias : {device.alias}</div>
	<div class="my-1 p-4">address : {device.address_string}</div>
	<div class="my-1 p-4">connected : {device.is_connected}</div>
	<div class="my-1 p-4">paired : {device.is_paired}</div>
	<div class="my-1 p-4">trusted : {device.is_trusted}</div>
	<div class="my-1 p-4">blocked : {device.is_blocked}</div>
	{#if device.is_connected}
		<button
			class="button is-error"
			on:click={disconnect}
			disabled={loading}
			class:is-loading={loading}>Disconnect</button
		>
	{:else}
		<button
			class="button is-primary"
			on:click={connect}
			disabled={loading}
			class:is-loading={loading}>Connect</button
		>
	{/if}
</div>
