# Campaign Center - AI Agent Context

Purpose: This document provides comprehensive context for AI coding assistants working on this political campaign platform built with Dioxus (Rust) and Tailwind CSS.

---

## Project Overview

Campaign Center will be a full-stack political campaign management platform that includes:

- Web App (`packages/web_app`): Primary SaaS application for campaign/organization management
- Events Website (`packages/events`): Public-facing event discovery platform
- Marketing Website (`packages/marketing`): Landing pages and marketing content
- Blog Website (`packages/blog`): Public-facing blog for articles and updates
- Support Website (`packages/support`): Help center and Intercom-style chat widget
- Surveys Website (`packages/surveys`): Public-facing survey/polling response platform for voters
- CMS App (`packages/cms`): Internal content management for support articles and blog posts
- Mobile App (`packages/mobile_app`): Native app for notifications and fieldwork (post-Dioxus 1.0)

Our codebase also contains hand-made websites (`packages/websites/*`) - but it will only be included temporarily.

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
│   │       ├── lib.rs         # Module exports, feature gates, initialize_databases()
│   │       ├── error.rs       # Error types
│   │       ├── postgres.rs    # PostgreSQL connection pool (server-only)
│   │       ├── redis.rs       # Redis session cache pool (server-only)
│   │       ├── schema.rs      # Diesel schema (auto-generated, server-only)
│   │       ├── enums.rs       # Enum module exports
│   │       ├── enums/         # Project enums
│   │       │   └── shared/, web_app/, events/, mobile_app/, support/, surveys/, blog/, cms/
│   │       ├── http.rs        # HTTP module exports
│   │       ├── http/          # HTTP utilities (server-only)
│   │       │   └── token.rs   # Session token handling, platform-aware auth
│   │       ├── interfaces.rs  # Interface module exports
│   │       ├── interfaces/    # DTOs for API requests/responses
│   │       │   └── shared/, web_app/, events/, mobile_app/, support/, surveys/, blog/, cms/
│   │       ├── models.rs      # Model module exports
│   │       ├── models/        # Diesel ORM models (server-only)
│   │       │   ├── event.rs, invitation.rs, notification.rs
│   │       │   ├── organization.rs, organization_member.rs
│   │       │   └── session.rs, user.rs
│   │       ├── providers.rs   # Provider module exports
│   │       ├── providers/     # Dioxus #[server] functions
│   │       │   └── shared/, web_app/, events/, mobile_app/, support/, surveys/, blog/, cms/
│   │       ├── services.rs    # Service module exports
│   │       ├── services/      # Business logic (server-only)
│   │       │   └── shared/, web_app/, events/, mobile_app/, support/, surveys/, blog/, cms/
│   │       ├── state.rs       # State module exports
│   │       └── state/         # Client-side state types with #[derive(Store)]
│   │           └── shared/, web_app/, events/, mobile_app/, support/, surveys/, blog/, cms/
│   ├── blog/                  # Public blog website (scaffold)
│   │   └── src/
│   │       └── lib.rs
│   ├── cms/                   # Content management system (scaffold)
│   │   └── src/
│   │       └── lib.rs
│   ├── events/                # Event discovery platform (scaffold)
│   │   └── src/
│   │       └── lib.rs
│   ├── marketing/             # Marketing website (scaffold)
│   │   └── src/
│   │       └── lib.rs
│   ├── mobile_app/            # Mobile application (scaffold)
│   │   └── src/
│   │       └── lib.rs
│   ├── support/               # Help center (scaffold)
│   │   └── src/
│   │       └── lib.rs
│   ├── surveys/               # Online survey platform (scaffold)
│   │   └── src/
│   │       └── lib.rs
│   ├── tooling/               # Build tools (CSS processing)
│   │   └── src/
│   │       └── main.rs
│   ├── ui/                    # Shared UI components
│   │   └── src/
│   │       ├── lib.rs         # Feature-gated module exports
│   │       ├── shared.rs      # Shared module exports (WIP)
│   │       ├── shared/        # Cross-project shared components (WIP)
│   │       ├── web_app.rs     # Web app module exports (WIP)
│   │       ├── web_app/       # Web app components (WIP)
│   │       ├── blog.rs        # Blog website module exports (scaffold)
│   │       ├── blog/          # Blog website components (scaffold)
│   │       ├── cms.rs         # CMS app module exports (scaffold)
│   │       ├── cms/           # CMS app components (scaffold)
│   │       ├── events.rs      # Events app module exports (scaffold)
│   │       ├── events/        # Events app components (scaffold)
│   │       ├── marketing.rs   # Marketing website module exports (scaffold)
│   │       ├── marketing/     # Marketing website components (scaffold)
│   │       ├── mobile_app.rs  # Mobile app module exports (scaffold)
│   │       ├── mobile_app/    # Mobile app components (scaffold)
│   │       ├── support.rs     # Support app module exports (scaffold)
│   │       ├── support/       # Support app components (scaffold)
│   │       ├── surveys.rs     # Surveys app module exports (scaffold)
│   │       └── surveys/       # Surveys app components (scaffold)
│   ├── web_app/               # Main SaaS application (WIP)
│   │   ├── Dioxus.toml        # Dioxus configuration
│   │   ├── assets/style.css   # Compiled Tailwind CSS
│   │   └── src/
│   │       ├── main.rs        # App entry point
│   │       ├── auth.rs        # Auth state management
│   │       ├── gate.rs        # Permission-based route guards
│   │       ├── routes.rs      # Route definitions + Layout component
│   │       └── routes/        # Route page components
│   └── websites/              # Temporary hand-made websites
├── docs/                      # Documentation
│   ├── AGENTS.md              # This file
│   ├── INFRASTRUCTURE.md      # Hosting infrastructure plan
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
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ Interfaces  │     │    Enums    │     │   Schema    │
│   (DTOs)    │     │  (shared)   │     │  (tables)   │
└─────────────┘     └─────────────┘     └─────────────┘
       │
       ▼
