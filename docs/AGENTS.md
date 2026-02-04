# Campaign Center - AI Agent Context

Purpose: This document provides comprehensive context for AI coding assistants working on this political campaign platform built with Dioxus (Rust) and Tailwind CSS.

---

## Project Overview

Campaign Center will be a full-stack political campaign management platform that includes:

- Web App (`packages/web_app`): Primary SaaS application for campaign/organization management
- Events Website (`packages/websites/events`): Public-facing event discovery platform (like Mobilize.us)
- Marketing Website (`packages/websites/marketing`): Landing pages and marketing content
- Support Website (`packages/websites/support`): Help center and Intercom-style chat widget
- Mobile App (`packages/mobile_app`): Native app for notifications and fieldwork (post-Dioxus 1.0)

Our codebase also contains External Websites (`packages/websites/external/*`) - but it will only be included temporarily.

### Related Documentation

- [ROADMAP.md](docs/ROADMAP.md) - Development timeline and feature planning

### Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Dioxus 0.7.x (CSR + SSR fullstack) |
| Styling | Tailwind CSS v3 |
| Backend | Axum (via Dioxus server functions) |
| Database | PostgreSQL with Diesel ORM + diesel-async |
| Sessions | Redis (via deadpool-redis) |
| Migrations | diesel_cli |
| Auth | Email/password with Argon2 + session tokens |
| Email | AWS SES (dev-only for now) |
| SMS | Twilio (dev-only for now) |
| Payments | Stripe (dev-only for now) |
| File Storage | MinIO (self-hosted S3-compatible) |
| Icons | lucide-dioxus |
| Hosting | Self-managed VPSs (eventual bare-metal) |

---

## Project Structure

```
campaign_center/
├── packages/
│   ├── api/                   # Backend API layer
│   │   └── src/
│   │       ├── error.rs       # Error types
│   │       ├── lib.rs         # Module exports with feature gates
│   │       ├── postgres.rs    # PostgreSQL connection pool (server-only)
│   │       ├── redis.rs       # Redis session cache pool (server-only)
│   │       ├── schema.rs      # Diesel schema (auto-generated, server-only)
│   │       │
│   │       ├── http/          # HTTP utilities (server-only)
│   │       │   ├── mod.rs
│   │       │   └── cookie.rs  # Cookie handling, session tokens, WithCookie wrapper
│   │       │
│   │       ├── interfaces/    # DTOs for API requests/responses
│   │       │   ├── shared/    # Shared DTOs
│   │       │   ├── web_app/   # auth.rs, organization.rs
│   │       │   ├── events/    # (empty - ready for events DTOs)
│   │       │   ├── mobile_app/
│   │       │   └── support/
│   │       │
│   │       ├── models/        # Diesel ORM models & relevant Enums (server-only)
│   │       │   ├── event.rs
│   │       │   ├── invitation.rs
│   │       │   ├── notification.rs
│   │       │   ├── organization.rs
│   │       │   ├── organization_member.rs
│   │       │   ├── session.rs
│   │       │   └── user.rs
│   │       │
│   │       ├── providers/     # Dioxus #[server] functions
│   │       │   ├── shared/    # (empty)
│   │       │   ├── web_app/   # auth.rs, organization.rs
│   │       │   ├── events/
│   │       │   ├── mobile_app/
│   │       │   └── support/
│   │       │
│   │       ├── services/      # Business logic (server-only)
│   │       │   ├── shared/    # auth.rs, organization.rs, session.rs, user.rs
│   │       │   ├── web_app/   # (empty)
│   │       │   ├── events/
│   │       │   ├── mobile_app/
│   │       │   └── support/
│   │       │
│   │       └── state/         # Client-side state types with #[derive(Store)]
│   │           ├── shared/    # organization.rs (OrganizationMembership, Permissions)
│   │           ├── web_app/   # user.rs, notification.rs, event.rs
│   │           ├── events/
│   │           ├── mobile_app/
│   │           └── support/
│   │
│   ├── ui/                    # Shared UI components
│   │   └── src/
│   │       ├── lib.rs         # Feature-gated module exports
│   │       ├── shared/        # Cross-app shared components
│   │       │   ├── button.rs
│   │       │   ├── checkbox.rs
│   │       │   ├── divider.rs
│   │       │   ├── icon.rs
│   │       │   ├── form/      # sms_opt_in.rs
│   │       │   └── input/     # masked_input.rs, unmasked_input.rs
│   │       │
│   │       ├── web_app/       # Web app components
│   │       │   ├── avatar.rs
│   │       │   ├── confirmation_modal.rs
│   │       │   ├── notification_badge.rs
│   │       │   ├── toast.rs
│   │       │   └── sidebar/   # nav_button.rs, nav_label.rs, sidebar_menu/
│   │       │
│   │       ├── events/        # (empty - ready for events components)
│   │       ├── marketing/     # (empty - ready for marketing components)
│   │       ├── mobile_app/    # (empty - future)
│   │       └── support/       # (empty - ready for support components)
│   │
│   ├── web_app/               # Main SaaS application
│   │   ├── Dioxus.toml        # Dioxus configuration
│   │   ├── assets/style.css   # Compiled Tailwind CSS
│   │   └── src/
│   │       ├── main.rs        # App entry point
│   │       ├── routes.rs      # Route definitions + Layout component
│   │       ├── gate.rs        # Permission-based route guards
│   │       └── routes/        # Route page components
│   │           ├── login.rs
│   │           ├── dashboard.rs
│   │           ├── events.rs
│   │           ├── actions.rs
│   │           ├── groups.rs
│   │           ├── analytics.rs
│   │           ├── exports.rs
│   │           ├── team.rs
│   │           ├── settings.rs
│   │           └── account/   # devices.rs, notifications.rs, organizations.rs
│   │
│   ├── mobile_app/            # Mobile application (future post-Dioxus 1.0)
│   ├── tooling/               # Build tools (CSS processing)
│   └── websites/              # Public-facing websites
│       ├── events/
│       ├── marketing/
│       ├── support/
│       └── external/          # Temporary external projects
│
├── docs/                      # Documentation
│   ├── AGENTS.md              # This file
│   └── ROADMAP.md             # Development roadmap
├── migrations/                # Diesel database migrations
├── tailwind/                  # Tailwind configuration
├── Cargo.toml                 # Workspace configuration
├── compose.yml                # Docker compose for dev environment
```

