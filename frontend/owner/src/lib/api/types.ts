// Identity
export interface UserResponse {
	id: string;
	tenant_id: string;
	phone: string;
	name: string;
	role: string;
	is_verified: boolean;
}

export interface TokenResponse {
	access_token: string;
	refresh_token: string;
}

// Location
export interface LocationResponse {
	id: string;
	tenant_id: string;
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
	amenities: string[];
}

export interface CreateLocationRequest {
	name: string;
	address: string;
	district: string;
	city: string;
	latitude: number;
	longitude: number;
	phone?: string;
	bay_count: number;
	queue_mode: string;
}

// Operating Hours
export interface OperatingHoursEntry {
	id?: string;
	location_id?: string;
	day_of_week: number;
	open_time: string;
	close_time: string;
	is_closed: boolean;
}

// Bays
export interface BayResponse {
	id: string;
	location_id: string;
	name: string;
	is_active: boolean;
}

// Catalog
export interface ServiceResponse {
	id: string;
	location_id: string;
	name: string;
	description: string | null;
	vehicle_type: string;
	base_price: number;
	duration_minutes: number;
	is_active: boolean;
	sort_order: number;
}

export interface CreateServiceRequest {
	name: string;
	description?: string;
	vehicle_type: string;
	base_price: number;
	duration_minutes: number;
}

// Queue
export interface QueueEntryResponse {
	id: string;
	location_id: string;
	queue_number: number;
	customer_name: string;
	customer_phone: string | null;
	vehicle_type: string;
	service_name: string;
	bay_id: string | null;
	status: string;
	estimated_wait_minutes: number;
	joined_at: string;
	started_at: string | null;
	completed_at: string | null;
}

export interface QueueStateResponse {
	location_id: string;
	waiting: QueueEntryResponse[];
	in_progress: QueueEntryResponse[];
	completed_today: number;
	estimated_wait_minutes: number;
}

export interface JoinQueueRequest {
	customer_name: string;
	customer_phone?: string;
	vehicle_type: string;
	service_id: string;
}

// Analytics
export interface DailySummaryResponse {
	date: string;
	total_revenue: number;
	completed_jobs: number;
	walk_ins: number;
	average_wait_minutes: number;
	cancellations: number;
}

export interface ServiceMetricResponse {
	service_name: string;
	count: number;
	revenue: number;
	average_duration_minutes: number;
}
