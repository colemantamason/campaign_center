use crate::enums::{InvitationStatus, MemberRole, OrganizationType, SubscriptionType};
use crate::error::{postgres_error, AppError};
use crate::models::{
    NewInvitation, NewOrganization, NewOrganizationMember, Organization, OrganizationMember,
    OrganizationMemberUpdate, OrganizationUpdate,
};
use crate::postgres::get_postgres_connection;
use crate::schema::{invitations, organization_members, organizations, users};
use crate::services::{
    validate_email, validate_nested_optional_string, validate_optional_slug,
    validate_optional_string, validate_required_string, MAX_ORGANIZATION_NAME_LENGTH,
    MAX_ORGANIZATION_SLUG_LENGTH,
};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use slug::slugify;

// validation constants based on database column limits
pub const MAX_ORGANIZATION_WEBSITE_URL_LENGTH: usize = 2048;
pub const MAX_ORGANIZATION_EMAIL_LENGTH: usize = 255;
pub const MAX_ORGANIZATION_PHONE_LENGTH: usize = 20;
pub const MAX_ORGANIZATION_ADDRESS_LENGTH: usize = 255;
pub const MAX_ORGANIZATION_CITY_LENGTH: usize = 100;
pub const MAX_ORGANIZATION_STATE_LENGTH: usize = 50;
pub const MAX_ORGANIZATION_ZIP_LENGTH: usize = 20;
pub const MAX_ORGANIZATION_COUNTRY_LENGTH: usize = 2;
pub const MAX_ORGANIZATION_TIMEZONE_LENGTH: usize = 50;

pub async fn create_organization(
    name: String,
    organization_type: OrganizationType,
    description: Option<String>,
    slug_override: Option<String>,
    user_id: i32,
) -> Result<(Organization, OrganizationMember), AppError> {
    validate_required_string("name", &name, MAX_ORGANIZATION_NAME_LENGTH)?;
    validate_optional_slug("slug", &slug_override, MAX_ORGANIZATION_SLUG_LENGTH)?;

    let connection = &mut get_postgres_connection().await?;

    let base_slug = slug_override
        .map(|slug| slugify(&slug))
        .unwrap_or_else(|| slugify(&name));

    let final_slug = ensure_unique_slug(connection, &base_slug).await?;

    let new_organization = NewOrganization::new(
        name.trim().to_string(),
        final_slug,
        organization_type,
        description,
        "America/New_York".to_string(), // TODO: allow timezone selection
        Vec::from([SubscriptionType::Events]), // TODO: add in subscriptions
        user_id,
    );

    let (organization, membership) = connection
        .transaction::<_, AppError, _>(|connection| {
            Box::pin(async move {
                let organization: Organization = diesel::insert_into(organizations::table)
                    .values(&new_organization)
                    .get_result(connection)
                    .await
                    .map_err(postgres_error)?;

                let owner_member =
                    NewOrganizationMember::new(organization.id, user_id, MemberRole::Owner);

                let membership: OrganizationMember =
                    diesel::insert_into(organization_members::table)
                        .values(&owner_member)
                        .get_result(connection)
                        .await
                        .map_err(postgres_error)?;

                Ok((organization, membership))
            })
        })
        .await?;

    Ok((organization, membership))
}

async fn ensure_unique_slug(
    connection: &mut AsyncPgConnection,
    base_slug: &str,
) -> Result<String, AppError> {
    let mut slug = base_slug.to_string();
    let mut counter = 1;

    loop {
        let exists: bool = organizations::table
            .filter(organizations::slug.eq(&slug))
            .count()
            .get_result::<i64>(connection)
            .await
            .map(|count| count > 0)
            .map_err(postgres_error)?;

        if !exists {
            return Ok(slug);
        }

        counter += 1;
        slug = format!("{}-{}", base_slug, counter);

        if counter > 100 {
            return Err(AppError::InternalError(
                "Could not generate unique slug".to_string(),
            ));
        }
    }
}

pub async fn get_organization_by_id(org_id: i32) -> Result<Organization, AppError> {
    let connection = &mut get_postgres_connection().await?;

    organizations::table
        .find(org_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Organization"))
}

pub async fn get_organization_by_slug(slug: &str) -> Result<Option<Organization>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    organizations::table
        .filter(organizations::slug.eq(slug))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)
}

