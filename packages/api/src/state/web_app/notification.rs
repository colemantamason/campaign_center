use crate::enums::NotificationType;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Store)]
pub struct Notification {
    pub id: i32,
    pub organization_id: Option<i32>,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
    pub read: bool,
}

impl Notification {
    pub fn new(
        id: i32,
        notification_type: NotificationType,
        title: String,
        message: String,
        read: bool,
    ) -> Self {
        Self {
            id,
            organization_id: None,
            notification_type,
            title,
            message,
            link: None,
            read,
        }
    }
}

pub type Notifications = HashMap<i32, Notification>;
