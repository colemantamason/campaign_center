use crate::enums::MemberRole;
#[cfg(feature = "server")]
use crate::http::get_session_token_from_headers;
#[cfg(feature = "server")]
use crate::initialize_databases;
use crate::interfaces::{
    CreateOrganizationRequest, InviteMemberRequest, OrganizationMemberResponse,
    OrganizationResponse,
};
#[cfg(feature = "server")]
use crate::models::NewInvitation;
#[cfg(feature = "server")]
use crate::postgres::get_postgres_connection;
#[cfg(feature = "server")]
use crate::redis::{
    cache_session, get_cached_session, update_cached_session_active_organization_membership_id,
    CachedSession,
};
#[cfg(feature = "server")]
use crate::schema::invitations;
#[cfg(feature = "server")]
use crate::services::{
    create_organization as create_organization_service, get_membership, get_organization_by_id,
    get_user_by_id, list_organization_members, list_user_organizations, remove_member,
    set_active_organization as set_active_organization_service, update_member_role,
    validate_session,
};
#[cfg(feature = "server")]
use diesel_async::RunQueryDsl;
use dioxus::{fullstack::HeaderMap, prelude::*};
use uuid::Uuid;

#[cfg(feature = "server")]
struct ValidatedSession {
    pub session_id: i32,
    pub user_id: i32,
}

#[cfg(feature = "server")]
async fn get_validated_session_from_headers(
    headers: &HeaderMap,
) -> Result<(String, ValidatedSession), ServerFnError> {
    let token_string = get_session_token_from_headers(headers)
        .ok_or_else(|| ServerFnError::new("Not authenticated"))?;

    let token =
        Uuid::parse_str(&token_string).map_err(|_| ServerFnError::new("Invalid session token"))?;

    // get session from cache or database
    if let Ok(Some(cached)) = get_cached_session(&token_string).await {
        return Ok((
            token_string,
            ValidatedSession {
                session_id: cached.session_id,
                user_id: cached.user_id,
            },
        ));
    }

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let cached = CachedSession {
        session_id: session.id,
        user_id: session.user_id,
        active_organization_membership_id: session.active_organization_membership_id,
    };

    cache_session(&token_string, &cached).await.ok();

    Ok((
        token_string,
        ValidatedSession {
            session_id: session.id,
            user_id: session.user_id,
        },
    ))
}

#[post("/api/org/create", headers: HeaderMap)]
pub async fn create_organization(
    request: CreateOrganizationRequest,
) -> Result<OrganizationResponse, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let (token_string, session) = get_validated_session_from_headers(&headers).await?;

    let (organization, membership) =
        create_organization_service(request.name, request.slug, session.user_id)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

    set_active_organization_service(session.session_id, Some(membership.id))
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    update_cached_session_active_organization_membership_id(&token_string, Some(membership.id))
        .await
        .ok();

    Ok(OrganizationResponse {
        id: organization.id,
        name: organization.name,
        slug: organization.slug,
        description: organization.description,
    })
}

#[post("/api/org/set-active", headers: HeaderMap)]
pub async fn set_active_organization(organization_id: i32) -> Result<(), ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let (token_string, session) = get_validated_session_from_headers(&headers).await?;

    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .ok_or_else(|| ServerFnError::new("Not a member of this organization"))?;

    set_active_organization_service(session.session_id, Some(membership.id))
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    update_cached_session_active_organization_membership_id(&token_string, Some(membership.id))
        .await
        .ok();

    Ok(())
}

#[get("/api/org/list", headers: HeaderMap)]
pub async fn get_user_organizations() -> Result<Vec<OrganizationResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let (_token_string, session) = get_validated_session_from_headers(&headers).await?;

    let organizations = list_user_organizations(session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(organizations
        .into_iter()
        .map(|(organization, _member)| OrganizationResponse {
            id: organization.id,
            name: organization.name,
            slug: organization.slug,
            description: organization.description,
        })
        .collect())
}

#[get("/api/org/{organization_id}", headers: HeaderMap)]
pub async fn get_organization(organization_id: i32) -> Result<OrganizationResponse, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let (_token_string, session) = get_validated_session_from_headers(&headers).await?;

    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    if membership.is_none() {
        return Err(ServerFnError::new("Not a member of this organization"));
    }

    let organization = get_organization_by_id(organization_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(OrganizationResponse {
        id: organization.id,
        name: organization.name,
        slug: organization.slug,
        description: organization.description,
    })
}

#[get("/api/org/{organization_id}/members", headers: HeaderMap)]
pub async fn get_organization_members(
    organization_id: i32,
) -> Result<Vec<OrganizationMemberResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let (_token_string, session) = get_validated_session_from_headers(&headers).await?;

    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    if membership.is_none() {
        return Err(ServerFnError::new("Not a member of this organization"));
    }

    let members = list_organization_members(organization_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let mut responses = Vec::new();

    for member in members {
        if let Ok(user) = get_user_by_id(member.user_id).await {
            responses.push(OrganizationMemberResponse {
                user_id: user.id,
                organization_id: member.organization_id,
                role: member.role.clone(),
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
            });
        }
    }

    Ok(responses)
}

#[post("/api/org/{organization_id}/invite", headers: HeaderMap)]
pub async fn invite_member(
    organization_id: i32,
    request: InviteMemberRequest,
) -> Result<(), ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let (_token_string, session) = get_validated_session_from_headers(&headers).await?;

    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .ok_or_else(|| ServerFnError::new("Not a member of this organization"))?;

    if membership.role != "owner" && membership.role != "admin" {
        return Err(ServerFnError::new(
            "Only owners and admins can invite members",
        ));
    }

    let connection = &mut get_postgres_connection()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let invitation = NewInvitation::new(
        organization_id,
        request.email,
        request.role,
        session.user_id,
    );

    diesel::insert_into(invitations::table)
        .values(&invitation)
        .execute(connection)
        .await
        .map_err(|error| ServerFnError::new(format!("Failed to create invitation: {}", error)))?;

    Ok(())
}

#[post("/api/org/{organization_id}/remove-member", headers: HeaderMap)]
pub async fn remove_organization_member(
    organization_id: i32,
    member_id: i32,
) -> Result<(), ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let (_token_string, session) = get_validated_session_from_headers(&headers).await?;

    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .ok_or_else(|| ServerFnError::new("Not a member of this organization"))?;

    if membership.role != "owner" && membership.role != "admin" {
        return Err(ServerFnError::new(
            "Only owners and admins can remove members",
        ));
    }

    remove_member(member_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[post("/api/org/{organization_id}/update-role", headers: HeaderMap)]
pub async fn update_organization_member_role(
    organization_id: i32,
    member_id: i32,
    new_role: String,
) -> Result<(), ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let (_token_string, session) = get_validated_session_from_headers(&headers).await?;

    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .ok_or_else(|| ServerFnError::new("Not a member of this organization"))?;

    if membership.role != "owner" {
        return Err(ServerFnError::new("Only owners can change member roles"));
    }

    let role = MemberRole::from_str(&new_role).ok_or_else(|| ServerFnError::new("Invalid role"))?;

    update_member_role(member_id, role)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}
