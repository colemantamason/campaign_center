use crate::schema::notifications;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(PartialEq)]
pub enum NotificationType {
    Info,
    EventReminder,
    TeamInvite,
    MemberJoined,
}

impl NotificationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationType::Info => "info",
            NotificationType::EventReminder => "event_reminder",
            NotificationType::TeamInvite => "team_invite",
            NotificationType::MemberJoined => "member_joined",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "info" => Some(NotificationType::Info),
            "event_reminder" => Some(NotificationType::EventReminder),
            "team_invite" => Some(NotificationType::TeamInvite),
            "member_joined" => Some(NotificationType::MemberJoined),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            NotificationType::Info => "Information",
            NotificationType::EventReminder => "Event Reminder",
            NotificationType::TeamInvite => "Team Invite",
            NotificationType::MemberJoined => "Member Joined",
        }
    }
}

impl Display for NotificationType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = notifications)]
#[diesel(check_for_backend(Postgres))]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub organization_id: Option<i32>,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
    pub read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Notification {
    pub fn get_notification_type(&self) -> NotificationType {
        NotificationType::from_str(&self.notification_type).unwrap_or(NotificationType::Info)
    }
}

#[derive(Insertable)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub user_id: i32,
    pub organization_id: Option<i32>,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
}

impl NewNotification {
    pub fn new(
        user_id: i32,
        notification_type: NotificationType,
        title: String,
        message: String,
    ) -> Self {
        Self {
            user_id,
            organization_id: None,
            notification_type: notification_type.as_str().to_string(),
            title,
            message,
            link: None,
        }
    }

    pub fn set_organization(mut self, organization_id: i32) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    pub fn set_link(mut self, link: String) -> Self {
        self.link = Some(link);
        self
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = notifications)]
pub struct NotificationUpdate {
    pub read: Option<bool>,
    pub read_at: Option<Option<DateTime<Utc>>>,
}
