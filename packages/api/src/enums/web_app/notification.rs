use crate::define_enum;

define_enum! {
    pub enum NotificationType {
        Info => ("info", "Information"),
        EventReminder => ("event_reminder", "Event Reminder"),
        TeamInvite => ("team_invite", "Team Invite"),
        MemberJoined => ("member_joined", "Member Joined"),
    }
}
