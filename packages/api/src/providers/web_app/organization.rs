use crate::database;
use crate::interfaces::{
    CreateOrganizationRequest, InviteMemberRequest, OrganizationMemberResponse,
    OrganizationResponse,
};
use crate::models::{MemberRole, NewInvitation};
use crate::schema::invitations;
use crate::services::{
    create_organization as create_organization_service, get_membership, get_organization_by_id,
    get_user_by_id, list_organization_members, list_user_organizations, remove_member,
    update_member_role, validate_session,
};
use diesel_async::RunQueryDsl;
use dioxus::prelude::*;
use uuid::Uuid;

#[server]
pub async fn create_organization(
    session_token: String,
    request: CreateOrganizationRequest,
) -> Result<OrganizationResponse, ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    // create_organization_service already adds the creator as owner
    let organization = create_organization_service(request.name, request.slug, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(OrganizationResponse {
        id: organization.id,
        name: organization.name,
        slug: organization.slug,
        description: organization.description,
    })
}

#[server]
pub async fn get_user_organizations(
    session_token: String,
) -> Result<Vec<OrganizationResponse>, ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let orgs = list_user_organizations(session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(orgs
        .into_iter()
        .map(|(organization, _member)| OrganizationResponse {
            id: organization.id,
            name: organization.name,
            slug: organization.slug,
            description: organization.description,
        })
        .collect())
}

#[server]
pub async fn get_organization(
    session_token: String,
    organization_id: i32,
) -> Result<OrganizationResponse, ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

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

#[server]
pub async fn get_organization_members(
    session_token: String,
    organization_id: i32,
) -> Result<Vec<OrganizationMemberResponse>, ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

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

#[server]
pub async fn invite_member(
    session_token: String,
    organization_id: i32,
    req: InviteMemberRequest,
) -> Result<(), ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .ok_or_else(|| ServerFnError::new("Not a member of this organization"))?;

    if membership.role != "owner" && membership.role != "admin" {
        return Err(ServerFnError::new(
            "Only owners and admins can invite members",
        ));
    }

    let conn = &mut database::get_connection()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let invitation = NewInvitation::new(organization_id, req.email, req.role, session.user_id);

    diesel::insert_into(invitations::table)
        .values(&invitation)
        .execute(conn)
        .await
        .map_err(|error| ServerFnError::new(format!("Failed to create invitation: {}", error)))?;

    // TODO: send invitation email

    Ok(())
}

#[server]
pub async fn remove_organization_member(
    session_token: String,
    organization_id: i32,
    member_id: i32,
) -> Result<(), ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

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

#[server]
pub async fn update_organization_member_role(
    session_token: String,
    organization_id: i32,
    member_id: i32,
    new_role: String,
) -> Result<(), ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

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