┌─────────────┐
│    HTTP     │
│  (tokens)   │
└─────────────┘
```

- enums/ - Shared enums used across providers, services, models, and interfaces
- http/ - HTTP utilities (session token handling, platform-aware)
- interfaces/ - Request/Response DTOs for API communication
- models/ - Diesel ORM models (database rows) and relevant Enums
- providers/ - Dioxus `#[server]` functions (API endpoints)
- services/ - Business logic and database operations
- state/ - Client-side state types with stores/signals

### Session & Cookie Handling

Session tokens are delivered securely:
- **Web browsers**: HttpOnly `Set-Cookie` header (not accessible to JavaScript)
- **Mobile apps**: `X-Session-Token` response header (stored in secure native storage)
- **Security**: Session tokens are NOT included in JSON response bodies (prevents XSS token theft)
- **Cookie configuration**: Supports subdomain sharing via `COOKIE_DOMAIN` environment variable.

**Platform Tracking**: Sessions include a `platform` field (`Platform` enum: `Web`, `Mobile`) that:
- Enables "Active Sessions" UI to show device types clearly
- Allows revoking all sessions by platform (e.g., "Sign out all mobile devices")
- Supports platform-specific session policies if needed (e.g., different expiry times)
- Each app passes its platform when creating sessions (`Platform::Web` for web_app, `Platform::Mobile` for mobile_app)

**Device Detection**: The `DeviceInfo` struct parses `user_agent` to detect iOS/Android and specific devices:
- `Session::device_info()` returns parsed device details (OS, version, browser, device type)
- `Session::device_display()` returns human-readable string like "iPhone (iOS 17.2)" or "Chrome on macOS"

**Shared Auth Pattern**: Auth providers live in `api/providers/shared/auth.rs` and are used by both web_app and mobile_app. The only difference between platforms is:
- Token delivery mechanism (cookie for web, `X-Session-Token` header for mobile)
- Platform value passed in request payloads (determines which token mechanism to use)

### Database Tables (Diesel Schema)

Current tables defined in `schema.rs`:

| Table | Description |
|-------|-------------|
| `users` | User accounts with auth info |
| `sessions` | Session tokens linked to users, active org membership, and platform (web/mobile) |
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
blog = []
cms = []
events = []
mobile_app = []
server = [
    "dioxus/server", 
    # ... more server deps,
]
support = []
surveys = []
web_app = []

# ui/Cargo.toml  
[features]
blog = []
cms = []
events = []
marketing = []
mobile_app = []
server = ["api/server"]
support = []
surveys = []
web = [
    "dioxus/web", 
    # ... more web deps,
]
web_app = []
```

This ensures each app only includes relevant code for smaller bundles. Make sure to add code to relevant gated sub-folders.

---

## Coding Conventions

### Execution Guidelines
ALWAYS: Feel free to ask questions before proceeding with implementing changes.
ALWAYS: Update this document when relevant lessons are learned or structural changes are made that require an update
ALWAYS: Ignore TODO comments unless explicitly asked to address them
ALWAYS: Add comments in lowercase
NEVER: Add comments when the code is easily readable

### Rust/Dioxus Guidelines
ALWAYS: Gate web-specific code with `#[cfg(feature = "web")]`
ALWAYS: Gate server-specific code with `#[cfg(feature = "server")]`
ALWAYS: Check existing patterns in similar components before implementing new ones
ALWAYS: Use exhaustive match whenever working with enums
ALWAYS: Make sure props are external to the component function
ALWAYS: Optional props use `Option<T>`, not default values
ALWAYS: Use the full naming of variables and functions instead of shorthand (i.e. |string| instead of |s| in a closure)

### CSS/Tailwind Guidelines

ALWAYS: Use flexbox for layout
AVOID: Complex positioning, floats, or cutting-edge CSS
ALWAYS: Style Lucide icons with class only
NEVER: Use inline style or other props for Lucide icon sizing
- INSTEAD: Use Tailwind class styling
NEVER: use eval() for JavaScript
- INSTEAD: Use wasm-bindgen/web-sys/gloo or other Rust WASM features if possible

---