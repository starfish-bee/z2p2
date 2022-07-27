use axum::http::StatusCode;
use axum::{Extension, Form};
use chrono::Utc;
use sqlx::{query, PgPool};
use tracing::{error, info, info_span, instrument, Instrument};
use uuid::Uuid;

use crate::domain::{RawSubscriber, Subscriber};

#[instrument(
    level = "error",
    skip_all,
    fields(
        subscriber_email = %raw_subscriber.email,
        subscriber_name= %raw_subscriber.name
    )
)]
pub async fn subscribe(
    Form(raw_subscriber): Form<RawSubscriber>,
    Extension(db_pool): Extension<PgPool>,
) -> Result<(), StatusCode> {
    info!("parsing subscriber details");
    let subscriber = Subscriber::try_from(raw_subscriber).map_err(|error| {
        error!(%error, "failed to parse subscriber details");
        StatusCode::BAD_REQUEST
    })?;
    insert_subscriber(subscriber, db_pool).await
}

async fn insert_subscriber(subscriber: Subscriber, db_pool: PgPool) -> Result<(), StatusCode> {
    match query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
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
