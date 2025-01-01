<script lang="ts">
	import { browser } from '$app/environment';
	import type { AdapterInfo, TargetAnchorEvent, TargetInputEvent } from '$lib/models';
	import Adapter from '$lib/shared_components/Adapter.svelte';
	import Devices from '$lib/shared_components/Devices.svelte';
	import Share from '$lib/shared_components/Share.svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let index = '0';

	let adapterInfo: AdapterInfo | null = null;

	if (browser) {
		onMount(() => {
			console.log("mounted");
			
			invoke('adapter_info').then((adapterInfoResult) => {
				console.log("got adapter");
				adapterInfo = adapterInfoResult as AdapterInfo;
			}).catch(erro => {
				console.error(erro);
				
			});
			let unmounted = false;
			let unlisten: UnlistenFn;		
			listen('adapter_info_update', (event) => {
				adapterInfo = event.payload as AdapterInfo;
				console.log(adapterInfo);
			}).then((unlistenResult) => {
				unlisten = unlistenResult;
				if (unmounted) {
					unlisten();
				}
			});
			return () => {
				if (unlisten) {
					unlisten();
				} else {
					unmounted = true;
				}
			};
		});
	}

	function set_index(event: TargetAnchorEvent) {
		index = event.currentTarget.id;
	}

	function updatePowered(event: TargetInputEvent) {
		invoke('set_powered', {
			powered: event.currentTarget.checked
		});
	}
</script>

<div class="container px-4 py-6 is-flex is-flex-direction-column" style="height: 100vh;">
	<div class="is-flex is-gap-4 is-align-items-centered">
		<h1 class="title"><span>Saiyan Blue</span></h1>
		<div class="field">
			<input
				id="powered"
				type="checkbox"
				name="powered"
				class="switch is-rounded"
				checked={adapterInfo?.is_powered}
				on:change={updatePowered}
			/>
			<label for="powered"><i class="icofont-power"></i></label>
		</div>
	</div>
	<div class="tabs is-flex-grow-0 is-flex-shrink-0 is-boxed">
		<ul>
			<li class:is-active={index == '0'}>
				<a id="0" href="/" on:click={set_index}>
					<span class="icon is-small">
						<i class="icofont-android-nexus"></i>
					</span>
					<span>Devices</span>
				</a>
			</li>
			<li class:is-active={index == '1'}>
				<a id="1" href="/" on:click={set_index}>
					<span class="icon is-small">
						<i class="icofont-share"></i>
					</span>
					<span>Share</span>
				</a>
			</li>
			<li class:is-active={index == '2'}>
				<a id="2" href="/" on:click={set_index}>
					<span class="icon is-small">
						<i class="icofont-bluetooth"></i>
					</span>
					<span>Adapter </span>
				</a>
			</li>
		</ul>
	</div>

	<div class="is-flex-grow-1" style="overflow-y: scroll;">
		{#if adapterInfo != null}
			{#if index == '1'}
				<Share {adapterInfo} />
			{/if}
			{#if index == '2'}
				<Adapter discoverableTimeout={adapterInfo.discoverable_timeout} {adapterInfo} />
			{/if}
			{#if index == '0'}
				<Devices {adapterInfo} />
			{/if}
		{:else}
			<progress class="progress is-small is-primary" max="100">15%</progress>
		{/if}
	</div>
</div>
