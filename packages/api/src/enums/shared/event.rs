use crate::define_enum;

define_enum! {
    pub enum EventType {
        Canvassing => ("canvassing", "Canvassing"),
        PhoneBanking => ("phone_banking", "Phone Banking"),
        TextBanking => ("text_banking", "Text Banking"),
        MeetAndGreet => ("meet_and_greet", "Meet and Greet"),
        Rally => ("rally", "Rally"),
        PostcardWriting => ("postcard_writing", "Postcard Writing"),
        CommunityEvent => ("community_event", "Community Event"),
        Meeting => ("meeting", "Meeting"),
        Training => ("training", "Training"),
        WatchParty => ("watch_party", "Watch Party"),
        PollMonitoring => ("poll_monitoring", "Poll Monitoring"),
        VoterRegistration => ("voter_registration", "Voter Registration"),
        Fundraiser => ("fundraiser", "Fundraiser"),
        Other => ("other", "Other"),
    }
}

define_enum! {
    pub enum EventVisibility {
        Public => ("public", "Public"),
        Private => ("private", "Private"),
        Draft => ("draft", "Draft"),
    }
}

define_enum! {
    pub enum SignupStatus {
        SignedUp => ("signed_up", "Signed Up"),
        CheckedIn => ("checked_in", "Checked In"),
        NoShow => ("no_show", "No Show"),
        Cancelled => ("cancelled", "Cancelled"),
    }
}
