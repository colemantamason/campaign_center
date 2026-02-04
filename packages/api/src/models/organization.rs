use crate::enums::SubscriptionType;
use crate::schema::organizations;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = organizations)]
#[diesel(check_for_backend(Postgres))]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub country: Option<String>,
    pub timezone: String,
    pub subscriptions: Vec<Option<String>>,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Organization {
    pub fn get_subscriptions(&self) -> Vec<SubscriptionType> {
        self.subscriptions
            .iter()
            .filter_map(|subscription| {
                subscription
                    .as_ref()
                    .and_then(|string| SubscriptionType::from_str(string))
            })
            .collect()
    }
}

#[derive(Insertable)]
#[diesel(table_name = organizations)]
pub struct NewOrganization {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub timezone: String,
    pub subscriptions: Vec<Option<String>>,
    pub created_by: i32,
}

impl NewOrganization {
    pub fn new(
        name: String,
        slug: String,
        timezone: String,
        subscriptions: Vec<SubscriptionType>,
        created_by: i32,
    ) -> Self {
        Self {
            name,
            slug,
            description: None,
            timezone,
            subscriptions: subscriptions
                .into_iter()
                .map(|subscription| Some(subscription.as_str().to_string()))
                .collect(),
            created_by,
        }
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = organizations)]
pub struct OrganizationUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub country: Option<String>,
    pub timezone: Option<String>,
    pub subscriptions: Option<Vec<Option<String>>>,
}
