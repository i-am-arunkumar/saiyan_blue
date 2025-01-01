export type TargetInputEvent = Event & { currentTarget: EventTarget & HTMLInputElement };

export type TargetButtonEvent = Event & { currentTarget: EventTarget & HTMLButtonElement };

export type TargetAnchorEvent = Event & { currentTarget: EventTarget & HTMLAnchorElement };

export interface AdapterInfo {
	discovering: boolean;
	address: string;
	address_type: string;
	alias: string;
	pairable: boolean;
	name: string;
	is_discoverable: boolean;
	is_powered: boolean;
	system_name: string;
	pairable_timeout: number;
	discoverable_timeout: number;
	class: number;
	device_major_name: string;
	device_minor_name: string;
	service_categories: string[];
	icon: string;
	active_advertising_instances: number;
	supported_advertising_instances: number;
	supported_advertising_system_includes: string[];
	supported_advertising_features: string[];
	max_advertisement_length: number;
	max_scan_response_length: number;
	min_tx_power: number;
	max_tx_power: number;
	uuids: Map<string, string>;
}

export interface DeviceInfo {
	discovered_adapter: string;
	name: string;
	alias: string;
	address_string: string;
	address: number[];
	class: number;
	device_major_name: string;
	device_minor_name: string;
	service_categories: string[];
	uuids: Map<string, string>;
	is_paired: boolean;
	is_connected: boolean;
	is_trusted: boolean;
	is_blocked: boolean;
	is_wake_allowed: boolean;
	is_legacy_pairing: boolean;
	battery_percentage: number;
}
