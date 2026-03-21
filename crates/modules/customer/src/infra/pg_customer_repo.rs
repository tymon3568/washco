use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::application::{
    AddServiceRecordInput, AddVehicleInput, CreateCustomerInput, CreateMembershipInput,
    CustomerRepository, UpdateCustomerInput,
};
use crate::domain::{
    Customer, CustomerSegment, Membership, MembershipStatus, MembershipType, ServiceHistoryEntry,
    Vehicle, VehicleType,
};

pub struct PgCustomerRepository {
    pool: PgPool,
}

impl PgCustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// -- Row types --

#[derive(FromRow)]
struct CustomerRow {
    id: Uuid,
    tenant_id: Uuid,
    phone: String,
    name: String,
    email: Option<String>,
    segment: String,
    total_visits: i32,
    total_spent: i64,
    last_visit_at: Option<DateTime<Utc>>,
    loyalty_points: i32,
    notes: Option<String>,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CustomerRow {
    fn into_customer(self) -> Customer {
        let tags = self.tags;
        Customer {
            id: self.id,
            tenant_id: self.tenant_id,
            phone: self.phone,
            name: self.name,
            email: self.email,
            segment: CustomerSegment::from_str(&self.segment).unwrap_or(CustomerSegment::New),
            total_visits: self.total_visits,
            total_spent: self.total_spent,
            last_visit_at: self.last_visit_at,
            loyalty_points: self.loyalty_points,
            notes: self.notes,
            tags,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(FromRow)]
struct VehicleRow {
    id: Uuid,
    tenant_id: Uuid,
    customer_id: Uuid,
    plate_number: Option<String>,
    vehicle_type: String,
    brand: Option<String>,
    model: Option<String>,
    color: Option<String>,
    year: Option<i32>,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl VehicleRow {
    fn into_vehicle(self) -> Vehicle {
        Vehicle {
            id: self.id,
            tenant_id: self.tenant_id,
            customer_id: self.customer_id,
            plate_number: self.plate_number,
            vehicle_type: VehicleType::from_str(&self.vehicle_type).unwrap_or(VehicleType::Sedan),
            brand: self.brand,
            model: self.model,
            color: self.color,
            year: self.year,
            notes: self.notes,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(FromRow)]
struct ServiceHistoryRow {
    id: Uuid,
    tenant_id: Uuid,
    vehicle_id: Uuid,
    customer_id: Uuid,
    location_id: Uuid,
    payment_id: Option<Uuid>,
    service_id: Uuid,
    service_name: String,
    amount_paid: i64,
    staff_name: Option<String>,
    notes: Option<String>,
    next_recommended_date: Option<NaiveDate>,
    next_recommended_service: Option<String>,
    serviced_at: DateTime<Utc>,
}

impl ServiceHistoryRow {
    fn into_entry(self) -> ServiceHistoryEntry {
        ServiceHistoryEntry {
            id: self.id,
            tenant_id: self.tenant_id,
            vehicle_id: self.vehicle_id,
            customer_id: self.customer_id,
            location_id: self.location_id,
            payment_id: self.payment_id,
            service_id: self.service_id,
            service_name: self.service_name,
            amount_paid: self.amount_paid,
            staff_name: self.staff_name,
            notes: self.notes,
            next_recommended_date: self.next_recommended_date,
            next_recommended_service: self.next_recommended_service,
            serviced_at: self.serviced_at,
        }
    }
}

#[derive(FromRow)]
struct MembershipRow {
    id: Uuid,
    tenant_id: Uuid,
    customer_id: Uuid,
    plan_name: String,
    plan_type: String,
    total_uses: Option<i32>,
    used_count: i32,
    price_paid: i64,
    valid_from: NaiveDate,
    valid_to: Option<NaiveDate>,
    status: String,
    created_at: DateTime<Utc>,
}

impl MembershipRow {
    fn into_membership(self) -> Membership {
        Membership {
            id: self.id,
            tenant_id: self.tenant_id,
            customer_id: self.customer_id,
            plan_name: self.plan_name,
            plan_type: MembershipType::from_str(&self.plan_type)
                .unwrap_or(MembershipType::WashCount),
            total_uses: self.total_uses,
            used_count: self.used_count,
            price_paid: self.price_paid,
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            status: MembershipStatus::from_str(&self.status).unwrap_or(MembershipStatus::Active),
            created_at: self.created_at,
        }
    }
}

// -- Repository impl --

impl CustomerRepository for PgCustomerRepository {
    // -- Customers --

    async fn create_customer(
        &self,
        tenant_id: Uuid,
        input: &CreateCustomerInput,
    ) -> anyhow::Result<Customer> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO customers
               (id, tenant_id, phone, name, email, segment, total_visits, total_spent,
                loyalty_points, notes, tags, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, 0, 0, 0, $7, $8, $9, $10)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(&input.phone)
        .bind(&input.name)
        .bind(&input.email)
        .bind(CustomerSegment::New.as_str())
        .bind(&input.notes)
        .bind(&input.tags as &[String])
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(Customer {
            id,
            tenant_id,
            phone: input.phone.clone(),
            name: input.name.clone(),
            email: input.email.clone(),
            segment: CustomerSegment::New,
            total_visits: 0,
            total_spent: 0,
            last_visit_at: None,
            loyalty_points: 0,
            notes: input.notes.clone(),
            tags: input.tags.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    async fn find_by_phone(
        &self,
        tenant_id: Uuid,
        phone: &str,
    ) -> anyhow::Result<Option<Customer>> {
        let row = sqlx::query_as::<_, CustomerRow>(
            r#"SELECT id, tenant_id, phone, name, email, segment,
                      total_visits, total_spent, last_visit_at, loyalty_points,
                      notes, tags, created_at, updated_at
               FROM customers
               WHERE tenant_id = $1 AND phone = $2"#,
        )
        .bind(tenant_id)
        .bind(phone)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(CustomerRow::into_customer))
    }

    async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Customer> {
        let row = sqlx::query_as::<_, CustomerRow>(
            r#"SELECT id, tenant_id, phone, name, email, segment,
                      total_visits, total_spent, last_visit_at, loyalty_points,
                      notes, tags, created_at, updated_at
               FROM customers
               WHERE tenant_id = $1 AND id = $2"#,
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into_customer())
    }

    async fn list_customers(
        &self,
        tenant_id: Uuid,
        segment: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Customer>> {
        let rows = if let Some(seg) = segment {
            sqlx::query_as::<_, CustomerRow>(
                r#"SELECT id, tenant_id, phone, name, email, segment,
                          total_visits, total_spent, last_visit_at, loyalty_points,
                          notes, tags, created_at, updated_at
                   FROM customers
                   WHERE tenant_id = $1 AND segment = $2
                   ORDER BY created_at DESC
                   LIMIT $3 OFFSET $4"#,
            )
            .bind(tenant_id)
            .bind(seg)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, CustomerRow>(
                r#"SELECT id, tenant_id, phone, name, email, segment,
                          total_visits, total_spent, last_visit_at, loyalty_points,
                          notes, tags, created_at, updated_at
                   FROM customers
                   WHERE tenant_id = $1
                   ORDER BY created_at DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(tenant_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows.into_iter().map(CustomerRow::into_customer).collect())
    }

    async fn update_customer(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: &UpdateCustomerInput,
    ) -> anyhow::Result<Customer> {
        let now = Utc::now();

        // Fetch current customer
        let current = self.get_by_id(tenant_id, id).await?;

        let name = input.name.as_deref().unwrap_or(&current.name);
        let email = input.email.as_ref().or(current.email.as_ref());
        let notes = input.notes.as_ref().or(current.notes.as_ref());
        let tags = input.tags.as_ref().unwrap_or(&current.tags);
        let loyalty_points = input.loyalty_points.unwrap_or(current.loyalty_points);
        sqlx::query(
            r#"UPDATE customers
               SET name = $1, email = $2, notes = $3, tags = $4,
                   loyalty_points = $5, updated_at = $6
               WHERE id = $7 AND tenant_id = $8"#,
        )
        .bind(name)
        .bind(email)
        .bind(notes)
        .bind(tags as &[String])
        .bind(loyalty_points)
        .bind(now)
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        self.get_by_id(tenant_id, id).await
    }

    async fn update_visit_stats(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        amount: i64,
    ) -> anyhow::Result<()> {
        let now = Utc::now();

        sqlx::query(
            r#"UPDATE customers
               SET total_visits = total_visits + 1,
                   total_spent = total_spent + $1,
                   last_visit_at = $2,
                   updated_at = $2
               WHERE id = $3 AND tenant_id = $4"#,
        )
        .bind(amount)
        .bind(now)
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        // Recompute segment
        let customer = self.get_by_id(tenant_id, id).await?;
        let new_segment = crate::application::CustomerService::<Self>::compute_segment(
            customer.total_visits,
            customer.last_visit_at,
        );

        sqlx::query(r#"UPDATE customers SET segment = $1 WHERE id = $2 AND tenant_id = $3"#)
            .bind(new_segment.as_str())
            .bind(id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // -- Vehicles --

    async fn add_vehicle(
        &self,
        tenant_id: Uuid,
        input: &AddVehicleInput,
    ) -> anyhow::Result<Vehicle> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let vehicle_type = VehicleType::from_str(&input.vehicle_type).unwrap_or(VehicleType::Sedan);

        sqlx::query(
            r#"INSERT INTO vehicles
               (id, tenant_id, customer_id, plate_number, vehicle_type,
                brand, model, color, year, notes, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.customer_id)
        .bind(&input.plate_number)
        .bind(vehicle_type.as_str())
        .bind(&input.brand)
        .bind(&input.model)
        .bind(&input.color)
        .bind(input.year)
        .bind(&input.notes)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(Vehicle {
            id,
            tenant_id,
            customer_id: input.customer_id,
            plate_number: input.plate_number.clone(),
            vehicle_type,
            brand: input.brand.clone(),
            model: input.model.clone(),
            color: input.color.clone(),
            year: input.year,
            notes: input.notes.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    async fn list_vehicles(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
    ) -> anyhow::Result<Vec<Vehicle>> {
        let rows = sqlx::query_as::<_, VehicleRow>(
            r#"SELECT id, tenant_id, customer_id, plate_number, vehicle_type,
                      brand, model, color, year, notes, created_at, updated_at
               FROM vehicles
               WHERE tenant_id = $1 AND customer_id = $2
               ORDER BY created_at DESC"#,
        )
        .bind(tenant_id)
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(VehicleRow::into_vehicle).collect())
    }

    async fn find_by_plate(&self, tenant_id: Uuid, plate: &str) -> anyhow::Result<Option<Vehicle>> {
        let row = sqlx::query_as::<_, VehicleRow>(
            r#"SELECT id, tenant_id, customer_id, plate_number, vehicle_type,
                      brand, model, color, year, notes, created_at, updated_at
               FROM vehicles
               WHERE tenant_id = $1 AND plate_number = $2"#,
        )
        .bind(tenant_id)
        .bind(plate)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(VehicleRow::into_vehicle))
    }

    // -- Service history --

    async fn add_service_record(
        &self,
        tenant_id: Uuid,
        input: &AddServiceRecordInput,
    ) -> anyhow::Result<ServiceHistoryEntry> {
        let id = Uuid::now_v7();
        let now = Utc::now();

        sqlx::query(
            r#"INSERT INTO service_history
               (id, tenant_id, vehicle_id, customer_id, location_id, payment_id,
                service_id, service_name, amount_paid, staff_name, notes,
                next_recommended_date, next_recommended_service, serviced_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.vehicle_id)
        .bind(input.customer_id)
        .bind(input.location_id)
        .bind(input.payment_id)
        .bind(input.service_id)
        .bind(&input.service_name)
        .bind(input.amount_paid)
        .bind(&input.staff_name)
        .bind(&input.notes)
        .bind(input.next_recommended_date)
        .bind(&input.next_recommended_service)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(ServiceHistoryEntry {
            id,
            tenant_id,
            vehicle_id: input.vehicle_id,
            customer_id: input.customer_id,
            location_id: input.location_id,
            payment_id: input.payment_id,
            service_id: input.service_id,
            service_name: input.service_name.clone(),
            amount_paid: input.amount_paid,
            staff_name: input.staff_name.clone(),
            notes: input.notes.clone(),
            next_recommended_date: input.next_recommended_date,
            next_recommended_service: input.next_recommended_service.clone(),
            serviced_at: now,
        })
    }

    async fn vehicle_history(
        &self,
        tenant_id: Uuid,
        vehicle_id: Uuid,
    ) -> anyhow::Result<Vec<ServiceHistoryEntry>> {
        let rows = sqlx::query_as::<_, ServiceHistoryRow>(
            r#"SELECT id, tenant_id, vehicle_id, customer_id, location_id, payment_id,
                      service_id, service_name, amount_paid, staff_name, notes,
                      next_recommended_date, next_recommended_service, serviced_at
               FROM service_history
               WHERE tenant_id = $1 AND vehicle_id = $2
               ORDER BY serviced_at DESC"#,
        )
        .bind(tenant_id)
        .bind(vehicle_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(ServiceHistoryRow::into_entry)
            .collect())
    }

    async fn due_reminders(
        &self,
        tenant_id: Uuid,
        as_of: NaiveDate,
    ) -> anyhow::Result<Vec<ServiceHistoryEntry>> {
        let rows = sqlx::query_as::<_, ServiceHistoryRow>(
            r#"SELECT id, tenant_id, vehicle_id, customer_id, location_id, payment_id,
                      service_id, service_name, amount_paid, staff_name, notes,
                      next_recommended_date, next_recommended_service, serviced_at
               FROM service_history
               WHERE tenant_id = $1
                 AND next_recommended_date IS NOT NULL
                 AND next_recommended_date <= $2
               ORDER BY next_recommended_date ASC"#,
        )
        .bind(tenant_id)
        .bind(as_of)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(ServiceHistoryRow::into_entry)
            .collect())
    }

    // -- Memberships --

    async fn create_membership(
        &self,
        tenant_id: Uuid,
        input: &CreateMembershipInput,
    ) -> anyhow::Result<Membership> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let plan_type =
            MembershipType::from_str(&input.plan_type).unwrap_or(MembershipType::WashCount);

        sqlx::query(
            r#"INSERT INTO memberships
               (id, tenant_id, customer_id, plan_name, plan_type, total_uses,
                used_count, price_paid, valid_from, valid_to, status, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, 0, $7, $8, $9, $10, $11)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.customer_id)
        .bind(&input.plan_name)
        .bind(plan_type.as_str())
        .bind(input.total_uses)
        .bind(input.price_paid)
        .bind(input.valid_from)
        .bind(input.valid_to)
        .bind(MembershipStatus::Active.as_str())
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(Membership {
            id,
            tenant_id,
            customer_id: input.customer_id,
            plan_name: input.plan_name.clone(),
            plan_type,
            total_uses: input.total_uses,
            used_count: 0,
            price_paid: input.price_paid,
            valid_from: input.valid_from,
            valid_to: input.valid_to,
            status: MembershipStatus::Active,
            created_at: now,
        })
    }

    async fn list_memberships(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
    ) -> anyhow::Result<Vec<Membership>> {
        let rows = sqlx::query_as::<_, MembershipRow>(
            r#"SELECT id, tenant_id, customer_id, plan_name, plan_type,
                      total_uses, used_count, price_paid, valid_from, valid_to,
                      status, created_at
               FROM memberships
               WHERE tenant_id = $1 AND customer_id = $2
               ORDER BY created_at DESC"#,
        )
        .bind(tenant_id)
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(MembershipRow::into_membership)
            .collect())
    }

    async fn use_membership(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Membership> {
        sqlx::query(
            r#"UPDATE memberships
               SET used_count = used_count + 1
               WHERE id = $1 AND tenant_id = $2 AND status = 'active'"#,
        )
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        let row = sqlx::query_as::<_, MembershipRow>(
            r#"SELECT id, tenant_id, customer_id, plan_name, plan_type,
                      total_uses, used_count, price_paid, valid_from, valid_to,
                      status, created_at
               FROM memberships
               WHERE id = $1 AND tenant_id = $2"#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into_membership())
    }
}
