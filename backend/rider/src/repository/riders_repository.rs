use crate::models::{CreateRiderRequest, Rider};
use sqlx::PgPool;
use uuid::Uuid;

pub struct RidersRepository {
    pool: PgPool,
}

impl RidersRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_rider(&self, request: CreateRiderRequest) -> Result<Rider, sqlx::Error> {
        let rider_id = Uuid::new_v4();

        let row = sqlx::query_as::<_, (Uuid, String, chrono::DateTime<chrono::Utc>)>(
            "INSERT INTO riders (rider_id, name) VALUES ($1, $2) RETURNING rider_id, name, created_at"
        )
        .bind(rider_id)
        .bind(request.name)
        .fetch_one(&self.pool)
        .await?;

        Ok(Rider {
            id: row.0,
            name: row.1,
            created_at: row.2,
        })
    }

    pub async fn get_rider_by_id(&self, rider_id: Uuid) -> Result<Option<Rider>, sqlx::Error> {
        let row = sqlx::query_as::<_, (Uuid, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT rider_id, name, created_at FROM riders WHERE rider_id = $1",
        )
        .bind(rider_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Rider {
                id: row.0,
                name: row.1,
                created_at: row.2,
            })),
            None => Ok(None),
        }
    }

    pub async fn get_all_riders(&self) -> Result<Vec<Rider>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (Uuid, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT rider_id, name, created_at FROM riders ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        let riders = rows
            .into_iter()
            .map(|row| Rider {
                id: row.0,
                name: row.1,
                created_at: row.2,
            })
            .collect();

        Ok(riders)
    }
}
