use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub const DEFAULT_INVITATION_EXPIRY_DAYS: i64 = 7;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum OrganizationType {
    Campaign,
    Organization,
}

impl OrganizationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrganizationType::Campaign => "campaign",
            OrganizationType::Organization => "organization",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "campaign" => Some(OrganizationType::Campaign),
            "organization" => Some(OrganizationType::Organization),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            OrganizationType::Campaign => "Campaign",
            OrganizationType::Organization => "Organization",
        }
    }
}

impl Display for OrganizationType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Expired,
}

impl InvitationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvitationStatus::Pending => "pending",
            InvitationStatus::Accepted => "accepted",
            InvitationStatus::Expired => "expired",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "pending" => Some(InvitationStatus::Pending),
            "accepted" => Some(InvitationStatus::Accepted),
            "expired" => Some(InvitationStatus::Expired),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            InvitationStatus::Pending => "Pending",
            InvitationStatus::Accepted => "Accepted",
            InvitationStatus::Expired => "Expired",
        }
    }
}

impl Display for InvitationStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}
