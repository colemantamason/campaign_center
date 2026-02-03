# Organizations & Teams Feature Specification

> **Feature Area**: Organizations & Team Management  
> **Status**: Requirements In Progress  
> **Phase**: Foundation (Phase 0) + MVP (Phase 1)

---

## Table of Contents

1. [Overview](#overview)
2. [Data Models](#data-models)
3. [Organization Management](#organization-management)
4. [Team & Roles](#team--roles)
5. [Permissions System](#permissions-system)
6. [Invitations](#invitations)
7. [API Specification](#api-specification)

---

## Overview

### Purpose

Organizations are the primary unit of account in Campaign Center. Each organization:
- Manages its own events, contacts, and campaigns
- Has its own team members with role-based permissions
- Has its own billing/subscription (future)
- Can collaborate with other organizations via co-hosting

### User Roles

| Role | Description | Permissions |
|------|-------------|-------------|
| **Owner** | Organization creator/primary admin | All permissions, cannot be removed |
| **Admin** | Full administrative access | All permissions except delete org |
| **Manager** | Event and team management | Create/edit events, manage attendees |
| **Member** | Basic access | View events, limited actions |

---

## Data Models

### Organization

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub id: i32,
    
    // Basic Info
    pub name: String,                    // Required, 1-255 chars
    pub slug: String,                    // URL-safe, unique globally
    pub description: Option<String>,     // Organization description
    pub avatar_url: Option<String>,      // Logo/avatar image
    pub website_url: Option<String>,     // Organization website
    
    // Contact
    pub email: Option<String>,           // Public contact email
    pub phone: Option<String>,           // Public contact phone
    
    // Location
    pub address: Option<OrganizationAddress>,
    pub timezone: String,                // Default: "America/New_York"
    
    // Settings
    pub default_event_capacity: Option<i32>,
    pub notification_preferences: NotificationPreferences,
    
    // Billing (Stripe)
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub subscription_status: SubscriptionStatus,
    pub subscription_plan: Option<SubscriptionPlan>,
    
    // Metadata
    pub member_count: i32,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationAddress {
    pub line_1: String,
    pub line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Trial,
    Active,
    PastDue,
    Cancelled,
    Suspended,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SubscriptionPlan {
    Starter,
    Professional,
    Enterprise,
}
```

### Organization Member

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMember {
    pub id: i32,
    pub organization_id: i32,
    pub user_id: i32,
    
    // User details (denormalized for convenience)
    pub user: UserSummary,
    
    // Role & Permissions
    pub role: MemberRole,
    pub permissions: Permissions,
    
    // Metadata
    pub invited_by: Option<i32>,
    pub joined_at: DateTime<Utc>,
    pub last_active_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MemberRole {
    Owner,
    Admin,
    Manager,
    Member,
}

impl MemberRole {
    pub fn display_name(&self) -> &str {
        match self {
            MemberRole::Owner => "Owner",
            MemberRole::Admin => "Admin",
            MemberRole::Manager => "Manager",
            MemberRole::Member => "Member",
        }
    }
}
```

### Invitation

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    pub id: i32,
    pub organization_id: i32,
    pub email: String,
    pub role: MemberRole,
    pub permissions: Permissions,
    
    // Token for accepting
    pub token: String,
    
    // Status
    pub status: InvitationStatus,
    pub expires_at: DateTime<Utc>,
    
    // Metadata
    pub invited_by: i32,
    pub created_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Rejected,
    Expired,
}
```

---

## Organization Management

### Create Organization Flow

```
┌─────────────────────────────────────────────────────────────┐
│            Create Your Organization                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Organization Name *                                         │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Springfield Teachers United                          │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  URL: campaigncenter.com/o/springfield-teachers-united      │
│       [Edit slug]                                           │
│                                                              │
│  Organization Type                                          │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Nonprofit/Advocacy Group                        ▼   │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Website (optional)                                         │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ https://teachersunited.org                           │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Timezone                                                   │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ America/Chicago (Central Time)                  ▼   │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│                      [Create Organization]                   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Organization Settings

```
Tabs: [General] [Team] [Billing] [Integrations] [Danger Zone]

───────────────────────────────────────────────────────────────

General Settings:
- Organization name
- Description
- Logo upload
- Public contact info (email, phone)
- Address
- Default timezone
- Default event capacity

Team Settings:
- Member list with roles
- Pending invitations
- Invite new members
- Role management

Billing:
- Current plan
- Upgrade/downgrade
- Payment method
- Invoice history

Integrations:
- Stripe Connect (for payments)
- Webhook settings (future)

Danger Zone:
- Transfer ownership
- Delete organization
```

### Organization Switcher

Users can belong to multiple organizations. The sidebar includes an organization switcher:

```
┌───────────────────────────────────────┐
│  Teachers United               ▼      │
├───────────────────────────────────────┤
│  ✓ Teachers United                    │
│    Springfield Parents Assoc.         │
│    Education Reform Coalition         │
│  ──────────────────────────────────── │
│  + Create New Organization            │
│  ⚙ Manage Organizations               │
└───────────────────────────────────────┘
```

---

## Team & Roles

### Team Management View

```
┌─────────────────────────────────────────────────────────────┐
│  Settings > Team                                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Team Members (4)                         [Invite Member]   │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ 👤 Jane Doe                              Owner         │  │
│  │    jane@teachersunited.org                             │  │
│  │    Last active: Today at 2:45 PM                       │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │ 👤 John Smith                            Admin    [▼] │  │
│  │    john@teachersunited.org                             │  │
│  │    Last active: Yesterday                              │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │ 👤 Sarah Johnson                         Manager  [▼] │  │
│  │    sarah@example.com                                   │  │
│  │    Last active: 3 days ago                             │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │ 👤 Mike Brown                            Member   [▼] │  │
│  │    mike@example.com                                    │  │
│  │    Last active: Last week                              │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Pending Invitations (1)                                    │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ ✉ alex@example.com                       Manager       │  │
│  │   Invited 2 days ago            [Resend] [Revoke]      │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Role Dropdown Actions

| Role | Available Actions |
|------|-------------------|
| Owner | (No actions - cannot modify) |
| Admin | Change role, Remove |
| Manager | Change role, Remove |
| Member | Change role, Remove |

Only Owners and Admins can modify team members.

---

## Permissions System

### Permission Categories

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionType {
    // Events
    Events,           // View events
    EventsCreate,     // Create new events
    EventsEdit,       // Edit existing events
    EventsDelete,     // Delete events
    EventsPublish,    // Publish/unpublish events
    
    // Attendees
    Attendees,        // View attendees
    AttendeesManage,  // Check in, update status
    AttendeesExport,  // Export attendee lists
    
    // Team
    Team,             // View team members
    TeamInvite,       // Invite new members
    TeamManage,       // Change roles, remove members
    
    // Settings
    Settings,         // View settings
    SettingsEdit,     // Edit settings
    SettingsBilling,  // Manage billing
    
    // Communications
    Communications,   // View message history
    CommunicationsSend, // Send messages
    
    // Analytics
    Analytics,        // View analytics
    
    // Groups
    Groups,           // View groups
    GroupsManage,     // Create/edit groups
    
    // Voter Data (Phase 3)
    VoterData,        // View voter data
    VoterDataEdit,    // Edit voter records
    VoterDataExport,  // Export voter data
}

pub type Permissions = HashMap<PermissionType, bool>;
```

### Role Default Permissions

| Permission | Owner | Admin | Manager | Member |
|------------|:-----:|:-----:|:-------:|:------:|
| Events | ✅ | ✅ | ✅ | ✅ |
| EventsCreate | ✅ | ✅ | ✅ | ❌ |
| EventsEdit | ✅ | ✅ | ✅ | ❌ |
| EventsDelete | ✅ | ✅ | ❌ | ❌ |
| EventsPublish | ✅ | ✅ | ✅ | ❌ |
| Attendees | ✅ | ✅ | ✅ | ❌ |
| AttendeesManage | ✅ | ✅ | ✅ | ❌ |
| AttendeesExport | ✅ | ✅ | ✅ | ❌ |
| Team | ✅ | ✅ | ✅ | ✅ |
| TeamInvite | ✅ | ✅ | ❌ | ❌ |
| TeamManage | ✅ | ✅ | ❌ | ❌ |
| Settings | ✅ | ✅ | ❌ | ❌ |
| SettingsEdit | ✅ | ✅ | ❌ | ❌ |
| SettingsBilling | ✅ | ❌ | ❌ | ❌ |
| Communications | ✅ | ✅ | ✅ | ❌ |
| CommunicationsSend | ✅ | ✅ | ✅ | ❌ |
| Analytics | ✅ | ✅ | ✅ | ❌ |
| Groups | ✅ | ✅ | ✅ | ❌ |
| GroupsManage | ✅ | ✅ | ❌ | ❌ |
| VoterData | ✅ | ✅ | ✅ | ❌ |
| VoterDataEdit | ✅ | ✅ | ❌ | ❌ |
| VoterDataExport | ✅ | ✅ | ❌ | ❌ |

### Custom Permissions

Admins can override default permissions per-member:

```
┌─────────────────────────────────────────────────────────────┐
│  Edit Permissions: Sarah Johnson (Manager)                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ☑ Use default Manager permissions                          │
│                                                              │
│  Or customize:                                               │
│                                                              │
│  Events                                                      │
│    ☑ View events                                            │
│    ☑ Create events                                          │
│    ☑ Edit events                                            │
│    ☐ Delete events (override)                               │
│    ☑ Publish events                                         │
│                                                              │
│  Attendees                                                   │
│    ☑ View attendees                                         │
│    ☑ Manage attendees                                       │
│    ☑ Export attendees                                       │
│                                                              │
│  ... more categories ...                                    │
│                                                              │
│             [Cancel]  [Save Permissions]                    │
└─────────────────────────────────────────────────────────────┘
```

---

## Invitations

### Invite Flow

1. **Admin initiates invite**: Enter email + select role
2. **System sends email**: With secure invitation link
3. **Recipient clicks link**: Redirected to accept page
4. **If existing user**: Added to organization immediately
5. **If new user**: Completes registration, then added

### Invitation Email

```
Subject: You're invited to join Teachers United on Campaign Center

Hi there!

Jane Doe has invited you to join Teachers United on Campaign Center 
as a Manager.

Campaign Center is where we organize our events, manage volunteers, 
and coordinate our campaigns.

[Accept Invitation]

This invitation expires in 7 days.

If you didn't expect this invitation, you can safely ignore this email.
```

### Accept Invitation Flow

**For existing users:**
```
┌─────────────────────────────────────────────────────────────┐
│  Join Teachers United                                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  👋 Welcome back, John!                                     │
│                                                              │
│  You've been invited to join Teachers United as a Manager.  │
│                                                              │
│  Invited by: Jane Doe                                       │
│                                                              │
│             [Accept Invitation]  [Decline]                  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**For new users:**
```
┌─────────────────────────────────────────────────────────────┐
│  Join Teachers United                                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Create your account to join Teachers United                │
│                                                              │
│  First Name *          Last Name *                          │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │                  │  │                  │                │
│  └──────────────────┘  └──────────────────┘                │
│                                                              │
│  Email (pre-filled)                                         │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ invitee@example.com                                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  Password *                                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ ••••••••••••                                         │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│               [Create Account & Join]                       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## API Specification

### Organization Endpoints

```rust
// Create organization
#[server]
pub async fn create_organization(req: CreateOrganizationRequest) -> Result<Organization, ServerFnError>;

// Get organization by ID
#[server]
pub async fn get_organization(org_id: i32) -> Result<Organization, ServerFnError>;

// Get organization by slug (public profile)
#[server]
pub async fn get_organization_by_slug(slug: String) -> Result<Organization, ServerFnError>;

// List user's organizations
#[server]
pub async fn list_user_organizations() -> Result<Vec<OrganizationMembership>, ServerFnError>;

// Update organization
#[server]
pub async fn update_organization(org_id: i32, req: UpdateOrganizationRequest) -> Result<Organization, ServerFnError>;

// Delete organization (owner only)
#[server]
pub async fn delete_organization(org_id: i32) -> Result<(), ServerFnError>;

// Transfer ownership
#[server]
pub async fn transfer_ownership(org_id: i32, new_owner_id: i32) -> Result<(), ServerFnError>;
```

### Team Endpoints

```rust
// List organization members
#[server]
pub async fn list_organization_members(org_id: i32) -> Result<Vec<OrganizationMember>, ServerFnError>;

// Update member role
#[server]
pub async fn update_member_role(member_id: i32, role: MemberRole) -> Result<OrganizationMember, ServerFnError>;

// Update member permissions
#[server]
pub async fn update_member_permissions(
    member_id: i32,
    permissions: Permissions,
) -> Result<OrganizationMember, ServerFnError>;

// Remove member
#[server]
pub async fn remove_member(member_id: i32) -> Result<(), ServerFnError>;
```

### Invitation Endpoints

```rust
// Send invitation
#[server]
pub async fn send_invitation(req: SendInvitationRequest) -> Result<Invitation, ServerFnError>;

// List pending invitations
#[server]
pub async fn list_invitations(org_id: i32) -> Result<Vec<Invitation>, ServerFnError>;

// Resend invitation
#[server]
pub async fn resend_invitation(invitation_id: i32) -> Result<Invitation, ServerFnError>;

// Revoke invitation
#[server]
pub async fn revoke_invitation(invitation_id: i32) -> Result<(), ServerFnError>;

// Accept invitation (by token)
#[server]
pub async fn accept_invitation(token: String) -> Result<OrganizationMember, ServerFnError>;

// Reject invitation
#[server]
pub async fn reject_invitation(token: String) -> Result<(), ServerFnError>;
```

### Request Types

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub slug: Option<String>,  // Auto-generated if not provided
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<OrganizationAddress>,
    pub timezone: Option<String>,
    pub default_event_capacity: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SendInvitationRequest {
    pub email: String,
    pub role: MemberRole,
    pub custom_permissions: Option<Permissions>,
    pub message: Option<String>,  // Personal note in email
}
```

---

## Related Documentation

- [AGENTS.md](../../AGENTS.md) - AI coding context
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Technical architecture
- [EVENTS.md](EVENTS.md) - Events feature spec
- [COMMUNICATIONS.md](COMMUNICATIONS.md) - Communications feature
