use axum::http::StatusCode;
use axum::{Extension, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}

pub async fn health_check() {}

pub async fn subscribe(
    Form(subscriber): Form<Subscriber>,
    Extension(db_pool): Extension<PgPool>,
) -> Result<(), StatusCode> {
    query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now()
    )
    .execute(&db_pool)
    .await
    .map(|_| ())
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
