use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::ReviewRepository;
use crate::domain::{Review, ReviewSummary};

pub struct PgReviewRepository {
    pool: PgPool,
}

impl PgReviewRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_review(row: &sqlx::postgres::PgRow) -> Review {
    Review {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        location_id: row.get("location_id"),
        queue_entry_id: row.get("queue_entry_id"),
        customer_name: row.get("customer_name"),
        customer_phone: row.get("customer_phone"),
        rating: row.get("rating"),
        comment: row.get("comment"),
        reply: row.get("reply"),
        replied_at: row.get("replied_at"),
        created_at: row.get("created_at"),
    }
}

const REVIEW_COLS: &str = "id, tenant_id, location_id, queue_entry_id, customer_name, customer_phone, rating, comment, reply, replied_at, created_at";

impl ReviewRepository for PgReviewRepository {
    async fn create(&self, review: &Review) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO reviews
               (id, tenant_id, location_id, queue_entry_id, customer_name, customer_phone,
                rating, comment, reply, replied_at, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
        )
        .bind(review.id)
        .bind(review.tenant_id)
        .bind(review.location_id)
        .bind(review.queue_entry_id)
        .bind(&review.customer_name)
        .bind(&review.customer_phone)
        .bind(review.rating)
        .bind(&review.comment)
        .bind(&review.reply)
        .bind(review.replied_at)
        .bind(review.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_id(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<Review>> {
        let q = format!("SELECT {REVIEW_COLS} FROM reviews WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.as_ref().map(row_to_review))
    }

    async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Review>> {
        let q = format!(
            "SELECT {REVIEW_COLS} FROM reviews
             WHERE tenant_id = $1 AND location_id = $2
             ORDER BY created_at DESC
             LIMIT $3 OFFSET $4"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_review).collect())
    }

    async fn summary_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<ReviewSummary> {
        let row = sqlx::query(
            "SELECT
                COALESCE(AVG(rating::float8), 0.0) as avg_rating,
                COUNT(*) as total_count,
                COUNT(*) FILTER (WHERE rating = 1) as r1,
                COUNT(*) FILTER (WHERE rating = 2) as r2,
                COUNT(*) FILTER (WHERE rating = 3) as r3,
                COUNT(*) FILTER (WHERE rating = 4) as r4,
                COUNT(*) FILTER (WHERE rating = 5) as r5
             FROM reviews
             WHERE tenant_id = $1 AND location_id = $2",
        )
        .bind(tenant_id)
        .bind(location_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(ReviewSummary {
            average_rating: row.get::<f64, _>("avg_rating"),
            total_count: row.get::<i64, _>("total_count"),
            distribution: [
                row.get::<i64, _>("r1"),
                row.get::<i64, _>("r2"),
                row.get::<i64, _>("r3"),
                row.get::<i64, _>("r4"),
                row.get::<i64, _>("r5"),
            ],
        })
    }

    async fn set_reply(&self, tenant_id: Uuid, id: Uuid, reply: &str) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE reviews SET reply = $1, replied_at = $2 WHERE id = $3 AND tenant_id = $4",
        )
        .bind(reply)
        .bind(Utc::now())
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn exists_for_queue_entry(
        &self,
        tenant_id: Uuid,
        queue_entry_id: Uuid,
    ) -> anyhow::Result<bool> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM reviews WHERE tenant_id = $1 AND queue_entry_id = $2) as exists",
        )
        .bind(tenant_id)
        .bind(queue_entry_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.get::<bool, _>("exists"))
    }
}
