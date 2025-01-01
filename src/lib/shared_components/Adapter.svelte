<script lang="ts">
	import type { AdapterInfo, TargetInputEvent } from '$lib/models';
	import { invoke } from '@tauri-apps/api/tauri';
	import AdapterDetailsGrid from './AdapterDetailsGrid.svelte';

	let editAlias = false;

	export let adapterInfo: AdapterInfo;
	export let discoverableTimeout = adapterInfo.discoverable_timeout
	let aliasName: String = adapterInfo.alias;
	
	$: discoverableTimeoutEnabled = discoverableTimeout !== 0 && adapterInfo.is_discoverable;

	function toggleAliasEditor() {
		editAlias = !editAlias;
	}

	function updatePowered(event: TargetInputEvent) {
		invoke('set_powered', {
			powered: event.currentTarget.checked
		});
	}

	function updatePairable(event: TargetInputEvent) {
		invoke('set_pairable', {
			pairable: event.currentTarget.checked
		});
	}

	function updateDiscoverableTimeout(event: TargetInputEvent) {
		if (event.currentTarget.checked) {
			discoverableTimeout = 180;
		} else {
			discoverableTimeout = 0;
		}

		invoke('set_discoverable_timeout', {
			discoverableTimeout: discoverableTimeout
		});
	}

	function onDiscoverableTimeoutChange(event: TargetInputEvent) {
		invoke('set_discoverable_timeout', {
			discoverableTimeout: discoverableTimeout
		});
	}

	function updateDiscoverable(event: TargetInputEvent) {
		if (!event.currentTarget.checked) {
			discoverableTimeout = 0;
			invoke('set_discoverable_timeout', {
				discoverableTimeout: discoverableTimeout
			});
		}
		invoke('set_discoverable', {
			discoverable: event.currentTarget.checked
		});
	}

	function updateAlias() {
		invoke('set_alias', {
			alias: aliasName
		});
		toggleAliasEditor();
	}
</script>

<div class="block px-4">
	<nav class="level mb-2">
		<div class="level-item has-text-centered">
			<div>
				<p class="heading">Power</p>
				<div class="field">
					<input
						id="powered"
						type="checkbox"
						name="powered"
						class="switch is-rtl is-rounded"
						checked={adapterInfo.is_powered}
						on:change={updatePowered}
					/>
					<label for="powered"><i class="icofont-power"></i></label>
				</div>
			</div>
		</div>
		<div class="level-item has-text-centered">
			<div>
				<p class="heading">Pairable</p>
				<div class="field">
					<input
						id="pairable"
						type="checkbox"
						name="pairable"
						class="switch is-rtl is-rounded"
						checked={adapterInfo.pairable}
						on:change={updatePairable}
					/>
					<label for="pairable"><i class="icofont-ui-love-add"></i></label>
				</div>
			</div>
		</div>
		<div class="level-item has-text-centered">
			<div>
				<p class="mr-2">Alias name</p>
				<div class="mt-2 is-flex is-gap-2 is-align-items-center">
					<p class="title is-6" style="max-width: 200px;">{adapterInfo.alias}</p>
					<button on:click={toggleAliasEditor} class="button is-small">
						<span class="icon is-small">
							<i class="icofont-edit-alt"></i>
						</span>
					</button>
				</div>
			</div>
		</div>
	</nav>
	<hr />

	<div class="is-flex is-fullwidth is-gap-4 is-align-items-center">
		<div class="is-text-centered">
			<p class="heading">Discoverable</p>
			<div class="field">
				<input
					id="discoverable"
					type="checkbox"
					disabled={!adapterInfo.is_powered}
					name="discoverable"
					class="switch is-rtl is-rounded"
					checked={adapterInfo.is_discoverable}
					on:change={updateDiscoverable}
				/>
				<label for="discoverable"><i class="icofont-search-2"></i></label>
			</div>
		</div>
		<div class="is-flex-grow-1 is-flex is-flex-direction-column is-align-items-center" >
			<div class="is-flex is-align-items-center is-gap-2">
				<div class="field">
					<input
						id="timeout"
						disabled={!adapterInfo.is_discoverable}
						type="checkbox"
						name="discoverable"
						class="switch is-small is-rounded"
						checked={discoverableTimeoutEnabled}
						on:change={updateDiscoverableTimeout}
					/>
					<label for="timeout" class="is-medium"
						>{discoverableTimeout == 0
							? 'Visibility timeout'
							: `Visibity timeout for ${discoverableTimeout / 60} minutes`}</label
					>
				</div>
			</div>
			<input
				class="slider is-fullwidth is-success is-circle"
				step="60"
				min="180"
				disabled={!discoverableTimeoutEnabled}
				max="1800"
				bind:value={discoverableTimeout}
				on:change={onDiscoverableTimeoutChange}
				type="range"
			/>
		</div>
	</div>

	<hr />

	<AdapterDetailsGrid {adapterInfo} />

	<div class:is-active={editAlias} class="modal">
		<div class="modal-background"></div>
		<div class="modal-card">
			<section class="modal-card-body">
				<div class="field">
					<label for="alias" class="label">Alias name</label>

					<div class="control">
						<input
							name="alias"
							bind:value={aliasName}
							class="input"
							type="text"
							placeholder="Alias name"
						/>
					</div>
				</div>
			</section>
			<footer class="modal-card-foot">
				<div class="buttons">
					<button on:click={updateAlias} class="button is-success">Save changes</button>
					<button on:click={toggleAliasEditor} class="button">Cancel</button>
				</div>
			</footer>
		</div>
	</div>
</div>
