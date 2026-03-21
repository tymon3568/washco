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

// Payment
export interface PaymentResponse {
	id: string;
	location_id: string;
	queue_entry_id: string | null;
	booking_id: string | null;
	customer_name: string;
	customer_phone: string | null;
	service_id: string;
	service_name: string;
	base_price: number;
	discount_amount: number;
	final_amount: number;
	promotion_id: string | null;
	payment_method: string;
	payment_status: string;
	paid_at: string | null;
	collected_by: string;
	verified_by: string | null;
	staff_id: string | null;
	assistant_id: string | null;
	notes: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreatePaymentRequest {
	location_id: string;
	queue_entry_id?: string;
	booking_id?: string;
	customer_name: string;
	customer_phone?: string;
	service_id: string;
	service_name: string;
	base_price: number;
	discount_amount: number;
	final_amount: number;
	promotion_id?: string;
	payment_method: string;
	staff_id?: string;
	assistant_id?: string;
	notes?: string;
}

export interface DailyRevenueResponse {
	total_revenue: number;
	completed_count: number;
	cash_amount: number;
	digital_amount: number;
	avg_per_job: number;
	pending_count: number;
}

export interface StaffEarningResponse {
	staff_id: string;
	staff_name: string;
	job_count: number;
	total_revenue: number;
	total_commission: number;
}

// Staff
export interface StaffResponse {
	id: string;
	user_id: string;
	location_id: string;
	display_name: string;
	skill_level: string;
	hourly_rate: number;
	is_active: boolean;
	joined_date: string;
	notes: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreateStaffRequest {
	user_id: string;
	display_name: string;
	skill_level: string;
	hourly_rate: number;
}

export interface ShiftResponse {
	id: string;
	location_id: string;
	staff_id: string;
	shift_date: string;
	start_time: string;
	end_time: string;
	actual_start: string | null;
	actual_end: string | null;
	status: string;
	notes: string | null;
	created_at: string;
}

export interface CommissionRuleResponse {
	id: string;
	location_id: string;
	name: string;
	service_id: string | null;
	skill_level: string | null;
	role_in_job: string;
	commission_type: string;
	commission_value: number;
	is_active: boolean;
	created_at: string;
	updated_at: string;
}

export interface CommissionSummaryResponse {
	staff_id: string;
	total_jobs: number;
	total_revenue: number;
	total_commission: number;
	period_from: string;
	period_to: string;
}

// Customer
export interface CustomerResponse {
	id: string;
	phone: string;
	name: string;
	email: string | null;
	segment: string;
	total_visits: number;
	total_spent: number;
	last_visit_at: string | null;
	loyalty_points: number;
	notes: string | null;
	tags: string[];
	created_at: string;
	updated_at: string;
}

export interface VehicleResponse {
	id: string;
	customer_id: string;
	plate_number: string | null;
	vehicle_type: string;
	brand: string | null;
	model: string | null;
	color: string | null;
	year: number | null;
	notes: string | null;
	created_at: string;
	updated_at: string;
}

export interface MembershipResponse {
	id: string;
	customer_id: string;
	plan_name: string;
	plan_type: string;
	total_uses: number | null;
	used_count: number;
	price_paid: number;
	valid_from: string;
	valid_to: string | null;
	status: string;
	created_at: string;
}

// Inventory
export interface MaterialResponse {
	id: string;
	location_id: string;
	name: string;
	category: string;
	unit: string;
	unit_cost: number;
	current_stock: number;
	min_stock: number;
	is_active: boolean;
	created_at: string;
	updated_at: string;
}

export interface LowStockAlertResponse {
	material_id: string;
	name: string;
	current_stock: number;
	min_stock: number;
	unit: string;
}

export interface MaterialVarianceResponse {
	material_id: string;
	material_name: string;
	unit: string;
	job_count: number;
	expected_usage: number;
	actual_usage: number;
	variance: number;
}

// Notification
export interface TemplateResponse {
	id: string;
	template_type: string;
	channel: string;
	subject: string | null;
	body_template: string;
	is_active: boolean;
	created_at: string;
}

export interface CreateTemplateRequest {
	template_type: string;
	channel?: string;
	subject?: string;
	body_template: string;
}

export interface UpdateTemplateRequest {
	template_type: string;
	channel?: string;
	subject?: string;
	body_template: string;
	is_active: boolean;
}

export interface SendNotificationRequest {
	recipient_phone: string;
	template_type: string;
	channel?: string;
	payload?: Record<string, unknown>;
}

export interface NotificationResponse {
	id: string;
	recipient_phone: string;
	channel: string;
	template_type: string;
	payload: Record<string, unknown>;
	rendered_body: string | null;
	status: string;
	sent_at: string | null;
	error: string | null;
	created_at: string;
}

// Pricing
export interface PricingRuleResponse {
	id: string;
	location_id: string;
	service_id: string | null;
	name: string;
	rule_type: string;
	multiplier: number;
	fixed_adjustment: number;
	conditions: Record<string, unknown>;
	priority: number;
	is_active: boolean;
	valid_from: string | null;
	valid_to: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreatePricingRuleRequest {
	location_id: string;
	service_id?: string;
	name: string;
	rule_type: string;
	multiplier: number;
	fixed_adjustment: number;
	conditions: Record<string, unknown>;
	priority: number;
	is_active: boolean;
	valid_from?: string;
	valid_to?: string;
}

export interface UpdatePricingRuleRequest {
	name: string;
	rule_type: string;
	multiplier: number;
	fixed_adjustment: number;
	conditions: Record<string, unknown>;
	priority: number;
	is_active: boolean;
	service_id?: string;
	valid_from?: string;
	valid_to?: string;
}

export interface PriceCalculationRequest {
	location_id: string;
	service_id?: string;
	base_price: number;
}

export interface AppliedRuleResponse {
	rule_id: string;
	rule_name: string;
	adjustment: number;
}

export interface PriceCalculationResponse {
	base_price: number;
	final_price: number;
	applied_rules: AppliedRuleResponse[];
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

export interface TrendDataPointResponse {
	date: string;
	revenue: number;
	completed_jobs: number;
	walk_ins: number;
	cancellations: number;
	average_wait_minutes: number;
}

export interface PeriodSummaryResponse {
	location_id: string;
	from: string;
	to: string;
	total_revenue: number;
	total_completed: number;
	total_walk_ins: number;
	total_cancellations: number;
	average_wait_minutes: number;
	busiest_day: string | null;
	peak_revenue: number;
}

export interface LocationComparisonResponse {
	location_id: string;
	location_name: string;
	total_revenue: number;
	total_completed: number;
	average_wait_minutes: number;
}

// Weather Triggers
export interface WeatherTriggerResponse {
	id: string;
	promotion_id: string;
	location_id: string;
	trigger_condition: string;
	auto_activate: boolean;
	is_active: boolean;
	last_triggered: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreateWeatherTriggerRequest {
	promotion_id: string;
	location_id: string;
	trigger_condition: string;
	auto_activate?: boolean;
}

export interface UpdateWeatherTriggerRequest {
	trigger_condition?: string;
	auto_activate?: boolean;
	is_active?: boolean;
}

export interface WeatherDataResponse {
	id: string;
	city: string;
	temperature_c: number | null;
	condition: string;
	humidity: number | null;
	fetched_at: string;
	forecast_for: string;
}

// Admin
export interface PlatformMetricsResponse {
	total_tenants: number;
	total_locations: number;
	active_locations: number;
	suspended_locations: number;
	total_users: number;
	total_queue_entries_today: number;
}

export interface AdminLocationResponse {
	id: string;
	tenant_id: string;
	tenant_name: string;
	name: string;
	city: string;
	status: string;
	created_at: string;
}

export interface AdminActionResponse {
	id: string;
	admin_user_id: string;
	action_type: string;
	target_type: string;
	target_id: string;
	reason: string | null;
	metadata: Record<string, unknown>;
	created_at: string;
}

export interface SubscriptionTierResponse {
	id: string;
	name: string;
	display_name: string;
	max_locations: number;
	max_staff: number;
	features: string[];
	sort_order: number;
	is_active: boolean;
}
