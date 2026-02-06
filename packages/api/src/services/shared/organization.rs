use crate::enums::{MemberRole, OrganizationType, SubscriptionType};
use crate::error::AppError;
use crate::models::{
    NewOrganization, NewOrganizationMember, Organization, OrganizationMember,
    OrganizationMemberUpdate, OrganizationUpdate,
};
use crate::postgres::get_postgres_connection;
use crate::schema::{organization_members, organizations};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use slug::slugify;

pub async fn create_organization(
    name: String,
    slug_override: Option<String>,
    organization_type: OrganizationType,
    user_id: i32,
) -> Result<(Organization, OrganizationMember), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::validation(
            "name",
            "Organization name is required",
        ));
    }

    let connection = &mut get_postgres_connection().await?;

    let base_slug = slug_override
        .map(|slug| slugify(&slug))
        .unwrap_or_else(|| slugify(&name));

    let final_slug = ensure_unique_slug(connection, &base_slug).await?;

    let new_organization = NewOrganization::new(
        name.trim().to_string(),
        final_slug,
        organization_type,
        "America/New_York".to_string(), // TODO: allow timezone selection
        Vec::from([SubscriptionType::Events]), // TODO: add in subscriptions
        user_id,
    );

    let organization: Organization = diesel::insert_into(organizations::table)
        .values(&new_organization)
        .get_result(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    let owner_member = NewOrganizationMember::new(organization.id, user_id, MemberRole::Owner);

    let membership: OrganizationMember = diesel::insert_into(organization_members::table)
        .values(&owner_member)
        .get_result(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

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
            .map_err(|error| AppError::ExternalServiceError {
                service: "Postgres".to_string(),
                message: error.to_string(),
            })?;

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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
        .ok_or_else(|| AppError::not_found("Organization"))
}

pub async fn get_organization_by_slug(slug: &str) -> Result<Option<Organization>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    organizations::table
        .filter(organizations::slug.eq(slug))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn update_organization(
    organization_id: i32,
    update: OrganizationUpdate,
) -> Result<Organization, AppError> {
    let connection = &mut get_postgres_connection().await?;

    diesel::update(organizations::table.find(organization_id))
        .set(&update)
        .get_result::<Organization>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn remove_member(member_id: i32) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let member: OrganizationMember = organization_members::table
        .find(member_id)
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    Ok(counts
        .into_iter()
        .map(|(org_id, count)| (org_id, count as i32))
        .collect())
}
