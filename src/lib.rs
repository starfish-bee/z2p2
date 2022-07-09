use axum::{
    routing::{get, post},
    Router,
};
use std::{error::Error, net::TcpListener};

mod routes;

pub async fn run(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscribe", post(routes::subscribe));

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
