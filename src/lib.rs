use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;
use std::{error::Error, net::TcpListener};

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
        .layer(Extension(context.db_pool));

    axum::Server::from_tcp(context.listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
