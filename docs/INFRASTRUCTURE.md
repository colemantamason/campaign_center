# Hosting Infrastructure Architecture

**Development**: All services (PostgreSQL, Redis, MinIO) run in Docker via `compose.yml`.

**How Dioxus Fullstack Works**:
Each Dioxus fullstack app compiles to a **single server binary** that:
- Runs an Axum HTTP server
- Serves the compiled WASM client bundle (static assets)
- Handles `#[server]` functions (API endpoints)
- Performs SSR (server-side rendering)

**Production** (8-12 VPS servers):

| Server | Services | Notes |
|--------|----------|-------|
| Database | PostgreSQL | Dedicated for data isolation, backups, and performance tuning |
| Cache & Storage | Redis + MinIO | Co-located lightweight services; split MinIO out if storage grows significantly |
| Web App | web_app binary | Primary SaaS application (Dioxus fullstack); scale horizontally later |
| Mobile App | mobile_app binary | Mobile backend (Dioxus fullstack - Native); separate for fault isolation and independent scaling |
| Events Website | events binary | Public event discovery (Dioxus fullstack - SSR); needs DB access for event data |
| Surveys Website | surveys binary | Public survey/polling response platform (Dioxus fullstack - SSR); needs DB access for questionnaire data |
| Marketing & Support Websites | marketing + support binaries | Marketing (Dioxus fullstack - SSR) and Help center (Dioxus fullstack); co-located as they share content data and have similar traffic patterns |
| CMS App + Workers Server | cms + workers binaries | CMS (Dioxus fullstack); workers binary (Axum) handles background jobs |

---

## Database Architecture

**Single PostgreSQL instance** with all data in the `public` schema.

**Why single Postgres, not separate databases or schemas:**
- Content tables (articles, media) reference core table (users) via foreign key — cross-database FKs are impossible, cross-schema FKs add Diesel ORM complexity
- Diesel's `diesel.toml` only supports one `[print_schema]` section per config file — multiple Postgres schemas would require separate config files, migration directories, and CLI invocations
- Operational simplicity: one backup strategy, one connection pool, one migration pipeline
- Table naming conventions provide clear logical grouping without tooling overhead

**When to revisit this decision:**
- If core campaign features (voter data, canvassing) generate enough write load that CMS read queries compete for resources
- If chat message volume grows beyond what a single Postgres handles (unlikely for support-only chat)
- The clean `api` services layer provides a natural abstraction boundary for splitting databases later without a rewrite

### Redis Usage

| Purpose | Pattern | Notes |
|---------|---------|-------|
| Sessions | Key-value (token → session data) | Already implemented; fast auth validation |
| CMS content cache | Key-value (slug → rendered article HTML) | Long expiry; articles change rarely; invalidate on publish |
| Chat real-time state | Pub/Sub channels + sorted sets | Presence indicators, typing status, real-time message delivery between WebSocket connections |
| Rate limiting | Sliding window counters | Auth endpoints, API throttling |

### Live Chat Architecture (Support App)

The live chat system is for **our SaaS customers contacting our support team only** (not a white-label feature for campaigns). This means we assume low volume.

```
┌──────────────┐    WebSocket   ┌──────────────┐    Redis Pub/Sub   ┌──────────────┐
│  Customer    │◀──────────────▶│   Support    │◀──────────────────▶│   Support    │
│  (widget)    │                │   App Server │                    │  Agent (CMS) │
└──────────────┘                └──────┬───────┘                    └──────────────┘
                                       │
                                       ▼
                                ┌──────────────┐
                                │  PostgreSQL  │
                                │  (persist)   │
                                └──────────────┘
```

- **WebSockets** on the support app server handle real-time message delivery
- **Redis Pub/Sub** routes messages between the customer-facing support app and agent-facing CMS app (since they're separate server processes)
- **PostgreSQL** persists all message history for conversation review, search, and analytics
- **Redis sorted sets** track agent presence/availability and typing indicators

---

**Background Workers**:
Workers are a **separate Rust binary** (not the Dioxus server binary). They:
- Share the `api` package's database/redis connection logic
- Run independently of HTTP requests (polling job queues, scheduled tasks)
- Handle: email queues, SMS sending, scheduled analytics, data exports
- Can start on the CMS server (low resource usage) and separate when volume grows

**Scaling Path**:
1. Add load balancer + additional instances for apps as users grow
2. Separate Workers to dedicated server when job volume increases
3. Split MinIO to dedicated server if storage needs grow
4. Add CDN for public content (marketing pages, blog articles, support articles) to reduce origin load
5. Add read replica for PostgreSQL if analytics/reporting queries impact transactional performance