# Hosting Infrastructure Architecture

**Development**: All services (PostgreSQL, Redis, MinIO) run in Docker via `compose.yml`.

**How Dioxus Fullstack Works**:
Each Dioxus fullstack app compiles to a **single server binary** that:
- Runs an Axum HTTP server
- Serves the compiled WASM client bundle (static assets)
- Handles `#[server]` functions (API endpoints)
- Performs SSR (server-side rendering)

**Production** (6-7 VPS servers):

| Server | Services | Notes |
|--------|----------|-------|
| Database | PostgreSQL | Dedicated for data isolation, backups, and performance tuning |
| Cache & Storage | Redis + MinIO | Co-located lightweight services; split MinIO out if storage grows significantly |
| Web App | web_app binary | Primary SaaS application (Dioxus fullstack); scale horizontally later |
| Mobile App | mobile_app binary | Mobile backend (Dioxus fullstack); separate for fault isolation and independent scaling |
| Events Website | events binary | Public event discovery (Dioxus fullstack); needs DB access for event data |
| Support Website | support binary | Help center + chat widget (Dioxus fullstack); can co-locate with Events initially if traffic is low |
| Marketing + Workers | nginx + workers binary | Marketing is static (nginx serves pre-built WASM/HTML); workers binary handles background jobs |

**Application Types**:
- **Fullstack apps** (web_app, mobile_app, events, support): Each produces its own server binary with bundled WASM client
- **Static sites** (marketing): Pre-built WASM/HTML served by nginx - no Dioxus server needed

**Background Workers**:
Workers are a **separate Rust binary** (not the Dioxus server binary). They:
- Share the `api` package's database/redis connection logic
- Run independently of HTTP requests (polling job queues, scheduled tasks)
- Handle: email queues, SMS sending, scheduled analytics, data exports
- Can start on the Marketing server (low resource usage) and separate when volume grows

**Scaling Path**:
1. Add load balancer + additional instances for apps as users grow
2. Separate Workers to dedicated server when job volume increases
3. Split MinIO to dedicated server if storage needs grow