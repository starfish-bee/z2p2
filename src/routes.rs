use axum::http::StatusCode;
use axum::{Extension, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use tracing::{error, info, info_span, instrument, Instrument};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}

#[instrument(level = "info")]
pub async fn health_check() {
    info!("health check called")
}

#[instrument(
    level = "error",
    skip_all,
    fields(
        subscriber_email = %subscriber.email,
        subscriber_name= %subscriber.name
    )
)]
pub async fn subscribe(
    Form(subscriber): Form<Subscriber>,
    Extension(db_pool): Extension<PgPool>,
) -> Result<(), StatusCode> {
    match query!(
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
    .instrument(info_span!("adding new subscriber"))
    .await
    {
        Ok(_) => {
            info!("successfully added new subscriber");
            Ok(())
        }
        Err(error) => {
            error!(%error, "failed to add new subscriber");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
