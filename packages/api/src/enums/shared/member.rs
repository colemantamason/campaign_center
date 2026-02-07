use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum MemberRole {
    Owner,
    Admin,
    Manager,
    Member,
}

impl MemberRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            MemberRole::Owner => "owner",
            MemberRole::Admin => "admin",
            MemberRole::Manager => "manager",
            MemberRole::Member => "member",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "owner" => Some(MemberRole::Owner),
            "admin" => Some(MemberRole::Admin),
            "manager" => Some(MemberRole::Manager),
            "member" => Some(MemberRole::Member),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            MemberRole::Owner => "Owner",
            MemberRole::Admin => "Admin",
            MemberRole::Manager => "Manager",
            MemberRole::Member => "Member",
        }
    }

    pub fn can_manage(&self, target: &MemberRole) -> bool {
        match (self, target) {
            (MemberRole::Owner, _) => true,
            (MemberRole::Admin, MemberRole::Owner) => false,
            (MemberRole::Admin, _) => true,
            _ => false,
        }
    }
}

impl Display for MemberRole {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}
