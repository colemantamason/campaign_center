use crate::schema::{event_shifts, event_signups, events};
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(PartialEq)]
pub enum EventType {
    Canvassing,
    PhoneBanking,
    TextBanking,
    MeetAndGreet,
    Rally,
    PostcardWriting,
    CommunityEvent,
    Meeting,
    Training,
    WatchParty,
    PollMonitoring,
    VoterRegistration,
    Fundraiser,
    Other,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Canvassing => "canvassing",
            EventType::PhoneBanking => "phone_banking",
            EventType::TextBanking => "text_banking",
            EventType::MeetAndGreet => "meet_and_greet",
            EventType::Rally => "rally",
            EventType::PostcardWriting => "postcard_writing",
            EventType::CommunityEvent => "community_event",
            EventType::Meeting => "meeting",
            EventType::Training => "training",
            EventType::WatchParty => "watch_party",
            EventType::PollMonitoring => "poll_monitoring",
            EventType::VoterRegistration => "voter_registration",
            EventType::Fundraiser => "fundraiser",
            EventType::Other => "other",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "canvassing" => Some(EventType::Canvassing),
            "phone_banking" => Some(EventType::PhoneBanking),
            "text_banking" => Some(EventType::TextBanking),
            "meet_and_greet" => Some(EventType::MeetAndGreet),
            "rally" => Some(EventType::Rally),
            "postcard_writing" => Some(EventType::PostcardWriting),
            "community_event" => Some(EventType::CommunityEvent),
            "meeting" => Some(EventType::Meeting),
            "training" => Some(EventType::Training),
            "watch_party" => Some(EventType::WatchParty),
            "poll_monitoring" => Some(EventType::PollMonitoring),
            "voter_registration" => Some(EventType::VoterRegistration),
            "fundraiser" => Some(EventType::Fundraiser),
            "other" => Some(EventType::Other),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            EventType::Canvassing => "Canvassing",
            EventType::PhoneBanking => "Phone Banking",
            EventType::TextBanking => "Text Banking",
            EventType::MeetAndGreet => "Meet and Greet",
            EventType::Rally => "Rally",
            EventType::PostcardWriting => "Postcard Writing",
            EventType::CommunityEvent => "Community Event",
            EventType::Meeting => "Meeting",
            EventType::Training => "Training",
            EventType::WatchParty => "Watch Party",
            EventType::PollMonitoring => "Poll Monitoring",
            EventType::VoterRegistration => "Voter Registration",
            EventType::Fundraiser => "Fundraiser",
            EventType::Other => "Other",
        }
    }
}

impl Display for EventType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(PartialEq)]
pub enum EventVisibility {
    Public,
    Private,
    Draft,
}

impl EventVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventVisibility::Public => "public",
            EventVisibility::Private => "private",
            EventVisibility::Draft => "draft",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "public" => Some(EventVisibility::Public),
            "private" => Some(EventVisibility::Private),
            "draft" => Some(EventVisibility::Draft),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            EventVisibility::Public => "Public",
            EventVisibility::Private => "Private",
            EventVisibility::Draft => "Draft",
        }
    }
}

impl Display for EventVisibility {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(Clone, PartialEq)]
pub enum SignupStatus {
    SignedUp,
    CheckedIn,
    NoShow,
    Cancelled,
}

impl SignupStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SignupStatus::SignedUp => "signed_up",
            SignupStatus::CheckedIn => "checked_in",
            SignupStatus::NoShow => "no_show",
            SignupStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "signed_up" => Some(SignupStatus::SignedUp),
            "checked_in" => Some(SignupStatus::CheckedIn),
            "no_show" => Some(SignupStatus::NoShow),
            "cancelled" => Some(SignupStatus::Cancelled),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SignupStatus::SignedUp => "Signed Up",
            SignupStatus::CheckedIn => "Checked In",
            SignupStatus::NoShow => "No Show",
            SignupStatus::Cancelled => "Cancelled",
        }
    }
}