pub async fn update_organization(
    organization_id: i32,
    update: OrganizationUpdate,
) -> Result<Organization, AppError> {
    // reject empty name if provided
    if let Some(ref name) = update.name {
        if name.trim().is_empty() {
            return Err(AppError::validation("name", "name is required"));
        }
    }

    validate_optional_string("name", &update.name, MAX_ORGANIZATION_NAME_LENGTH)?;
    validate_nested_optional_string("description", &update.description, 10_000)?;
    validate_nested_optional_string(
        "avatar_url",
        &update.avatar_url,
        MAX_ORGANIZATION_WEBSITE_URL_LENGTH,
    )?;
    validate_nested_optional_string(
        "website_url",
        &update.website_url,
        MAX_ORGANIZATION_WEBSITE_URL_LENGTH,
    )?;
    validate_nested_optional_string("email", &update.email, MAX_ORGANIZATION_EMAIL_LENGTH)?;
    validate_nested_optional_string(
        "phone_number",
        &update.phone_number,
        MAX_ORGANIZATION_PHONE_LENGTH,
    )?;
    validate_nested_optional_string(
        "address_line_1",
        &update.address_line_1,
        MAX_ORGANIZATION_ADDRESS_LENGTH,
    )?;
    validate_nested_optional_string(
        "address_line_2",
        &update.address_line_2,
        MAX_ORGANIZATION_ADDRESS_LENGTH,
    )?;
    validate_nested_optional_string("city", &update.city, MAX_ORGANIZATION_CITY_LENGTH)?;
    validate_nested_optional_string("state", &update.state, MAX_ORGANIZATION_STATE_LENGTH)?;
    validate_nested_optional_string("zip_code", &update.zip_code, MAX_ORGANIZATION_ZIP_LENGTH)?;
    validate_nested_optional_string("country", &update.country, MAX_ORGANIZATION_COUNTRY_LENGTH)?;
    validate_optional_string(
        "timezone",
        &update.timezone,
        MAX_ORGANIZATION_TIMEZONE_LENGTH,
    )?;

    let connection = &mut get_postgres_connection().await?;

    diesel::update(organizations::table.find(organization_id))
        .set(&update)
        .get_result::<Organization>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn get_membership(
    organization_id: i32,
    user_id: i32,
) -> Result<Option<OrganizationMember>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    organization_members::table
        .filter(organization_members::organization_id.eq(organization_id))
        .filter(organization_members::user_id.eq(user_id))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)
}

