use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Default, Store)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub avatar_url: Option<String>,
    pub member_count: i32,
    pub user_role: String,
}

impl Organization {
    pub fn new(
        id: i32,
        name: String,
        avatar_url: Option<String>,
        member_count: i32,
        user_role: String,
    ) -> Self {
        Self {
            id,
            name,
            avatar_url,
            member_count,
            user_role,
        }
    }
}

pub type Organizations = HashMap<i32, Organization>;

pub fn get_mock_organizations() -> Organizations {
    let create_organization = |id, name, avatar_url, member_count, user_role| {
        (
            id,
            Organization::new(id, name, avatar_url, member_count, user_role),
        )
    };

    Organizations::from([
        create_organization(
            0,
            "Test Organization".to_string(),
            Some(
                "https://m.media-amazon.com/images/I/61siVuPB-0L._AC_UF894,1000_QL80_.jpg"
                    .to_string(),
            ),
            12,
            "Admin".to_string(),
        ),
        create_organization(
            1,
            "Test Organization 2".to_string(),
            None,
            5,
            "Member".to_string(),
        ),
        create_organization(
            2,
            "Test Organization 3".to_string(),
            None,
            8,
            "Member".to_string(),
        ),
        create_organization(
            3,
            "Test Organization 4".to_string(),
            None,
            150,
            "Member".to_string(),
        ),
        create_organization(
            4,
            "Test Organization 50".to_string(),
            None,
            3,
            "Member".to_string(),
        ),
        create_organization(
            5,
            "Test Organization 51".to_string(),
            None,
            42,
            "Member".to_string(),
        ),
        create_organization(
            6,
            "Test Organization 51".to_string(),
            None,
            1,
            "Member".to_string(),
        ),
        create_organization(
            7,
            "Test Organization 8".to_string(),
            None,
            10,
            "Member".to_string(),
        ),
        create_organization(
            8,
            "Test Organization 9".to_string(),
            None,
            7,
            "Member".to_string(),
        ),
    ])
}
