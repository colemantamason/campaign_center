use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum SubscriptionType {
    Events,
}

impl SubscriptionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubscriptionType::Events => "events",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "events" => Some(SubscriptionType::Events),
            _ => None,
        }
    }
}
