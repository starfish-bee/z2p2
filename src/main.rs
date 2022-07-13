use std::{error::Error, net::TcpListener};

use sqlx::PgPool;
use zero2prod2::{config::get_configuration, run, AppContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let config = get_configuration()?;
    let app_address = format!("localhost:{}", config.application_port);

    let listener = TcpListener::bind(app_address)?;
    let db_pool = PgPool::connect(&config.database.connection_string()).await?;
    run(AppContext { listener, db_pool }).await
}
