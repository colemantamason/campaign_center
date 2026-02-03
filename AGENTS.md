# Campaign Center - AI Agent Context

> **Purpose**: This document provides comprehensive context for AI coding assistants working on this political campaign platform built with Dioxus (Rust) and Tailwind CSS.

---

## Project Overview

**Campaign Center** is a full-stack political campaign management platform that includes:

- **Web App** (`packages/web_app`): Primary SaaS application for campaign/organization management
- **Events Website** (`packages/websites/events`): Public-facing event discovery platform (like Mobilize.us)
- **Marketing Website** (`packages/websites/marketing`): Landing pages and marketing content
- **Support Website** (`packages/websites/support`): Help center and Intercom-style chat widget
- **Mobile App** (`packages/mobile_app`): Native app for notifications and fieldwork (post-Dioxus 1.0)
- **External Websites** (`packages/websites/external/*`): Static sites for manual campaign websites

### Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Dioxus 0.7.x (CSR + SSR fullstack) |
| Styling | Tailwind CSS v3 |
| Backend | Axum (via Dioxus server functions) |
| Database | PostgreSQL with Diesel ORM |
| Migrations | diesel_cli |
| Auth | Email/password with session tokens |
| Email | AWS SES |
| SMS | Twilio |
| Payments | Stripe |
| File Storage | MinIO (self-hosted S3-compatible) |
| Icons | lucide-dioxus |
| Hosting | Self-managed VPS (eventual bare-metal) |

---

## Project Structure

```
campaign_center/
├── packages/
│   ├── api/           # Backend API layer
│   │   └── src/
│   │       ├── database.rs    # Database connection pool
│   │       ├── error.rs       # Error types
│   │       ├── schema.rs      # Diesel schema (auto-generated)
│   │       ├── interfaces/    # DTOs for API requests/responses
│   │       │   ├── shared/    # Shared DTOs across apps
│   │       │   ├── web_app/   # Web app DTOs
│   │       │   ├── events/    # Events feature DTOs
│   │       │   ├── mobile_app/# Mobile app DTOs
│   │       │   └── support/   # Support feature DTOs
│   │       ├── models/        # Database models (Diesel ORM)
│   │       │   ├── user.rs
│   │       │   ├── organization.rs
│   │       │   ├── session.rs
│   │       │   ├── organization_member.rs
│   │       │   └── invitation.rs
│   │       ├── providers/     # Dioxus server functions
│   │       │   ├── shared/    # Shared server functions
│   │       │   ├── web_app/   # Web app endpoints
│   │       │   ├── events/    # Events feature endpoints
│   │       │   ├── mobile_app/# Mobile app endpoints
│   │       │   └── support/   # Support feature endpoints
│   │       ├── services/      # Business logic
│   │       │   ├── shared/    # Core services (auth, user, session, org)
│   │       │   ├── web_app/   # Web app specific services
│   │       │   ├── events/    # Events feature services
│   │       │   ├── mobile_app/# Mobile app services
│   │       │   └── support/   # Support feature services
│   │       └── state/         # Client-side state types
│   │           ├── shared/    # Shared state (roles, organization)
│   │           ├── web_app/   # Web app state (user, notification, event)
│   │           ├── events/    # Events feature state
│   │           ├── mobile_app/# Mobile app state
│   │           └── support/   # Support feature state
│   │
│   ├── ui/            # Shared UI components
│   │   └── src/
│   │       ├── events/        # Events-specific components
│   │       ├── marketing/     # Marketing site components
│   │       ├── mobile_app/    # Mobile-specific components
│   │       ├── shared/        # Cross-app shared components
│   │       ├── support/       # Support feature components
│   │       └── web_app/       # Web app components
│   │
│   ├── web_app/       # Main SaaS application
│   │   └── src/
│   │       ├── main.rs        # App entry point
│   │       ├── routes.rs      # Route definitions + Layout
│   │       ├── gate.rs        # Permission-based route guards
│   │       └── routes/        # Route components
│   │
│   ├── mobile_app/    # Mobile application (future)
│   ├── tooling/       # Build tools (CSS processing)
│   └── websites/      # Public-facing websites
│       ├── events/
│       ├── marketing/
│       ├── support/
│       └── external/  # Static campaign sites
│
├── tailwind/          # Tailwind configuration
├── target/            # Build outputs
├── Cargo.toml         # Workspace configuration
└── AGENTS.md          # This file
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
```

