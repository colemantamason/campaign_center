use dioxus::prelude::*;

pub enum EventType {
    Canvassing,
    Phone_Banking,
    Text_Banking,
    Meet_And_Greet,
    Rally,
    Postcard_Writing,
    Community_Event,
    Meeting,
    Training,
    Watch_Party,
    Poll_Monitoring,
    Voter_Registration,
    Fundraiser,
    Other,
}

pub enum EventVisibility {
    Public,
    Private,
    Draft,
}

pub enum EventScheduleFrequency {
    Once,
    Daily,
    Weekly,
    Bi_Weekly,
    Monthly,
}

#[derive(Store, Clone, Default)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub event_type: EventType,
    pub visibility: EventVisibility,
    pub description: String,
    pub attendee_message: Option<String>,
    pub image_url: Option<String>,
    // one of the locations should be required
    pub location_in_person: Option<String>, // need address inputs & a TBD option & an option for showing the address vs sending to attendees via email
    pub location_online: Option<String>, // event link & additional instructions (optional) on how to join (String)
    pub communication_bring_a_friend: bool,
    pub communication_other_events: bool,
    pub communication_confirmation: bool,
    pub communication_check_in: bool,
    pub contact_name: String,
    // one of the contact methods should be required
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub co_hosts: Vec<String>,      // change String to co_hosts type later
    pub invite_groups: Vec<String>, // change String to group type later
    // make schedule its own struct later
    pub schedule: Vec<(
        String,
        EventScheduleFrequency,
        String,
        String,
        String,
        String,
        String,
    )>, //change to Time Zone, Frequency, Start Date, End Date, Start Time, End Time, Max Capacity types later
}

impl Event {
    pub fn new(
        id: i32,
        name: String,
        event_type: EventType,
        visibility: EventVisibility,
        description: String,
        attendee_message: Option<String>,
        image_url: Option<String>,
        location_in_person: Option<String>,
        location_online: Option<String>,
        communication_bring_a_friend: bool,
        communication_other_events: bool,
        communication_confirmation: bool,
        communication_check_in: bool,
        contact_name: String,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        co_hosts: Vec<String>,
        invite_groups: Vec<String>,
        schedule: Vec<(String, String, String, String, String, String, String)>,
    ) -> Self {
        Self {
            id,
            name,
            event_type,
            visibility,
            description,
            attendee_message,
            image_url,
            location_in_person,
            location_online,
            communication_bring_a_friend,
            communication_other_events,
            communication_confirmation,
            communication_check_in,
            contact_name,
            contact_email,
            contact_phone,
            co_hosts,
            invite_groups,
            schedule,
        }
    }
}

pub type Events = HashMap<i32, Event>;

pub fn get_mock_events() -> Events {
    let create_event = |id,
                        name,
                        event_type,
                        visibility,
                        description,
                        attendee_message,
                        image_url,
                        location_in_person,
                        location_online,
                        communication_bring_a_friend,
                        communication_other_events,
                        communication_confirmation,
                        communication_check_in,
                        contact_name,
                        contact_email,
                        contact_phone,
                        co_hosts,
                        invite_groups,
                        schedule| {
        (
            id,
            Event::new(
                id,
                name,
                event_type,
                visibility,
                description,
                attendee_message,
                image_url,
                location_in_person,
                location_online,
                communication_bring_a_friend,
                communication_other_events,
                communication_confirmation,
                communication_check_in,
                contact_name,
                contact_email,
                contact_phone,
                co_hosts,
                invite_groups,
                schedule,
            ),
        )
    };

    Events::from([create_event(
        0,
        "Test Event".to_string(),
        EventType::Canvassing,
        EventVisibility::Public,
        "This is a test event".to_string(),
        Some("Looking forward to seeing you there!".to_string()),
        None,
        Some("123 Main St, Any town, USA".to_string()),
        None,
        true,
        true,
        true,
        true,
        "John Doe".to_string(),
        Some("john.doe@example.com".to_string()),
        Some("555-1234".to_string()),
        vec!["Jane Smith".to_string()],
        vec!["Group A".to_string(), "Group B".to_string()],
        vec![(
            "UTC".to_string(),
            EventScheduleFrequency::Once,
            "2024-07-01".to_string(),
            "2024-07-01".to_string(),
            "10:00".to_string(),
            "12:00".to_string(),
            "50".to_string(),
        )],
    )])
}