### API Package Architecture

The `api` package follows a layered architecture:

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Providers  │────▶│  Services   │────▶│   Models    │
│ (endpoints) │     │ (business)  │     │ (database)  │
└─────────────┘     └─────────────┘     └─────────────┘
       │                                       │
       ▼                                       ▼
┌─────────────┐                         ┌─────────────┐
│ Interfaces  │                         │   Schema    │
│   (DTOs)    │                         │  (tables)   │
└─────────────┘                         └─────────────┘
       │
       ▼
┌─────────────┐
│    HTTP     │
│  (cookies)  │
└─────────────┘
```

- http/ - HTTP utilities (cookie handling, session tokens, `WithCookie` wrapper)
- interfaces/ - Request/Response DTOs for API communication
- models/ - Diesel ORM models (database rows) and relevant Enums
- providers/ - Dioxus `#[server]` functions (API endpoints)
- services/ - Business logic and database operations
- state/ - Client-side state types with `#[derive(Store)]`

### Session & Cookie Handling

Session tokens are delivered securely:
- **Web browsers**: HttpOnly `Set-Cookie` header (not accessible to JavaScript)
- **Mobile apps**: `X-Session-Token` response header (stored in secure native storage)
- **Security**: Session tokens are NOT included in JSON response bodies (prevents XSS token theft)

Cookie configuration supports subdomain sharing via `COOKIE_DOMAIN` environment variable.

### Database Tables (Diesel Schema)

Current tables defined in `schema.rs`:

| Table | Description |
|-------|-------------|
| `users` | User accounts with auth info |
| `sessions` | Session tokens linked to users and active org membership |
| `organizations` | Organizations/campaigns |
| `organization_members` | User-to-org membership with roles |
| `invitations` | Team member invitation tokens |
| `events` | Campaign events |
| `event_shifts` | Time slots for events |
| `event_signups` | User RSVPs to event shifts |
| `notifications` | User notification records |
| `password_reset_tokens` | Password reset flow tokens |

### Web App Routes

Current routes defined in `web_app/src/routes.rs`:

| Route | Component | Description |
|-------|-----------|-------------|
| `/login` | Login | Authentication page |
| `/` | Dashboard | Main dashboard |
| `/events` | Events | Event management |
| `/actions` | Actions | Action pages |
| `/groups` | Groups | Contact groups |
| `/analytics` | Analytics | Analytics dashboard |
| `/exports` | Exports | Data exports |
| `/team` | Team | Team management |
| `/settings` | Settings | Organization settings |
| `/account` | Account | User account settings |
| `/account/devices` | DeviceSessions | Active sessions management |
| `/account/notifications` | NotificationPreferences | Notification settings |
| `/account/organizations` | OrganizationManagement | Manage orgs |

### State Management Pattern

Client-side state uses Dioxus `Store` derive macro:

```rust
#[derive(Store)]
pub struct ExampleUserAccount {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    // ... more UserAccount data
}

// provided via global context
use_context_provider(|| ExampleUserAccountContext {
    user_account: Store::new(example_function_to_get_db_data()),
});
```

### Permission Gate Pattern

The Web Application has Route protection using the `Gate` component:

```rust
// usage at the top of a route component - other checks like auth are performed as well
Gate {
    required_permission: Some(SubscriptionType::Events),
    permission_fallback_route: Some("/dashboard".to_string()),
    // children rendered only if permission check passes
}
```


### Feature Flags Architecture

The project uses Cargo features in our ui & api packages to gate code per-application:

```toml
# api/Cargo.toml
[features]
events = []
mobile_app = []
server = [
    "dioxus/server", 
    # ... more server deps,
]
support = []
web_app = []

# ui/Cargo.toml  
[features]
events = []
marketing = []
mobile_app = []
server = ["api/server"]
support = []
web = [
    "dioxus/web", 
    # ... more web deps,
]
web_app = []
```

This ensures each app only includes relevant code for smaller bundles. Make sure to add code to relevant gated sub-folders.

---

## Coding Conventions

### Rust/Dioxus Guidelines
ALWAYS: Update this document or the Roadmap when relevant lessons are learned or structural changes are made that require an update
ALWAYS: Ignore TODO comments unless explicitly asked to address them
ALWAYS: Add comments in lowercase
NEVER: Add comments when the code is easily readable
ALWAYS: Gate web-specific code with `#[cfg(feature = "web")]`
ALWAYS: Gate server-specific code with `#[cfg(feature = "server")]`
ALWAYS: Check existing patterns in similar components before implementing new ones
ALWAYS: Use exhaustive match whenever working with enums
ALWAYS: Make sure props are external to the component function
ALWAYS: Optional props use `Option<T>`, not default values

### CSS/Tailwind Guidelines

ALWAYS: Use flexbox for layout
AVOID: Complex positioning, floats, or cutting-edge CSS
ALWAYS: Style Lucide icons with class only
NEVER: Use inline style or other props for Lucide icon sizing
Pattern: common_classes + size_classes + variant_classes + custom_classes
NEVER use eval() for JavaScript
INSTEAD: Use wasm-bindgen/web-sys/gloo or other Rust WASM features if possible

---