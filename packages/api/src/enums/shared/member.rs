use crate::define_enum;

define_enum! {
    pub enum MemberRole {
        Owner => ("owner", "Owner"),
        Admin => ("admin", "Admin"),
        Manager => ("manager", "Manager"),
        Member => ("member", "Member"),
    }
}

impl MemberRole {
    pub fn can_manage(&self, target: &MemberRole) -> bool {
        match (self, target) {
            (MemberRole::Owner, _) => true,
            (MemberRole::Admin, MemberRole::Owner) => false,
            (MemberRole::Admin, _) => true,
            _ => false,
        }
    }
}
