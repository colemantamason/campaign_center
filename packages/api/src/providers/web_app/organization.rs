use crate::interfaces::{
    CreateOrganizationRequest, InviteMemberRequest, OrganizationMemberResponse,
    OrganizationResponse,
};
use crate::models::{MemberRole, NewInvitation};
use crate::postgres::{get_postgres_connection, initialize_postgres_pool};
use crate::redis::{
    cache_session, get_cached_session, initialize_redis_pool,
    update_cached_session_active_organization_membership_id, CachedSession,
};
use crate::schema::invitations;
use crate::services::{
    create_organization as create_organization_service, get_membership, get_organization_by_id,
    get_session_from_headers, get_user_by_id, list_organization_members, list_user_organizations,
    remove_member, set_active_organization as set_active_organization_service, update_member_role,
    validate_session,
};
use diesel_async::RunQueryDsl;
use dioxus::{fullstack::HeaderMap, prelude::*};
use uuid::Uuid;

fn initialize_databases() {
    initialize_postgres_pool().ok();
    initialize_redis_pool().ok();
}

async fn get_validated_session_from_headers(
    headers: &HeaderMap,
) -> Result<(String, crate::models::Session), ServerFnError> {
    let token_string =
        get_session_from_headers(headers).ok_or_else(|| ServerFnError::new("Not authenticated"))?;

    let token =
        Uuid::parse_str(&token_string).map_err(|_| ServerFnError::new("Invalid session token"))?;
    // try redis cache first, but we always need the full session for these operations
    if let Ok(Some(_cached)) = get_cached_session(&token_string).await {
        // we have cached data, but we need the full session for some operations
        // validate against the database to get the full session
        let session = validate_session(token)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        return Ok((token_string, session));
    }

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    // cache the session
    let cached = CachedSession {
        session_id: session.id,
        user_id: session.user_id,
        active_organization_membership_id: session.active_organization_membership_id,
    };

    cache_session(&token_string, &cached).await.ok();

    Ok((token_string, session))
}

#[post("/api/org/create", headers: HeaderMap)]
pub async fn create_organization(
    request: CreateOrganizationRequest,
) -> Result<OrganizationResponse, ServerFnError> {
    initialize_databases();

    let (token_string, session) = get_validated_session_from_headers(&headers).await?;

    // create_organization_service already adds the creator as owner
    let organization = create_organization_service(request.name, request.slug, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    // auto-set this as the active organization
    set_active_organization_service(session.id, Some(organization.id))
        .await
        .ok();

    // update the cached session with the new active org
    update_cached_session_active_organization_membership_id(&token_string, Some(organization.id))
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
    initialize_databases();

    let (token_string, session) = get_validated_session_from_headers(&headers).await?;

    // verify user is a member of the organization
    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    if membership.is_none() {
        return Err(ServerFnError::new("Not a member of this organization"));
    }

    set_active_organization_service(session.id, Some(organization_id))
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    // update the cached session
    update_cached_session_active_organization_membership_id(&token_string, Some(organization_id))
        .await
        .ok();

    Ok(())
}

#[get("/api/org/list", headers: HeaderMap)]
pub async fn get_user_organizations() -> Result<Vec<OrganizationResponse>, ServerFnError> {
    initialize_databases();

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
    initialize_databases();

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
    initialize_databases();

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
    initialize_databases();

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
    initialize_databases();

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
    initialize_databases();

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