pub async fn list_user_organizations(
    user_id: i32,
) -> Result<Vec<(Organization, OrganizationMember)>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    organization_members::table
        .inner_join(organizations::table)
        .filter(organization_members::user_id.eq(user_id))
        .order(organizations::name.asc())
        .select((
            organizations::all_columns,
            organization_members::all_columns,
        ))
        .load::<(Organization, OrganizationMember)>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn list_organization_members(
    organization_id: i32,
) -> Result<Vec<OrganizationMember>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    organization_members::table
        .filter(organization_members::organization_id.eq(organization_id))
        .order(organization_members::joined_at.asc())
        .load::<OrganizationMember>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn get_member_by_id(member_id: i32) -> Result<OrganizationMember, AppError> {
    let connection = &mut get_postgres_connection().await?;

    organization_members::table
        .find(member_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Member"))
}

pub async fn add_member(
    organization_id: i32,
    user_id: i32,
    role: MemberRole,
    invited_by: Option<i32>,
) -> Result<OrganizationMember, AppError> {
    if role == MemberRole::Owner {
        return Err(AppError::validation(
            "role",
            "Cannot add another owner to the organization",
        ));
    }

    let existing = get_membership(organization_id, user_id).await?;

    if existing.is_some() {
        return Err(AppError::already_exists("Member"));
    }

    let connection = &mut get_postgres_connection().await?;

    let mut new_member = NewOrganizationMember::new(organization_id, user_id, role);

    if let Some(inviter) = invited_by {
        new_member = new_member.set_invited_by(inviter);
    }

    diesel::insert_into(organization_members::table)
        .values(&new_member)
        .get_result::<OrganizationMember>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn update_member_role(
    member_id: i32,
    new_role: MemberRole,
) -> Result<OrganizationMember, AppError> {
    if new_role == MemberRole::Owner {
        return Err(AppError::validation(
            "role",
            "Cannot change role to owner. Use transfer ownership instead.",
        ));
    }

    let connection = &mut get_postgres_connection().await?;

    let current: OrganizationMember = organization_members::table
        .find(member_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Member"))?;

    if current.get_role() == MemberRole::Owner {
        return Err(AppError::validation(
            "role",
            "Cannot change owner's role. Transfer ownership first.",
        ));
    }

    diesel::update(organization_members::table.find(member_id))
        .set(OrganizationMemberUpdate {
            role: Some(new_role.as_str().to_string()),
            ..Default::default()
        })
        .get_result::<OrganizationMember>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn remove_member(member_id: i32) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let member: OrganizationMember = organization_members::table
        .find(member_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Member"))?;

    if member.get_role() == MemberRole::Owner {
        return Err(AppError::validation(
            "member",
            "Cannot remove the organization owner",
        ));
    }

    diesel::delete(organization_members::table.find(member_id))
        .execute(connection)
        .await
        .map_err(postgres_error)?;

    Ok(())
}

pub async fn count_members(organization_id: i32) -> Result<i32, AppError> {
    let connection = &mut get_postgres_connection().await?;

    organization_members::table
        .filter(organization_members::organization_id.eq(organization_id))
        .count()
        .get_result::<i64>(connection)
        .await
        .map(|count| count as i32)
        .map_err(postgres_error)
}

pub async fn batch_count_members(
    organization_ids: &[i32],
) -> Result<std::collections::HashMap<i32, i32>, AppError> {
    if organization_ids.is_empty() {
        return Ok(std::collections::HashMap::new());
    }

    let connection = &mut get_postgres_connection().await?;

    let counts: Vec<(i32, i64)> = organization_members::table
        .filter(organization_members::organization_id.eq_any(organization_ids))
        .group_by(organization_members::organization_id)
        .select((
            organization_members::organization_id,
            diesel::dsl::count(organization_members::id),
        ))
        .load(connection)
        .await
        .map_err(postgres_error)?;

    Ok(counts
        .into_iter()
        .map(|(org_id, count)| (org_id, count as i32))
        .collect())
}

pub struct MemberWithUserInfo {
    pub member: OrganizationMember,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

pub async fn get_members_with_user_info(
    organization_id: i32,
) -> Result<Vec<MemberWithUserInfo>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let rows: Vec<(OrganizationMember, String, String, String)> = organization_members::table
        .inner_join(users::table.on(users::id.eq(organization_members::user_id)))
        .filter(organization_members::organization_id.eq(organization_id))
        .order(organization_members::joined_at.asc())
        .select((
            organization_members::all_columns,
            users::email,
            users::first_name,
            users::last_name,
        ))
        .load(connection)
        .await
        .map_err(postgres_error)?;

    Ok(rows
        .into_iter()
        .map(
            |(member, email, first_name, last_name)| MemberWithUserInfo {
                member,
                email,
                first_name,
                last_name,
            },
        )
        .collect())
}

pub async fn create_invitation(
    organization_id: i32,
    email: String,
    role: MemberRole,
    invited_by: i32,
) -> Result<(), AppError> {
    validate_email(&email)?;

    let email = email.to_lowercase();

    let connection = &mut get_postgres_connection().await?;

    let existing = invitations::table
        .filter(invitations::organization_id.eq(organization_id))
        .filter(invitations::email.eq(&email))
        .filter(invitations::status.eq(InvitationStatus::Pending.as_str()))
        .first::<crate::models::Invitation>(connection)
        .await
        .optional()
        .map_err(postgres_error)?;

    if let Some(invite) = existing {
        if !invite.is_expired() {
            return Err(AppError::already_exists(
                "Pending invitation for this email",
            ));
        }

        diesel::delete(invitations::table.find(invite.id))
            .execute(connection)
            .await
            .map_err(postgres_error)?;
    }

    let user_with_email = users::table
        .filter(users::email.eq(&email))
        .select(users::id)
        .first::<i32>(connection)
        .await
        .optional()
        .map_err(postgres_error)?;

    if let Some(user_id) = user_with_email {
        let already_member = organization_members::table
            .filter(organization_members::organization_id.eq(organization_id))
            .filter(organization_members::user_id.eq(user_id))
            .first::<OrganizationMember>(connection)
            .await
            .optional()
            .map_err(postgres_error)?;

        if already_member.is_some() {
            return Err(AppError::already_exists(
                "User is already a member of this organization",
            ));
        }
    }

    diesel::delete(
        invitations::table
            .filter(invitations::organization_id.eq(organization_id))
            .filter(invitations::email.eq(&email))
            .filter(invitations::status.ne(InvitationStatus::Pending.as_str())),
    )
    .execute(connection)
    .await
    .map_err(postgres_error)?;

    let invitation = NewInvitation::new(
        organization_id,
        email,
        role.as_str().to_string(),
        invited_by,
    );

    diesel::insert_into(invitations::table)
        .values(&invitation)
        .execute(connection)
        .await
        .map_err(postgres_error)?;

    Ok(())
}
