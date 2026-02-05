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
