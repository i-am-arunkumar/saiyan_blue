<script lang="ts">
	import type { AdapterInfo } from '$lib/models';

	export let adapterInfo: AdapterInfo;
</script>
<h1 class="title is-4">Adaptor Details</h1>
<div class="fixed-grid has-3-cols-widescreen has-2-cols has-2-cols-desktop has-1-cols-mobile">
	<div class="grid is-gap-2">
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Adapter name :</p>
			<p class="title is-spaced is-6">{adapterInfo.name}</p>
		</div>
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Host name :</p>
			<p class="title is-spaced is-6">{adapterInfo.system_name}</p>
		</div>
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Address :</p>
			<p class="title is-spaced is-6">{adapterInfo.address}</p>
		</div>
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Address type :</p>
			<p class="title is-spaced is-6">{adapterInfo.address_type}</p>
		</div>
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Transmission speed :</p>
			<p class="title is-spaced is-6">
				Min {adapterInfo.min_tx_power} / Max {adapterInfo.max_tx_power}
			</p>
		</div>
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Maximum Advertisement Length :</p>
			<p class="title is-spaced is-6">{adapterInfo.max_advertisement_length}</p>
		</div>
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Active Advertisement Instances :</p>
			<p class="title is-spaced is-6">{adapterInfo.active_advertising_instances}</p>
		</div>
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Supported Advertisement Instances :</p>
			<p class="title is-spaced is-6">{adapterInfo.supported_advertising_instances}</p>
		</div>
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Maximum Scan Response :</p>
			<p class="title is-spaced is-6">{adapterInfo.max_scan_response_length}</p>
		</div>		
		<div class="cell is-flex is-align-items-center">
			<p class="mr-2">Pairable Timeout :</p>
			<p class="title is-spaced is-6">
				{adapterInfo.pairable_timeout != 0 ? adapterInfo.pairable_timeout : 'No timeout'}
			</p>
		</div>        
		{#if adapterInfo.supported_advertising_features.length !== 0}
			<div class="cell is-flex">
				<p class="mr-2">Hardware Features :</p>

				<ul class="has-text-weight-medium">
					{#each adapterInfo.supported_advertising_system_includes as feature}
						<li>{feature}</li>
					{/each}
				</ul>
			</div>
		{/if}
	</div>
</div>
<hr/>

<div class="content">
	<div class="title is-4">UUIDs</div>
<ul>
	{#each Object.entries(adapterInfo.uuids) as uuid}
		<li>{uuid[0]} ({uuid[1]})</li>
	{/each}
</ul>
</div>

{#if adapterInfo.is_powered}
	<div class="title is-4">Class of Device/Service</div>
	<div class="fixed-grid has-3-cols-widescreen has-2-cols has-2-cols-desktop has-1-cols-mobile">
		<div class="grid">
			<div class="cell is-flex is-align-items-center">
				<p class="mr-2">Class :</p>
				<p class="title is-spaced is-6">0x{adapterInfo.class.toString(16).toUpperCase()}</p>
			</div>
			<div class="cell is-flex is-align-items-center">
				<p class="mr-2">Device Type :</p>
				<p class="title is-spaced is-6">{adapterInfo.device_minor_name}</p>
			</div>
			<div class="cell is-flex">
				<p class="mr-2">Services :</p>
				<ul class="has-text-weight-medium">				
					{#each adapterInfo.service_categories  as category}
						<li>{category}</li>
					{/each}
				</ul>
			</div>
		</div>
	</div>
{/if}
