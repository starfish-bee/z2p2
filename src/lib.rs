use axum::{
    body::Body,
    http::Request,
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;
use std::{error::Error, net::TcpListener};
use tower::ServiceBuilder;
use tower_http::{
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{error_span, Level};
use uuid::Uuid;

pub mod config;
mod routes;

pub struct AppContext {
    pub listener: TcpListener,
    pub db_pool: PgPool,
}

pub async fn run(context: AppContext) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscribe", post(routes::subscribe))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<Body>| {
                            let request_id = Uuid::new_v4();
                            error_span!(
                                "request",
                                id = %request_id,
                                method = %request.method(),
                                uri = %request.uri(),
                            )
                        })
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Micros),
                        ),
                )
                .layer(Extension(context.db_pool)),
        );

    axum::Server::from_tcp(context.listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
