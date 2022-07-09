use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}

pub async fn health_check() {}

pub async fn subscribe(Form(subscriber): Form<Subscriber>) -> String {
    format!("name: {}\nemail: {}", subscriber.name, subscriber.email)
}