impl Display for SignupStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(Postgres))]
pub struct Event {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub event_type: String,
    pub visibility: String,
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
    pub co_hosts: Vec<Option<String>>,
    pub invite_groups: Vec<Option<String>>,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Event {
    pub fn get_event_type(&self) -> EventType {
        EventType::from_str(&self.event_type).unwrap_or(EventType::Other)
    }

    pub fn get_visibility(&self) -> EventVisibility {
        EventVisibility::from_str(&self.visibility).unwrap_or(EventVisibility::Draft)
    }
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub organization_id: i32,
    pub name: String,
    pub event_type: String,
    pub visibility: String,
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
    pub co_hosts: Vec<Option<String>>,
    pub invite_groups: Vec<Option<String>>,
    pub created_by: i32,
}

impl NewEvent {
    pub fn new(
        organization_id: i32,
        name: String,
        event_type: EventType,
        contact_name: String,
        created_by: i32,
    ) -> Self {
        Self {
            organization_id,
            name,
            event_type: event_type.as_str().to_string(),
            visibility: EventVisibility::Draft.as_str().to_string(),
            description: None,
            attendee_message: None,
            image_url: None,
            location_in_person: None,
            location_online: None,
            communication_bring_a_friend: false,
            communication_other_events: false,
            communication_confirmation: true,
            communication_check_in: true,
            contact_name,
            contact_email: None,
            contact_phone: None,
            co_hosts: vec![],
            invite_groups: vec![],
            created_by,
        }
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = events)]
pub struct EventUpdate {
    pub name: Option<String>,
    pub event_type: Option<String>,
    pub visibility: Option<String>,
    pub description: Option<Option<String>>,
    pub attendee_message: Option<Option<String>>,
    pub image_url: Option<Option<String>>,
    pub location_in_person: Option<Option<String>>,
    pub location_online: Option<Option<String>>,
    pub communication_bring_a_friend: Option<bool>,
    pub communication_other_events: Option<bool>,
    pub communication_confirmation: Option<bool>,
    pub communication_check_in: Option<bool>,
    pub contact_name: Option<String>,
    pub contact_email: Option<Option<String>>,
    pub contact_phone: Option<Option<String>>,
    pub co_hosts: Option<Vec<Option<String>>>,
    pub invite_groups: Option<Vec<Option<String>>>,
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = event_shifts)]
#[diesel(check_for_backend(Postgres))]
pub struct EventShift {
    pub id: i32,
    pub event_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timezone: String,
    pub capacity: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = event_shifts)]
pub struct NewEventShift {
    pub event_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timezone: String,
    pub capacity: Option<i32>,
    pub notes: Option<String>,
}

impl NewEventShift {
    pub fn new(
        event_id: i32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        timezone: String,
    ) -> Self {
        Self {
            event_id,
            start_time,
            end_time,
            timezone,
            capacity: None,
            notes: None,
        }
    }

    pub fn set_capacity(mut self, capacity: i32) -> Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn set_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = event_shifts)]
pub struct EventShiftUpdate {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub capacity: Option<Option<i32>>,
    pub notes: Option<Option<String>>,
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = event_signups)]
#[diesel(check_for_backend(Postgres))]
pub struct EventSignup {
    pub id: i32,
    pub event_shift_id: i32,
    pub user_id: i32,
    pub status: String,
    pub notes: Option<String>,
    pub signed_up_at: DateTime<Utc>,
    pub checked_in_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

impl EventSignup {
    pub fn get_status(&self) -> SignupStatus {
        SignupStatus::from_str(&self.status).unwrap_or(SignupStatus::SignedUp)
    }
}

#[derive(Insertable)]
#[diesel(table_name = event_signups)]
pub struct NewEventSignup {
    pub event_shift_id: i32,
    pub user_id: i32,
    pub status: String,
    pub notes: Option<String>,
}

impl NewEventSignup {
    pub fn new(event_shift_id: i32, user_id: i32) -> Self {
        Self {
            event_shift_id,
            user_id,
            status: SignupStatus::SignedUp.as_str().to_string(),
            notes: None,
        }
    }

    pub fn set_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = event_signups)]
pub struct EventSignupUpdate {
    pub status: Option<String>,
    pub notes: Option<Option<String>>,
    pub checked_in_at: Option<Option<DateTime<Utc>>>,
    pub cancelled_at: Option<Option<DateTime<Utc>>>,
}
