use tracing::{info, instrument};

#[instrument(level = "error")]
pub async fn health_check() {
    info!("health check called")
}
