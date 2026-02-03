# Campaign Center - Technical Architecture

> **Last Updated**: February 2026  
> **Status**: Planning Phase

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Application Architecture](#application-architecture)
3. [Backend Architecture](#backend-architecture)
4. [Database Design](#database-design)
5. [Infrastructure](#infrastructure)
6. [Security Architecture](#security-architecture)
7. [Integration Points](#integration-points)

---

## System Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           LOAD BALANCER                                  │
│                        (Nginx / Caddy)                                   │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        ▼                           ▼                           ▼
┌───────────────┐         ┌───────────────┐         ┌───────────────┐
│   Web App     │         │ Events Site   │         │ Marketing &   │
│   (Dioxus)    │         │ (Dioxus)      │         │ Support Sites │
│   Port 3000   │         │ Port 3001     │         │ Port 3002/3   │
└───────┬───────┘         └───────┬───────┘         └───────────────┘
        │                         │
        └────────────┬────────────┘
                     ▼
        ┌───────────────────────────────────────────────────────┐
        │                  SHARED SERVICES                       │
        ├───────────────────────────────────────────────────────┤
        │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
        │  │ PostgreSQL  │  │   MinIO     │  │   Redis     │    │
        │  │ (Database)  │  │ (Files/S3)  │  │ (Sessions)  │    │
        │  └─────────────┘  └─────────────┘  └─────────────┘    │
        └───────────────────────────────────────────────────────┘
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        ▼                           ▼                           ▼
┌───────────────┐         ┌───────────────┐         ┌───────────────┐
│   AWS SES     │         │   Twilio      │         │   Stripe      │
│   (Email)     │         │   (SMS)       │         │   (Payments)  │
└───────────────┘         └───────────────┘         └───────────────┘
```

### Application Matrix

| Application | Type | Purpose | Target Users |
|------------|------|---------|--------------|
| Web App | Dioxus Fullstack | Campaign/org management | Staff, admins |
| Events Site | Dioxus Fullstack | Public event discovery | Volunteers, public |
| Marketing Site | Dioxus Fullstack | Landing pages, signups | Prospects |
| Support Site | Dioxus Fullstack | Help center, chat widget | All users |
| Mobile App | Dioxus Native | Notifications, fieldwork | Field staff |

---

## Application Architecture

### Package Dependency Graph

```
                              ┌─────────────┐
                              │   web_app   │
                              │  (binary)   │
                              └──────┬──────┘
                                     │
                    ┌────────────────┼────────────────┐
                    ▼                ▼                ▼
              ┌─────────┐      ┌─────────┐      ┌─────────┐
              │   ui    │      │   api   │      │ dioxus  │
              │(shared) │      │(shared) │      │  core   │
              └────┬────┘      └────┬────┘      └─────────┘
                   │                │
                   └────────┬───────┘
                            ▼
                    ┌───────────────┐
                    │    shared     │
                    │   (types)     │
                    └───────────────┘
```

### Feature Flag Matrix

| Package | Feature | Enables |
|---------|---------|---------|
| `api` | `server` | Server functions, DB access |
| `api` | `events` | Events API types/functions |
| `api` | `web_app` | Web app API types/functions |
| `api` | `shared` | Cross-app shared types |
| `ui` | `web` | Browser-specific code (gloo, web-sys) |
| `ui` | `server` | Server-side rendering |
| `ui` | `events` | Events UI components |
| `ui` | `web_app` | Web app UI components |
| `ui` | `shared` | Shared UI components |

### Component Library Structure

```
ui/src/
├── shared/                    # Cross-app components
│   ├── button.rs             # Button, ButtonType, ButtonVariant, ButtonSize
│   ├── input.rs              # Input, InputType, InputVariant, masked inputs
│   ├── checkbox.rs           # Checkbox component
│   ├── divider.rs            # Divider component
│   ├── form.rs               # Form utilities, FormStatus
│   └── icon.rs               # Icon wrapper for lucide
│
├── web_app/                   # Web app specific
│   ├── avatar.rs             # User/org avatars
│   ├── confirmation_modal.rs # Confirmation dialogs
│   ├── notification_badge.rs # Notification indicators
│   ├── sidebar/              # Sidebar navigation
│   │   ├── mod.rs
│   │   ├── nav_button.rs
│   │   ├── nav_label.rs
│   │   └── sidebar_menu.rs
│   └── toast.rs              # Toast notification system
│
└── events/                    # Events website components
    ├── event_card.rs         # Event listing card
    ├── event_detail.rs       # Event detail view
    ├── rsvp_form.rs          # RSVP/registration form
    └── search/               # Event search components
```

---

## Backend Architecture

### Server Function Pattern

All API calls use Dioxus server functions, which compile to:
- **Client**: RPC calls over HTTP
- **Server**: Axum handlers with database access

```rust
// api/src/web_app/events.rs

// Request/Response types (shared between client and server)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timezone: String,
    pub location: EventLocation,
    pub capacity: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub organization_id: i32,
    pub title: String,
    // ... all fields
}

// Server function (only compiled for server)
#[server]
pub async fn create_event(req: CreateEventRequest) -> Result<Event, ServerFnError> {
    use crate::db::establish_connection;
    use crate::schema::events;
    use diesel::prelude::*;
    
    // Validate session/permissions
    let session = get_session().await?;
    let org_id = session.active_organization_id
        .ok_or(ServerFnError::new("No active organization"))?;
    
    // Insert into database
    let conn = &mut establish_connection()?;
    let new_event = diesel::insert_into(events::table)
        .values(&NewEvent::from_request(req, org_id))
        .get_result::<Event>(conn)?;
    
    Ok(new_event)
}
```

### Session Management

```rust
// Session stored in Redis with PostgreSQL fallback
pub struct Session {
    pub id: String,           // UUID token
    pub user_id: i32,
    pub active_organization_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub user_agent: String,
    pub ip_address: String,
}

// Middleware extracts session from cookie
pub async fn get_session() -> Result<Session, ServerFnError> {
    // Extract from Axum request context
    let session_token = extract_session_cookie()?;
    let session = redis_get_session(&session_token).await
        .or_else(|_| db_get_session(&session_token).await)?;
    
    if session.expires_at < Utc::now() {
        return Err(ServerFnError::new("Session expired"));
    }
    
    Ok(session)
}
```

### Error Handling

```rust
// Custom error types that serialize properly
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppError {
    NotAuthenticated,
    NotAuthorized { required: PermissionType },
    NotFound { entity: String },
    ValidationError { field: String, message: String },
    DatabaseError { message: String },
    ExternalService { service: String, message: String },
}

impl From<AppError> for ServerFnError {
    fn from(err: AppError) -> Self {
        ServerFnError::new(serde_json::to_string(&err).unwrap_or_default())
    }
}
```

---

## Database Design

### Core Schema Overview

```sql
-- Organizations & Users
organizations
users
organization_members
invitations

-- Events & RSVPs
events
event_cohosts
event_attendees
event_reminders
event_shifts

-- Groups & Communications
groups
group_members
messages
message_recipients

-- Voter Data (Phase 2)
voters
voter_contacts
canvass_routes
phone_bank_sessions
text_campaigns

-- Platform
sessions
audit_logs
files
```

### Key Tables Detail

#### Organizations

```sql
CREATE TABLE organizations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    avatar_url TEXT,
    website_url TEXT,
    
    -- Settings
    timezone VARCHAR(50) NOT NULL DEFAULT 'America/New_York',
    default_event_capacity INTEGER,
    
    -- Stripe integration
    stripe_customer_id VARCHAR(255),
    stripe_subscription_id VARCHAR(255),
    subscription_status VARCHAR(50) DEFAULT 'trial',
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_organizations_slug ON organizations(slug);
```

#### Users

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified_at TIMESTAMPTZ,
    password_hash VARCHAR(255) NOT NULL,
    
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    phone_verified_at TIMESTAMPTZ,
    avatar_url TEXT,
    
    -- Preferences
    timezone VARCHAR(50) DEFAULT 'America/New_York',
    notification_preferences JSONB DEFAULT '{}',
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users(email);
```

#### Organization Members

```sql
CREATE TABLE organization_members (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    role VARCHAR(50) NOT NULL DEFAULT 'member', -- 'owner', 'admin', 'member'
    permissions JSONB NOT NULL DEFAULT '{}',
    
    invited_by INTEGER REFERENCES users(id),
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(organization_id, user_id)
);

CREATE INDEX idx_org_members_org ON organization_members(organization_id);
CREATE INDEX idx_org_members_user ON organization_members(user_id);
```

#### Events

```sql
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    
    -- Basic Info
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    description TEXT,
    short_description VARCHAR(500),
    cover_image_url TEXT,
    
    -- Timing
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) NOT NULL DEFAULT 'America/New_York',
    
    -- Location
    location_type VARCHAR(20) NOT NULL, -- 'in_person', 'virtual', 'hybrid'
    address_line_1 VARCHAR(255),
    address_line_2 VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(50),
    zip VARCHAR(20),
    country VARCHAR(2) DEFAULT 'US',
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    virtual_link TEXT,
    virtual_platform VARCHAR(50),
    
    -- Capacity & Registration
    capacity INTEGER,
    waitlist_enabled BOOLEAN DEFAULT FALSE,
    registration_deadline TIMESTAMPTZ,
    requires_approval BOOLEAN DEFAULT FALSE,
    
    -- Visibility
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    is_featured BOOLEAN DEFAULT FALSE,
    visibility VARCHAR(20) DEFAULT 'public', -- 'public', 'unlisted', 'private'
    
    -- Contact
    contact_email VARCHAR(255),
    contact_phone VARCHAR(20),
    
    -- Metadata
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    
    UNIQUE(organization_id, slug)
);

CREATE INDEX idx_events_org ON events(organization_id);
CREATE INDEX idx_events_start ON events(start_time);
CREATE INDEX idx_events_published ON events(is_published, start_time);
```

#### Event Attendees

```sql
CREATE TABLE event_attendees (
    id SERIAL PRIMARY KEY,
    event_id INTEGER NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id),
    
    -- Guest info (for non-users)
    email VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    
    -- RSVP Details
    status VARCHAR(20) NOT NULL DEFAULT 'registered', -- 'registered', 'waitlisted', 'cancelled', 'attended', 'no_show'
    guest_count INTEGER DEFAULT 0,
    notes TEXT,
    
    -- Source tracking
    source VARCHAR(50), -- 'direct', 'share', 'embed', 'email'
    referrer_id INTEGER REFERENCES users(id),
    
    -- Metadata
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    checked_in_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    
    UNIQUE(event_id, email)
);

CREATE INDEX idx_attendees_event ON event_attendees(event_id);
CREATE INDEX idx_attendees_user ON event_attendees(user_id);
CREATE INDEX idx_attendees_email ON event_attendees(email);
```

#### Event Co-hosts

```sql
CREATE TABLE event_cohosts (
    id SERIAL PRIMARY KEY,
    event_id INTEGER NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    
    -- Permissions
    can_edit BOOLEAN DEFAULT FALSE,
    can_manage_attendees BOOLEAN DEFAULT TRUE,
    can_send_messages BOOLEAN DEFAULT TRUE,
    
    -- Status
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'accepted', 'rejected'
    invited_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    responded_at TIMESTAMPTZ,
    
    UNIQUE(event_id, organization_id)
);

CREATE INDEX idx_cohosts_event ON event_cohosts(event_id);
CREATE INDEX idx_cohosts_org ON event_cohosts(organization_id);
```

---

## Infrastructure

### VPS Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                        VPS SERVER                               │
│                    (Ubuntu 24.04 LTS)                          │
├────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    Docker Network                         │  │
│  │                                                           │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │  │
│  │  │   Caddy     │  │   Web App   │  │ Events App  │      │  │
│  │  │ (Reverse    │  │   :3000     │  │   :3001     │      │  │
│  │  │  Proxy)     │  │             │  │             │      │  │
│  │  │  :80/:443   │  │             │  │             │      │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘      │  │
│  │                                                           │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │  │
│  │  │ PostgreSQL  │  │   Redis     │  │   MinIO     │      │  │
│  │  │  :5432      │  │   :6379     │  │   :9000     │      │  │
│  │  │             │  │             │  │             │      │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘      │  │
│  │                                                           │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
│  Volumes: /data/postgres, /data/redis, /data/minio            │
│                                                                 │
└────────────────────────────────────────────────────────────────┘
```

### Docker Compose Structure

```yaml
# docker-compose.yml
version: '3.8'

services:
  caddy:
    image: caddy:2-alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile
      - caddy_data:/data
    depends_on:
      - web_app
      - events_app

  web_app:
    build:
      context: .
      dockerfile: Dockerfile.web_app
    environment:
      - DATABASE_URL=postgres://user:pass@postgres/campaign_center
      - REDIS_URL=redis://redis:6379
      - MINIO_ENDPOINT=http://minio:9000
    depends_on:
      - postgres
      - redis
      - minio

  events_app:
    build:
      context: .
      dockerfile: Dockerfile.events
    environment:
      - DATABASE_URL=postgres://user:pass@postgres/campaign_center
      - REDIS_URL=redis://redis:6379
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:16-alpine
    environment:
      - POSTGRES_DB=campaign_center
      - POSTGRES_USER=campaign
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U campaign"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    command: redis-server --appendonly yes

  minio:
    image: minio/minio
    environment:
      - MINIO_ROOT_USER=${MINIO_ACCESS_KEY}
      - MINIO_ROOT_PASSWORD=${MINIO_SECRET_KEY}
    volumes:
      - minio_data:/data
    command: server /data --console-address ":9001"

volumes:
  caddy_data:
  postgres_data:
  redis_data:
  minio_data:
```

### Caddy Configuration

```
# Caddyfile
{
    email admin@campaigncenter.com
}

app.campaigncenter.com {
    reverse_proxy web_app:3000
}

events.campaigncenter.com {
    reverse_proxy events_app:3001
}

campaigncenter.com, www.campaigncenter.com {
    reverse_proxy marketing_app:3002
}

support.campaigncenter.com {
    reverse_proxy support_app:3003
}

# MinIO console (internal only)
storage.internal.campaigncenter.com {
    reverse_proxy minio:9001
}
```

### Backup Strategy

```bash
#!/bin/bash
# /scripts/backup.sh

# Daily PostgreSQL backup
pg_dump -h postgres -U campaign campaign_center | gzip > /backups/postgres/$(date +%Y%m%d).sql.gz

# Sync to off-site storage (S3-compatible)
aws s3 sync /backups s3://campaign-center-backups/ --endpoint-url https://offsite-storage.com

# Retain last 30 daily, 12 weekly, 12 monthly
find /backups/postgres -mtime +30 -delete
```

---

## Security Architecture

### Authentication Flow

```
┌─────────────┐     1. POST /api/login      ┌─────────────┐
│   Browser   │ ──────────────────────────► │   Server    │
│             │     {email, password}       │             │
└─────────────┘                             └──────┬──────┘
                                                   │
                                    2. Verify password hash
                                                   │
                                            ┌──────▼──────┐
                                            │  PostgreSQL │
                                            │   (users)   │
                                            └──────┬──────┘
                                                   │
                                    3. Create session
                                                   │
                                            ┌──────▼──────┐
                                            │    Redis    │
                                            │ (sessions)  │
                                            └──────┬──────┘
                                                   │
┌─────────────┐     4. Set-Cookie: session_id     │
│   Browser   │ ◄──────────────────────────────────┘
│             │     (HttpOnly, Secure, SameSite)
└─────────────┘
```

### Password Security

```rust
// Using Argon2id for password hashing
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
```

### API Security Headers

```rust
// Middleware for security headers
pub fn security_headers() -> impl IntoResponse {
    ([
        ("X-Content-Type-Options", "nosniff"),
        ("X-Frame-Options", "DENY"),
        ("X-XSS-Protection", "1; mode=block"),
        ("Referrer-Policy", "strict-origin-when-cross-origin"),
        ("Content-Security-Policy", "default-src 'self'; ..."),
    ])
}
```

### Rate Limiting

```rust
// Per-endpoint rate limits stored in Redis
pub enum RateLimit {
    Login { max: 5, window_secs: 300 },         // 5 per 5 minutes
    Registration { max: 3, window_secs: 3600 }, // 3 per hour
    Api { max: 100, window_secs: 60 },          // 100 per minute
}
```

---

## Integration Points

### AWS SES (Email)

```rust
// Email sending abstraction
pub struct EmailService {
    client: aws_sdk_ses::Client,
    from_address: String,
}

impl EmailService {
    pub async fn send_transactional(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), Error> {
        // Use SES SendEmail API
    }
    
    pub async fn send_bulk(
        &self,
        template: &str,
        recipients: Vec<BulkRecipient>,
    ) -> Result<BulkSendResult, Error> {
        // Use SES SendBulkTemplatedEmail API
    }
}

// Email types
pub enum EmailTemplate {
    Welcome { user_name: String },
    EventReminder { event: Event, hours_until: i32 },
    EventConfirmation { event: Event, attendee: Attendee },
    PasswordReset { reset_link: String },
    TeamInvitation { org_name: String, inviter: String, invite_link: String },
}
```

### Twilio (SMS)

```rust
pub struct SmsService {
    client: twilio::Client,
    from_number: String,
}

impl SmsService {
    pub async fn send(
        &self,
        to: &str,
        message: &str,
    ) -> Result<MessageSid, Error> {
        // Twilio REST API
    }
    
    pub async fn send_bulk(
        &self,
        recipients: Vec<SmsRecipient>,
    ) -> Result<BulkSmsResult, Error> {
        // Messaging Service for bulk
    }
}

pub enum SmsTemplate {
    EventReminder { event_title: String, time: String, link: String },
    RsvpConfirmation { event_title: String },
    VolunteerShiftReminder { shift: String, location: String },
}
```

### Stripe (Payments)

```rust
pub struct PaymentService {
    client: stripe::Client,
}

impl PaymentService {
    // Ticketed events
    pub async fn create_checkout_session(
        &self,
        event: &Event,
        tickets: Vec<TicketSelection>,
        attendee_email: &str,
    ) -> Result<CheckoutSession, Error> {
        // Create Stripe Checkout Session
    }
    
    // Organization subscriptions
    pub async fn create_subscription(
        &self,
        org: &Organization,
        plan: SubscriptionPlan,
    ) -> Result<Subscription, Error> {
        // Create Stripe Subscription
    }
    
    // Webhook handling
    pub async fn handle_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<WebhookEvent, Error> {
        // Verify and process Stripe webhooks
    }
}
```

### MinIO (File Storage)

```rust
pub struct FileService {
    client: aws_sdk_s3::Client,  // S3-compatible
    bucket: String,
    public_url: String,
}

impl FileService {
    pub async fn upload(
        &self,
        key: &str,
        content: &[u8],
        content_type: &str,
    ) -> Result<String, Error> {
        // Upload to MinIO, return public URL
    }
    
    pub async fn generate_presigned_upload(
        &self,
        key: &str,
        expires_in: Duration,
    ) -> Result<PresignedUrl, Error> {
        // Generate presigned URL for direct upload
    }
    
    pub async fn delete(&self, key: &str) -> Result<(), Error> {
        // Delete from MinIO
    }
}

// File organization
// /avatars/users/{user_id}/{filename}
// /avatars/orgs/{org_id}/{filename}
// /events/{event_id}/cover/{filename}
// /exports/{org_id}/{export_id}/{filename}
```

---

## Performance Considerations

### Caching Strategy

```
┌─────────────┐
│   Request   │
└──────┬──────┘
       │
       ▼
┌──────────────────────────────────────────────────────┐
│                  Cache Layers                         │
├──────────────────────────────────────────────────────┤
│  L1: Dioxus Memo/Signals (in-memory, per-component)  │
│  L2: Redis (shared, sessions, frequently accessed)   │
│  L3: PostgreSQL (persistent, source of truth)        │
└──────────────────────────────────────────────────────┘
```

### Database Indexing

```sql
-- Critical query patterns indexed
CREATE INDEX idx_events_org_published ON events(organization_id, is_published, start_time);
CREATE INDEX idx_events_location ON events USING GIST (
    ll_to_earth(latitude, longitude)
) WHERE latitude IS NOT NULL;
CREATE INDEX idx_attendees_event_status ON event_attendees(event_id, status);
CREATE INDEX idx_messages_recipient ON message_recipients(user_id, read_at);
```

---

## Monitoring & Observability

### Logging

```rust
// Structured logging with tracing
use tracing::{info, error, span, Level};

#[server]
pub async fn create_event(req: CreateEventRequest) -> Result<Event, ServerFnError> {
    let span = span!(Level::INFO, "create_event", org_id = %req.organization_id);
    let _enter = span.enter();
    
    info!("Creating event: {}", req.title);
    
    // ... implementation
    
    match result {
        Ok(event) => {
            info!(event_id = %event.id, "Event created successfully");
            Ok(event)
        }
        Err(error) => {
            error!(error = ?e, "Failed to create event");
            Err(error.into())
        }
    }
}
```

### Health Checks

```rust
// /health endpoint
pub async fn health_check() -> impl IntoResponse {
    let db_healthy = check_database().await.is_ok();
    let redis_healthy = check_redis().await.is_ok();
    let minio_healthy = check_minio().await.is_ok();
    
    if db_healthy && redis_healthy && minio_healthy {
        (StatusCode::OK, "healthy")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "unhealthy")
    }
}
```

---

## Related Documentation

- [AGENTS.md](../AGENTS.md) - AI coding assistant context
- [ROADMAP.md](ROADMAP.md) - Development timeline
- [Features Documentation](features/) - Detailed feature specs
