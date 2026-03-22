// Auth
export interface TokenResponse {
	access_token: string;
	refresh_token: string;
}

export interface UserResponse {
	id: string;
	tenant_id: string | null;
	phone: string;
	name: string | null;
	role: string;
	is_verified: boolean;
}

// Location
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
	distance_meters: number;
}

export interface LocationDetail {
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
	cover_image_url: string | null;
	description: string | null;
	amenities: string[];
	payment_methods: string[];
}

export interface OperatingHour {
	day_of_week: number;
	open_time: string;
	close_time: string;
	is_closed: boolean;
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

// Queue
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
	customer_phone: string;
	vehicle_type: string;
	service_name: string;
	bay_id: string | null;
	status: string;
	joined_at: string;
	estimated_wait_minutes: number;
}

// Booking
export interface BookingResponse {
	id: string;
	location_id: string;
	location_name: string | null;
	service_id: string;
	service_name: string | null;
	customer_name: string;
	customer_phone: string;
	vehicle_type: string;
	plate_number: string | null;
	booking_date: string;
	time_slot: string;
	status: string;
	notes: string | null;
	estimated_price: number | null;
	created_at: string;
}

export interface CreateBookingRequest {
	location_id: string;
	service_id: string;
	customer_name: string;
	customer_phone: string;
	vehicle_type: string;
	plate_number?: string;
	booking_date: string;
	time_slot: string;
	notes?: string;
}

// Review
export interface ReviewResponse {
	id: string;
	location_id: string;
	customer_name: string;
	customer_phone: string | null;
	rating: number;
	comment: string | null;
	reply: string | null;
	replied_at: string | null;
	created_at: string;
}

export interface CreateReviewRequest {
	location_id: string;
	customer_name: string;
	customer_phone?: string;
	rating: number;
	comment?: string;
}

// Promotion
export interface PromotionResponse {
	id: string;
	name: string;
	description: string | null;
	code: string;
	discount_type: string;
	discount_value: number;
	min_order_amount: number | null;
	max_discount: number | null;
	starts_at: string;
	ends_at: string;
	is_active: boolean;
}

export interface PromoValidationResponse {
	valid: boolean;
	promotion: PromotionResponse | null;
	discount_amount: number;
	message: string;
}

// Pricing
export interface PriceCalculation {
	base_price: number;
	adjustments: PriceAdjustment[];
	final_price: number;
}

export interface PriceAdjustment {
	rule_name: string;
	adjustment_type: string;
	amount: number;
}

// Customer
export interface CustomerProfile {
	id: string;
	phone: string;
	name: string | null;
	email: string | null;
	segment: string;
	loyalty_points: number;
	total_visits: number;
	total_spent: number;
}

export interface VehicleResponse {
	id: string;
	customer_id: string;
	plate_number: string;
	vehicle_type: string;
	brand: string | null;
	model: string | null;
	color: string | null;
}