- **interfaces/** - Request/Response DTOs for API communication
- **models/** - Diesel ORM models (database rows)
- **providers/** - Dioxus `#[server]` functions (API endpoints)
- **services/** - Business logic and database operations
- **state/** - Client-side state types with `#[derive(Store)]`


### Feature Flags Architecture

The project uses Cargo features to gate code per-application:

```toml
# api/Cargo.toml
[features]
events = []
mobile_app = []
server = ["dioxus/server", ...]
support = []
web_app = []

# ui/Cargo.toml  
[features]
events = []
marketing = []
mobile_app = []
server = ["api/server"]
support = []
web = ["dioxus/web", ...]
web_app = []
```

This ensures each app only includes relevant code for smaller bundles.

---

## Coding Conventions

### Rust/Dioxus Guidelines

#### Props & Types
```rust
// ✅ ALWAYS: Define props OUTSIDE the component
#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    r#type: ButtonType,
    disabled: Option<Memo<bool>>,  // Use Option for optional props
    onclick: Option<EventHandler<MouseEvent>>,
    size: ButtonSize,
    variant: ButtonVariant,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    // ...
}

// ❌ NEVER: Inline props or default values in derive
```

#### Enums for Variants
```rust
// ✅ ALWAYS: Use exhaustive enums for variants
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Destructive,
    Link,
    Sidebar,
    SidebarActive,
}

// Use exhaustive match
let classes = match variant {
    ButtonVariant::Primary => "bg-primary text-primary-foreground",
    ButtonVariant::Secondary => "bg-secondary text-secondary-foreground",
    // ... all variants
};

// ❌ NEVER: Use if/else chains when match is possible
```

#### String Handling
```rust
// ✅ ALWAYS: Use .to_string()
let name = "Campaign".to_string();
let formatted = format!("{} {}", first, last);

// ❌ NEVER: Use String::from() or String::new()
let name = String::from("Campaign");  // Avoid
```

#### Error Handling
```rust
// ✅ ALWAYS: Use let-else for early returns
let Some(user) = get_user() else {
    return rsx! {};
};

let Ok(data) = fetch_data().await else {
    log::error!("Failed to fetch data");
    return Err(AppError::FetchFailed);
};

// ❌ NEVER: Use .unwrap() in production code
let user = get_user().unwrap();  // Avoid
```

#### Signal Access
```rust
// ✅ ALWAYS: Use .read() and .write() for efficiency
let value = signal.read().field;
signal.write().field = new_value;

// Clone only when necessary
let owned = signal.cloned();

// ❌ AVOID: Unnecessary cloning
let value = (*signal.read()).clone().field;  // Wasteful
```

#### Component Organization
```rust
// ✅ Component file structure:
// 1. Submodule declarations
mod child_component;
mod helper;

// 2. Imports
use dioxus::prelude::*;
use crate::shared::button::Button;

// 3. Types and enums
#[derive(Clone, PartialEq)]
pub enum ComponentVariant { ... }

// 4. Props struct
#[derive(Clone, PartialEq, Props)]
pub struct ComponentProps { ... }

// 5. Component function
#[component]
pub fn Component(props: ComponentProps) -> Element { ... }

// 6. Helper functions (private)
fn helper_function() { ... }
```

#### Numbers
```rust
// ✅ ALWAYS: Use i32 for general integers
pub struct Event {
    pub id: i32,
    pub capacity: i32,
    pub attendee_count: i32,
}

// Only use other types when specifically required (e.g., i64 for timestamps)
```

### CSS/Tailwind Guidelines

#### Flexbox First
```rust
// ✅ ALWAYS: Use flexbox for layout
div { class: "flex flex-col gap-4",
    div { class: "flex items-center justify-between", ... }
}

// ❌ AVOID: Complex positioning, floats, or cutting-edge CSS
```

#### Icon Styling
```rust
// ✅ ALWAYS: Style Lucide icons with class only
use lucide_dioxus::Calendar;

rsx! {
    Calendar { class: "w-5 h-5 text-primary" }
}

// ❌ NEVER: Use inline style or other props for sizing
Calendar { width: 20, height: 20 }  // Avoid
```

#### Class Organization
```rust
// Pattern: common_classes + size_classes + variant_classes + custom_classes
let combined_classes = format!(
    "{} {} {} {}",
    common_classes,
    size_classes,
    variant_classes,
    props.class.unwrap_or_default()
);
```

### Forbidden Patterns

```rust
// ❌ NEVER use eval() for JavaScript
eval("document.getElementById('x').focus()");

// ✅ INSTEAD: Use wasm-bindgen/web-sys/gloo
#[cfg(feature = "web")]
{
    use web_sys::window;
    if let Some(doc) = window().and_then(|w| w.document()) {
        // DOM manipulation here
    }
}
```

---

## Architecture Patterns

### Context & State Management

```rust
// Global user context (provided at Layout level)
#[derive(Clone, PartialEq)]
pub struct UserAccountContext {
    pub user_account: Store<UserAccount>,
}

// Usage in components
let ctx = use_context::<UserAccountContext>();
let user = ctx.user_account.read();
```

### Route Guards (Gate Component)

```rust
// Permission-based route protection
rsx! {
    Gate {
        required_permission: PermissionType::Events,
        permission_fallback_route: Routes::Dashboard {}.to_string(),
        // Protected content
        div { "Events content" }
    }
}
```

### Server Functions (API Calls)

```rust
// Server functions are in api/src/providers/
// DTOs are in api/src/interfaces/

// In api/src/interfaces/web_app/auth.rs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

// In api/src/providers/web_app/auth.rs
use crate::interfaces::web_app::RegisterRequest;

#[server]
pub async fn register(req: RegisterRequest) -> Result<AuthResponse, ServerFnError> {
    use crate::services::{register_user, create_session};
    // Server-side logic with database access
    let user = register_user(req.email, req.password, req.first_name, req.last_name).await?;
    let session = create_session(user.id, None, None).await?;
    // ...
}

// Called from components
use api::providers::web_app::register;
let result = register(request).await;
```

### Toast Notifications

```rust
let mut toast = use_context::<ToastContext>();
toast.create(
    "Success".to_string(),
    "Event created successfully!".to_string(),
    ToastVariant::Success,
);
```

---

## Database Schema Patterns

Using Diesel with PostgreSQL:

```rust
// migrations/YYYYMMDDHHMMSS_create_events/up.sql
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) NOT NULL DEFAULT 'America/New_York',
    location_type VARCHAR(20) NOT NULL, -- 'in_person', 'virtual', 'hybrid'
    address TEXT,
    virtual_link TEXT,
    capacity INTEGER,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

// src/schema.rs (auto-generated by diesel)
diesel::table! {
    events (id) {
        id -> Int4,
        organization_id -> Int4,
        title -> Varchar,
        // ...
    }
}
```

---

## File Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Components | snake_case file, PascalCase export | `event_card.rs` → `EventCard` |
| Types | snake_case file, PascalCase types | `event.rs` → `Event`, `EventType` |
| Modules | snake_case | `web_app.rs`, `mobile_app.rs` |
| Routes | snake_case matching URL | `/events` → `routes/events.rs` |
| Features | snake_case | `web_app`, `events`, `shared` |

---

## Important Notes for AI Assistants

1. **Ignore TODO comments** unless explicitly asked to address them
2. **Prefer wasm-bindgen/web-sys/gloo** over any JavaScript eval
3. **Always gate web-specific code** with `#[cfg(feature = "web")]`
4. **Always gate server-specific code** with `#[cfg(feature = "server")]`
5. **Check existing patterns** in similar components before implementing new ones
6. **Use exhaustive match** whenever working with enums
7. **Props are always external** to the component function
8. **Optional props** use `Option<T>`, not default values

---

## Related Documentation

- [Architecture Plan](docs/ARCHITECTURE.md)
- [Development Roadmap](docs/ROADMAP.md)
- [Feature Specifications](docs/features/)
  - [Events Feature](docs/features/EVENTS.md)
  - [Organizations & Teams](docs/features/ORGANIZATIONS.md)
  - [Communications](docs/features/COMMUNICATIONS.md)
