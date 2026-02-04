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
