export interface NearbyLocation {
	id: string;
	name: string;
	slug: string;
	address: string;
	district: string;
	city: string;
	phone: string | null;
	latitude: number;
	longitude: number;
	bay_count: number;
	queue_mode: string;
	status: string;
	distance: number;
}

export interface ServiceResponse {
	id: string;
	name: string;
	description: string | null;
	vehicle_type: string;
	base_price: number;
	duration_minutes: number;
}

export interface QueueStateResponse {
	location_id: string;
	waiting: QueueEntryResponse[];
	in_progress: QueueEntryResponse[];
	completed_today: number;
	estimated_wait_minutes: number;
}

export interface QueueEntryResponse {
	id: string;
	queue_number: number;
	customer_name: string;
	vehicle_type: string;
	service_name: string;
	status: string;
	joined_at: string;
}
