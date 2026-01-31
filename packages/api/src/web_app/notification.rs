use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Store)]
pub struct Notification {
    pub id: i32,
    pub organization_id: i32,
    pub info: String,
    pub read: bool,
}

impl Notification {
    pub fn new(id: i32, organization_id: i32, info: String, read: bool) -> Self {
        Self {
            id,
            organization_id,
            info,
            read,
        }
    }
}

pub type Notifications = HashMap<i32, Notification>;

pub fn get_mock_notifications() -> Notifications {
    let create_notification =
        |id, organization_id, info, read| (id, Notification::new(id, organization_id, info, read));

    Notifications::from([
        create_notification(0, 0, "Test".to_string(), false),
        create_notification(1, 1, "Test 2".to_string(), false),
        create_notification(2, 2, "Test 3".to_string(), false),
        create_notification(3, 3, "Test 4".to_string(), false),
        create_notification(4, 4, "Test 50".to_string(), true),
        create_notification(5, 5, "Test 51".to_string(), true),
        create_notification(6, 6, "Test 51".to_string(), false),
        create_notification(7, 7, "Test 8".to_string(), false),
        create_notification(8, 8, "Test 9".to_string(), true),
    ])
}
