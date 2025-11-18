use crate::models::{CreateRideRequest, Ride};
use sqlx::PgPool;
use uuid::Uuid;

pub struct RidesRepository {
    // SQLx PgPool is already Arc-like internally - PgPool itself is a connection pool that's designed to be cloned cheaply and shared across threads. It's essentially a smart pointer to the underlying pool.
    pool: PgPool,
}

impl RidesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_ride(&self, request: CreateRideRequest) -> Result<Ride, sqlx::Error> {
        let ride_id = Uuid::new_v4();

        let row = sqlx::query_as::<_, (Uuid, Uuid, f64, f64, f64, f64, String, Option<Uuid>, Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            r#"
            INSERT INTO rides (id, rider_id, origin_lat, origin_lng, destination_lat, destination_lng, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'requested')
            RETURNING id, rider_id, origin_lat, origin_lng, destination_lat, destination_lng, status, 
                      driver_id, match_time, pickup_time, dropoff_time, created_at, updated_at
            "#
        )
        .bind(ride_id)
        .bind(request.rider_id)
        .bind(request.origin_lat)
        .bind(request.origin_lng)
        .bind(request.destination_lat)
        .bind(request.destination_lng)
        .fetch_one(&self.pool)
        .await?;

        Ok(Ride {
            id: row.0,
            rider_id: row.1,
            origin_lat: row.2,
            origin_lng: row.3,
            destination_lat: row.4,
            destination_lng: row.5,
            status: row.6,
            driver_id: row.7,
            match_time: row.8,
            pickup_time: row.9,
            dropoff_time: row.10,
            created_at: row.11,
            updated_at: row.12,
        })
    }

    pub async fn get_ride_by_id(&self, ride_id: Uuid) -> Result<Option<Ride>, sqlx::Error> {
        let row = sqlx::query_as::<
            _,
            (
                Uuid,
                Uuid,
                f64,
                f64,
                f64,
                f64,
                String,
                Option<Uuid>,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
                chrono::DateTime<chrono::Utc>,
                chrono::DateTime<chrono::Utc>,
            ),
        >(
            r#"
            SELECT id, rider_id, origin_lat, origin_lng, destination_lat, destination_lng, status,
                   driver_id, match_time, pickup_time, dropoff_time, created_at, updated_at
            FROM rides WHERE id = $1
            "#,
        )
        .bind(ride_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Ride {
                id: row.0,
                rider_id: row.1,
                origin_lat: row.2,
                origin_lng: row.3,
                destination_lat: row.4,
                destination_lng: row.5,
                status: row.6,
                driver_id: row.7,
                match_time: row.8,
                pickup_time: row.9,
                dropoff_time: row.10,
                created_at: row.11,
                updated_at: row.12,
            })),
            None => Ok(None),
        }
    }

    pub async fn get_rides_by_rider_id(&self, rider_id: Uuid) -> Result<Vec<Ride>, sqlx::Error> {
        let rows = sqlx::query_as::<
            _,
            (
                Uuid,
                Uuid,
                f64,
                f64,
                f64,
                f64,
                String,
                Option<Uuid>,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
                chrono::DateTime<chrono::Utc>,
                chrono::DateTime<chrono::Utc>,
            ),
        >(
            r#"
            SELECT id, rider_id, origin_lat, origin_lng, destination_lat, destination_lng, status,
                   driver_id, match_time, pickup_time, dropoff_time, created_at, updated_at
            FROM rides WHERE rider_id = $1 ORDER BY created_at DESC
            "#,
        )
        .bind(rider_id)
        .fetch_all(&self.pool)
        .await?;

        let rides = rows
            .into_iter()
            .map(|row| Ride {
                id: row.0,
                rider_id: row.1,
                origin_lat: row.2,
                origin_lng: row.3,
                destination_lat: row.4,
                destination_lng: row.5,
                status: row.6,
                driver_id: row.7,
                match_time: row.8,
                pickup_time: row.9,
                dropoff_time: row.10,
                created_at: row.11,
                updated_at: row.12,
            })
            .collect();

        Ok(rides)
    }

    pub async fn update_ride_status(
        &self,
        ride_id: Uuid,
        status: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE rides 
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(ride_id)
        .bind(status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
