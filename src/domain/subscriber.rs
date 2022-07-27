use serde::Deserialize;

use super::{SubscriberEmail, SubscriberName};

#[derive(Deserialize)]
pub struct RawSubscriber {
    pub name: String,
    pub email: String,
}

pub struct Subscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}

impl TryFrom<RawSubscriber> for Subscriber {
    type Error = String;

    fn try_from(raw: RawSubscriber) -> Result<Self, Self::Error> {
        Ok(Self {
            name: SubscriberName::parse(raw.name)?,
            email: SubscriberEmail::parse(raw.email)?,
        })
    }
}
