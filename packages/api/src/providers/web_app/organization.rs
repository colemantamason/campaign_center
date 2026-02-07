use crate::enums::MemberRole;
use crate::http::AuthSession;
use crate::interfaces::{
    CreateOrganizationRequest, InviteMemberRequest, OrganizationMemberResponse,
    OrganizationResponse, UpdateOrganizationRequest,
};
use crate::models::OrganizationMember;
#[cfg(feature = "server")]
use crate::redis::update_redis_cached_session_active_organization_membership_id;
#[cfg(feature = "server")]
use crate::services::{
    create_invitation, create_organization as create_organization_service, get_member_by_id,
    get_members_with_user_info, get_membership, get_organization_by_id, list_user_organizations,
    remove_member, set_active_organization as set_active_organization_service, update_member_role,
    update_organization as update_organization_service,
};
use dioxus::prelude::*;

#[cfg(feature = "server")]
async fn require_membership(
    organization_id: i32,
    user_id: i32,
) -> Result<OrganizationMember, ServerFnError> {
    get_membership(organization_id, user_id)
        .await?
        .ok_or_else(|| ServerFnError::new("Not a member of this organization"))
}

#[cfg(feature = "server")]
async fn require_membership_with_role(
    organization_id: i32,
    user_id: i32,
    min_role: MemberRole,
) -> Result<OrganizationMember, ServerFnError> {
    let membership = require_membership(organization_id, user_id).await?;
    let caller_role = membership.get_role();
    let allowed = match min_role {
        MemberRole::Owner => caller_role == MemberRole::Owner,
        MemberRole::Admin => caller_role == MemberRole::Owner || caller_role == MemberRole::Admin,
        MemberRole::Manager => {
            caller_role == MemberRole::Owner
                || caller_role == MemberRole::Admin
                || caller_role == MemberRole::Manager
        }
        MemberRole::Member => true,
    };
    if !allowed {
        return Err(ServerFnError::new(format!(
            "Requires {} role or higher",
            min_role.display_name()
        )));
    }
    Ok(membership)
}

#[post("/api/org/create", auth: AuthSession)]
pub async fn create_organization(
    request: CreateOrganizationRequest,
) -> Result<OrganizationResponse, ServerFnError> {
    let session = auth.require_auth()?;

    let (organization, membership) = create_organization_service(
        request.name,
        request.organization_type,
        request.description,
        request.slug,
        session.user_id,
    )
    .await?;

    set_active_organization_service(session.session_id, Some(membership.id)).await?;

    update_redis_cached_session_active_organization_membership_id(
        &session.token,
        Some(membership.id),
    )
    .await
    .ok();

    Ok(OrganizationResponse::from(organization))
}

#[post("/api/org/set-active", auth: AuthSession)]
pub async fn set_active_organization(organization_id: i32) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;
    let membership = require_membership(organization_id, session.user_id).await?;

    set_active_organization_service(session.session_id, Some(membership.id)).await?;

    update_redis_cached_session_active_organization_membership_id(
        &session.token,
        Some(membership.id),
    )
    .await
    .ok();

    Ok(())
}

#[get("/api/org/list", auth: AuthSession)]
pub async fn get_user_organizations() -> Result<Vec<OrganizationResponse>, ServerFnError> {
    let session = auth.require_auth()?;

    let organizations = list_user_organizations(session.user_id).await?;

    Ok(organizations
        .into_iter()
        .map(|(organization, _member)| OrganizationResponse::from(organization))
        .collect())
}

#[get("/api/org/{organization_id}", auth: AuthSession)]
pub async fn get_organization(organization_id: i32) -> Result<OrganizationResponse, ServerFnError> {
    let session = auth.require_auth()?;
    require_membership(organization_id, session.user_id).await?;

    let organization = get_organization_by_id(organization_id).await?;

    Ok(OrganizationResponse::from(organization))
}

#[post("/api/org/{organization_id}/update", auth: AuthSession)]
pub async fn update_organization(
    organization_id: i32,
    request: UpdateOrganizationRequest,
) -> Result<OrganizationResponse, ServerFnError> {
    let session = auth.require_auth()?;
    require_membership_with_role(organization_id, session.user_id, MemberRole::Admin).await?;

    let update = crate::models::OrganizationUpdate {
        name: request.name,
        description: request.description,
        avatar_url: request.avatar_url,
        website_url: request.website_url,
        email: request.email,
        phone_number: request.phone_number,
        address_line_1: request.address_line_1,
        address_line_2: request.address_line_2,
        city: request.city,
        state: request.state,
        zip_code: request.zip_code,
        country: request.country,
        timezone: request.timezone,
        ..Default::default()
    };

    let organization = update_organization_service(organization_id, update).await?;

    Ok(OrganizationResponse::from(organization))
}

#[get("/api/org/{organization_id}/members", auth: AuthSession)]
pub async fn get_organization_members(
    organization_id: i32,
) -> Result<Vec<OrganizationMemberResponse>, ServerFnError> {
    let session = auth.require_auth()?;
    require_membership(organization_id, session.user_id).await?;

    let members_with_info = get_members_with_user_info(organization_id).await?;

    Ok(members_with_info
        .into_iter()
        .map(|info| OrganizationMemberResponse {
            user_id: info.member.user_id,
            organization_id: info.member.organization_id,
            role: info.member.role,
            email: info.email,
            first_name: info.first_name,
            last_name: info.last_name,
        })
        .collect())
}

#[post("/api/org/{organization_id}/invite", auth: AuthSession)]
pub async fn invite_member(
    organization_id: i32,
    request: InviteMemberRequest,
) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;
    require_membership_with_role(organization_id, session.user_id, MemberRole::Admin).await?;

    let role =
        MemberRole::from_str(&request.role).ok_or_else(|| ServerFnError::new("Invalid role"))?;

    if role == MemberRole::Owner {
        return Err(ServerFnError::new("Cannot invite as owner"));
    }

    create_invitation(organization_id, request.email, role, session.user_id).await?;

    Ok(())
}

#[post("/api/org/{organization_id}/remove-member", auth: AuthSession)]
pub async fn remove_organization_member(
    organization_id: i32,
    member_id: i32,
) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;
    require_membership_with_role(organization_id, session.user_id, MemberRole::Admin).await?;

    let target_member = get_member_by_id(member_id).await?;

    if target_member.organization_id != organization_id {
        return Err(ServerFnError::new("Member not found in this organization"));
    }

    remove_member(target_member.id).await?;

    Ok(())
}

#[post("/api/org/{organization_id}/update-role", auth: AuthSession)]
pub async fn update_organization_member_role(
    organization_id: i32,
    member_id: i32,
    new_role: String,
) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;
    require_membership_with_role(organization_id, session.user_id, MemberRole::Owner).await?;

    let target_member = get_member_by_id(member_id).await?;

    if target_member.organization_id != organization_id {
        return Err(ServerFnError::new("Member not found in this organization"));
    }

    let role = MemberRole::from_str(&new_role).ok_or_else(|| ServerFnError::new("Invalid role"))?;

    update_member_role(target_member.id, role).await?;

    Ok(())
}
