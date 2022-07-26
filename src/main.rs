use std::{env, error::Error, fs::OpenOptions, io::BufWriter, net::TcpListener};

use secrecy::ExposeSecret;
use sqlx::PgPool;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use zero2prod2::{config::get_configuration, run, tracing::TimingLayer, AppContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let timing_layer = if env::var("TIMING_LOG").is_ok() {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("foo.txt")
            .unwrap();
        let writer = BufWriter::with_capacity(1000, file);
        Some(TimingLayer::new(writer))
    } else {
        None
    };

    let subscriber = tracing_subscriber::fmt().finish().with(timing_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
    let config = get_configuration()?;
    let app_address = format!("localhost:{}", config.application_port);

    let listener = TcpListener::bind(app_address)?;
    let db_pool = PgPool::connect(config.database.connection_string().expose_secret()).await?;
    run(AppContext { listener, db_pool }).await
}
