//! Integration tests for WashCo API
//!
//! Requires a running PostgreSQL + KeyDB:
//!   podman compose up -d postgres keydb
//!
//! Run with:
//!   DATABASE_URL=postgres://washco:washco@localhost:5432/washco_test \
//!   JWT_SECRET=test-secret-key \
//!   cargo test -p washco-server --test api_test -- --test-threads=1

use reqwest::Client;
use serde_json::{Value, json};

async fn base_url() -> String {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://washco:washco@localhost:5432/washco_test".to_string());
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "test-secret-key".to_string());

    unsafe {
        std::env::set_var("DATABASE_URL", &db_url);
        std::env::set_var("JWT_SECRET", &jwt_secret);
        std::env::set_var("HOST", "127.0.0.1");
        std::env::set_var("PORT", "0");
    }

    let config = washco_server::config::AppConfig::from_env().unwrap();
    let state = washco_server::state::AppState::new(&config).await.unwrap();
    let app = washco_server::router::build(state, &config);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    format!("http://{addr}")
}

fn client() -> Client {
    Client::new()
}

async fn authed_client(base: &str, phone: &str) -> (Client, String) {
    let c = client();

    // Register
    c.post(format!("{base}/api/v1/auth/register"))
        .json(&json!({
            "phone": phone,
            "name": "Test User",
            "business_name": "Test Wash LLC"
        }))
        .send()
        .await
        .unwrap();

    // OTP request
    c.post(format!("{base}/api/v1/auth/otp/request"))
        .json(&json!({ "phone": phone }))
        .send()
        .await
        .unwrap();

    // OTP verify
    let res = c
        .post(format!("{base}/api/v1/auth/otp/verify"))
        .json(&json!({ "phone": phone, "code": "000000" }))
        .send()
        .await
        .unwrap();
    let body: Value = res.json().await.unwrap();
    let token = body["access_token"].as_str().unwrap().to_string();

    (c, token)
}

// ── Health checks ──

#[tokio::test]
async fn health_returns_ok() {
    let base = base_url().await;
    let res = client().get(format!("{base}/health")).send().await.unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn ready_returns_ok() {
    let base = base_url().await;
    let res = client().get(format!("{base}/ready")).send().await.unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["status"], "ready");
}

// ── Auth flow ──

