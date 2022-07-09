use std::{error::Error, net::TcpListener};

pub async fn spawn_app() -> Result<String, Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(async move {
        zero2prod2::run(listener).await.expect("server crashed");
    });

    Ok(addr)
}
