use crate::models::{EventType, EventVisibility, SignupStatus};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Store)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub r#type: EventType,
    pub visibility: EventVisibility,
    pub description: Option<String>,
    pub attendee_message: Option<String>,
    pub image_url: Option<String>,
    pub location_in_person: Option<String>,
    pub location_online: Option<String>,
    pub communication_bring_a_friend: bool,
    pub communication_other_events: bool,
    pub communication_confirmation: bool,
    pub communication_check_in: bool,
    pub contact_name: String,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub co_hosts: Vec<String>,
    pub invite_groups: Vec<String>,
    pub shifts: Vec<EventShift>,
}

#[derive(Clone)]
pub struct EventShift {
    pub id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timezone: String,
    pub capacity: Option<i32>,
    pub notes: Option<String>,
    pub signups: Vec<EventSignup>,
}

#[derive(Clone)]
pub struct EventSignup {
    pub id: i32,
    pub user_id: i32,
    pub status: SignupStatus,
    pub notes: Option<String>,
    pub signed_up_at: DateTime<Utc>,
    pub checked_in_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

pub type Events = HashMap<i32, Event>;
