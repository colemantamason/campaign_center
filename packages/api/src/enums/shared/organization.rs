use crate::define_enum;

pub const DEFAULT_INVITATION_EXPIRY_DAYS: i64 = 7;

define_enum! {
    pub enum OrganizationType {
        Campaign => ("campaign", "Campaign"),
        Organization => ("organization", "Organization"),
    }
}

define_enum! {
    pub enum InvitationStatus {
        Pending => ("pending", "Pending"),
        Accepted => ("accepted", "Accepted"),
        Expired => ("expired", "Expired"),
    }
}
