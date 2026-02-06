use crate::enums::MemberRole;
use crate::http::AuthSession;
use crate::interfaces::{
    CreateOrganizationRequest, InviteMemberRequest, OrganizationMemberResponse,
    OrganizationResponse,
};
#[cfg(feature = "server")]
use crate::models::NewInvitation;
#[cfg(feature = "server")]
use crate::postgres::get_postgres_connection;
#[cfg(feature = "server")]
use crate::redis::update_cached_session_active_organization_membership_id;
#[cfg(feature = "server")]
use crate::schema::invitations;
#[cfg(feature = "server")]
use crate::services::{
    create_organization as create_organization_service, get_membership, get_organization_by_id,
    get_user_by_id, list_organization_members, list_user_organizations, remove_member,
    set_active_organization as set_active_organization_service, update_member_role,
};
#[cfg(feature = "server")]
use diesel_async::RunQueryDsl;
use dioxus::prelude::*;

#[post("/api/org/create", auth: AuthSession)]
pub async fn create_organization(
    request: CreateOrganizationRequest,
) -> Result<OrganizationResponse, ServerFnError> {
    let session = auth.require_auth()?;

    let (organization, membership) =
        create_organization_service(request.name, request.slug, request.organization_type, session.user_id)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

    set_active_organization_service(session.session_id, Some(membership.id))
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    update_cached_session_active_organization_membership_id(&session.token, Some(membership.id))
        .await
        .ok();

    Ok(OrganizationResponse {
        id: organization.id,
        name: organization.name,
        slug: organization.slug,
        description: organization.description,
    })
}

#[post("/api/org/set-active", auth: AuthSession)]
pub async fn set_active_organization(organization_id: i32) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;

    let membership = get_membership(organization_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .ok_or_else(|| ServerFnError::new("Not a member of this organization"))?;

    set_active_organization_service(session.session_id, Some(membership.id))
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    update_cached_session_active_organization_membership_id(&session.token, Some(membership.id))
        .await
        .ok();

    Ok(())
}

#[get("/api/org/list", auth: AuthSession)]
pub async fn get_user_organizations() -> Result<Vec<OrganizationResponse>, ServerFnError> {
    let session = auth.require_auth()?;

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

#[get("/api/org/{organization_id}", auth: AuthSession)]
pub async fn get_organization(organization_id: i32) -> Result<OrganizationResponse, ServerFnError> {
    let session = auth.require_auth()?;

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

#[get("/api/org/{organization_id}/members", auth: AuthSession)]
pub async fn get_organization_members(
    organization_id: i32,
) -> Result<Vec<OrganizationMemberResponse>, ServerFnError> {
    let session = auth.require_auth()?;

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

#[post("/api/org/{organization_id}/invite", auth: AuthSession)]
pub async fn invite_member(
    organization_id: i32,
    request: InviteMemberRequest,
) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;

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

#[post("/api/org/{organization_id}/remove-member", auth: AuthSession)]
pub async fn remove_organization_member(
    organization_id: i32,
    member_id: i32,
) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;

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

#[post("/api/org/{organization_id}/update-role", auth: AuthSession)]
pub async fn update_organization_member_role(
    organization_id: i32,
    member_id: i32,
    new_role: String,
) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;

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