#[tokio::test]
async fn auth_register_and_login_flow() {
    let base = base_url().await;
    let c = client();

    // Step 1: Register
    let res = c
        .post(format!("{base}/api/v1/auth/register"))
        .json(&json!({
            "phone": "0999100001",
            "name": "Auth Test User",
            "business_name": "Auth Test Wash"
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert!(body["user_id"].is_string());

    // Step 2: Request OTP
    let res = c
        .post(format!("{base}/api/v1/auth/otp/request"))
        .json(&json!({ "phone": "0999100001" }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    // Step 3: Verify OTP (dev mode accepts "000000")
    let res = c
        .post(format!("{base}/api/v1/auth/otp/verify"))
        .json(&json!({ "phone": "0999100001", "code": "000000" }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    let access_token = body["access_token"].as_str().unwrap();
    let refresh_token = body["refresh_token"].as_str().unwrap();
    assert!(!access_token.is_empty());

    // Step 4: Get profile
    let res = c
        .get(format!("{base}/api/v1/auth/me"))
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["phone"], "0999100001");
    assert_eq!(body["name"], "Auth Test User");

    // Step 5: Refresh token
    let res = c
        .post(format!("{base}/api/v1/auth/refresh"))
        .json(&json!({ "refresh_token": refresh_token }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert!(body["access_token"].is_string());
}

// ── Unauthorized access ──

#[tokio::test]
async fn unauthenticated_request_returns_401() {
    let base = base_url().await;
    let res = client()
        .get(format!("{base}/api/v1/auth/me"))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
}

// ── Location CRUD ──

#[tokio::test]
async fn location_crud_flow() {
    let base = base_url().await;
    let (c, token) = authed_client(&base, "0999100010").await;

    // Create
    let res = c
        .post(format!("{base}/api/v1/locations"))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Test Wash Q1",
            "slug": "test-wash-q1-integ",
            "phone": "02811112222",
            "address": "100 Lê Lợi, Quận 1",
            "district": "Quận 1",
            "city": "Hồ Chí Minh",
            "latitude": 10.7756,
            "longitude": 106.7019,
            "bay_count": 3,
            "queue_mode": "hybrid"
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    let location_id = body["id"].as_str().unwrap().to_string();
    assert_eq!(body["name"], "Test Wash Q1");

    // List
    let res = c
        .get(format!("{base}/api/v1/locations"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert!(body.as_array().unwrap().len() >= 1);

    // Update
    let res = c
        .put(format!("{base}/api/v1/locations/{location_id}"))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Test Wash Q1 Updated",
            "slug": "test-wash-q1-integ",
            "phone": "02811112222",
            "address": "100 Lê Lợi, Quận 1",
            "district": "Quận 1",
            "city": "Hồ Chí Minh",
            "latitude": 10.7756,
            "longitude": 106.7019,
            "bay_count": 4,
            "queue_mode": "hybrid"
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["name"], "Test Wash Q1 Updated");
}

// ── Catalog CRUD ──

#[tokio::test]
async fn catalog_service_crud() {
    let base = base_url().await;
    let (c, token) = authed_client(&base, "0999100020").await;

    // Create location
    let res = c
        .post(format!("{base}/api/v1/locations"))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Catalog Test Loc",
            "slug": "catalog-test-integ",
            "address": "1 Test St",
            "district": "Q1",
            "city": "HCM",
            "latitude": 10.77,
            "longitude": 106.70,
            "bay_count": 2,
            "queue_mode": "hybrid"
        }))
        .send()
        .await
        .unwrap();
    let loc: Value = res.json().await.unwrap();
    let location_id = loc["id"].as_str().unwrap();

    // Create service
    let res = c
        .post(format!(
            "{base}/api/v1/catalog/locations/{location_id}/services"
        ))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Rửa xe cơ bản",
            "description": "Rửa ngoài + hút bụi",
            "vehicle_type": "car",
            "base_price": 80000,
            "duration_minutes": 20
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let svc: Value = res.json().await.unwrap();
    let service_id = svc["id"].as_str().unwrap();
    assert_eq!(svc["base_price"], 80000);

    // List
    let res = c
        .get(format!(
            "{base}/api/v1/catalog/locations/{location_id}/services"
        ))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert!(body.as_array().unwrap().len() >= 1);

    // Update
    let res = c
        .put(format!("{base}/api/v1/catalog/services/{service_id}"))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Rửa xe cơ bản v2",
            "description": "Rửa ngoài + hút bụi + lau kính",
            "vehicle_type": "car",
            "base_price": 90000,
            "duration_minutes": 25
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["base_price"], 90000);

    // Delete
    let res = c
        .delete(format!("{base}/api/v1/catalog/services/{service_id}"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
}

// ── Queue flow ──

#[tokio::test]
async fn queue_join_advance_complete_flow() {
    let base = base_url().await;
    let (c, token) = authed_client(&base, "0999100030").await;

    // Create location
    let res = c
        .post(format!("{base}/api/v1/locations"))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Queue Test Loc",
            "slug": "queue-test-integ",
            "address": "1 Q St",
            "district": "Q1",
            "city": "HCM",
            "latitude": 10.77,
            "longitude": 106.70,
            "bay_count": 2,
            "queue_mode": "hybrid"
        }))
        .send()
        .await
        .unwrap();
    let loc: Value = res.json().await.unwrap();
    let location_id = loc["id"].as_str().unwrap();

    // Create service
    let res = c
        .post(format!(
            "{base}/api/v1/catalog/locations/{location_id}/services"
        ))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Rửa cơ bản",
            "vehicle_type": "car",
            "base_price": 80000,
            "duration_minutes": 20
        }))
        .send()
        .await
        .unwrap();
    let svc: Value = res.json().await.unwrap();
    let service_id = svc["id"].as_str().unwrap();

    // Join queue
    let res = c
        .post(format!("{base}/api/v1/queue/locations/{location_id}/join"))
        .bearer_auth(&token)
        .json(&json!({
            "customer_name": "Nguyễn Test",
            "customer_phone": "0912999999",
            "vehicle_type": "car",
            "service_id": service_id
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let entry: Value = res.json().await.unwrap();
    let entry_id = entry["id"].as_str().unwrap();
    assert_eq!(entry["status"], "waiting");
    assert!(entry["queue_number"].as_i64().unwrap() >= 1);

    // Get queue
    let res = c
        .get(format!("{base}/api/v1/queue/locations/{location_id}"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    // Advance to in_progress
    let res = c
        .post(format!("{base}/api/v1/queue/{entry_id}/advance"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["status"], "in_progress");

    // Complete
    let res = c
        .post(format!("{base}/api/v1/queue/{entry_id}/complete"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["status"], "completed");
}

// ── Booking flow ──

#[tokio::test]
async fn booking_create_and_list() {
    let base = base_url().await;
    let (c, token) = authed_client(&base, "0999100040").await;

    // Create location + service
    let res = c
        .post(format!("{base}/api/v1/locations"))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Booking Test Loc",
            "slug": "booking-test-integ",
            "address": "1 B St",
            "district": "Q1",
            "city": "HCM",
            "latitude": 10.77,
            "longitude": 106.70,
            "bay_count": 2,
            "queue_mode": "hybrid"
        }))
        .send()
        .await
        .unwrap();
    let loc: Value = res.json().await.unwrap();
    let location_id = loc["id"].as_str().unwrap();

    let res = c
        .post(format!(
            "{base}/api/v1/catalog/locations/{location_id}/services"
        ))
        .bearer_auth(&token)
        .json(&json!({
            "name": "Rửa VIP",
            "vehicle_type": "car",
            "base_price": 300000,
            "duration_minutes": 60
        }))
        .send()
        .await
        .unwrap();
    let svc: Value = res.json().await.unwrap();
    let service_id = svc["id"].as_str().unwrap();

    // Create booking
    let tomorrow = (chrono::Utc::now() + chrono::Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();

    let res = c
        .post(format!("{base}/api/v1/bookings"))
        .bearer_auth(&token)
        .json(&json!({
            "location_id": location_id,
            "service_id": service_id,
            "customer_name": "Trần Booking",
            "customer_phone": "0912888888",
            "vehicle_type": "car",
            "booking_date": tomorrow,
            "time_slot": "10:00"
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["status"], "pending");

    // List bookings
    let res = c
        .get(format!("{base}/api/v1/bookings/locations/{location_id}"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
}

// ── Multi-tenancy isolation ──

#[tokio::test]
async fn multi_tenant_isolation() {
    let base = base_url().await;
    let (c_a, token_a) = authed_client(&base, "0999100050").await;
    let (c_b, token_b) = authed_client(&base, "0999100060").await;

    // Tenant A creates a location
    let res = c_a
        .post(format!("{base}/api/v1/locations"))
        .bearer_auth(&token_a)
        .json(&json!({
            "name": "Tenant A Only",
            "slug": "tenant-a-loc-integ",
            "address": "1 A St",
            "district": "Q1",
            "city": "HCM",
            "latitude": 10.77,
            "longitude": 106.70,
            "bay_count": 2,
            "queue_mode": "hybrid"
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    // Tenant B creates a location
    let res = c_b
        .post(format!("{base}/api/v1/locations"))
        .bearer_auth(&token_b)
        .json(&json!({
            "name": "Tenant B Only",
            "slug": "tenant-b-loc-integ",
            "address": "1 B St",
            "district": "Q2",
            "city": "HCM",
            "latitude": 10.78,
            "longitude": 106.71,
            "bay_count": 1,
            "queue_mode": "walkin"
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    // Tenant A should not see Tenant B's locations
    let res = c_a
        .get(format!("{base}/api/v1/locations"))
        .bearer_auth(&token_a)
        .send()
        .await
        .unwrap();
    let locations_a: Vec<Value> = res.json().await.unwrap();
    for loc in &locations_a {
        assert!(
            !loc["name"].as_str().unwrap().contains("Tenant B"),
            "Tenant A should not see Tenant B's locations"
        );
    }

    // Tenant B should not see Tenant A's locations
    let res = c_b
        .get(format!("{base}/api/v1/locations"))
        .bearer_auth(&token_b)
        .send()
        .await
        .unwrap();
    let locations_b: Vec<Value> = res.json().await.unwrap();
    for loc in &locations_b {
        assert!(
            !loc["name"].as_str().unwrap().contains("Tenant A"),
            "Tenant B should not see Tenant A's locations"
        );
    }
}
