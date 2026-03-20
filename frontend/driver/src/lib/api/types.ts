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

export interface BookingResponse {
	id: string;
	location_id: string;
	location_name?: string;
	service_id: string;
	service_name?: string;
	customer_name: string;
	customer_phone: string;
	vehicle_type: string;
	booking_date: string;
	time_slot: string;
	status: string;
	notes: string | null;
	created_at: string;
}
