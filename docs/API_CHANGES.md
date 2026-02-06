# API Changes — CMS, Blog, Support Articles & Chat

> **Last Updated**: 6 February 2026

Tracks implementation of the CMS content system (articles, categories, tags, revisions, media), public article views (blog, support/help center), and support chat. Each item is marked with its completion status.

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Implemented |
| ⬜ | Not started |

---

## Phase A: Enums (shared)

| File | Types | Status |
|------|-------|--------|
| `enums/shared/article.rs` | `ArticleType { Blog, Support }`, `ArticleStatus { Draft, Published, Archived }`, `ARTICLE_CACHE_EXPIRY_SECONDS` | ✅ |
| `enums/shared/chat.rs` | `ConversationStatus { Open, Assigned, Resolved, Closed }`, `ChatParticipantRole { Customer, Agent }`, `ChatMessageType { Text, System, Attachment }` | ⬜ |

- ✅ Both article enums registered in `enums/shared.rs`
- ⬜ Chat enums not yet registered

---

## Phase B: Models (server-only)

### Content Models

| File | Types | Status |
|------|-------|--------|
| `models/article_category.rs` | `ArticleCategory`, `NewArticleCategory`, `ArticleCategoryUpdate` | ✅ |
| `models/article.rs` | `Article`, `NewArticle`, `ArticleUpdate` | ✅ |
| `models/article_tag.rs` | `ArticleTag`, `NewArticleTag`, `ArticleTagLink` (join table) | ✅ |
| `models/article_revision.rs` | `ArticleRevision`, `NewArticleRevision` | ✅ |
| `models/media_asset.rs` | `MediaAsset`, `NewMediaAsset` | ✅ |

### Chat Models

| File | Types | Status |
|------|-------|--------|
| `models/chat_conversation.rs` | `ChatConversation`, `NewChatConversation`, `ChatConversationUpdate` | ⬜ |
| `models/chat_message.rs` | `ChatMessage`, `NewChatMessage` | ⬜ |
| `models/chat_participant.rs` | `ChatParticipant`, `NewChatParticipant` | ⬜ |

- ✅ All content models registered in `models.rs`
- ⬜ Chat models not yet registered
- Note: Chat tables already exist in the database migration and `schema.rs` — only the Rust model structs are needed

### Content Type: `serde_json::Value`

Article `content` is stored as JSONB (`serde_json::Value`). The CMS editor will produce a structured JSON document tree (nodes like headings, paragraphs, lists, images, etc.) that a Dioxus renderer component converts to VNodes at render time. This is the same pattern used by ProseMirror/Slate editors — the content schema will be defined as our custom editor is built.

---

## Phase C: Interfaces (DTOs)

### CMS Interfaces

| File | Types | Status |
|------|-------|--------|
| `interfaces/cms/article.rs` | `CreateArticleRequest`, `UpdateArticleRequest`, `ListArticlesRequest`, `ArticleResponse`, `ArticleListResponse`, `ArticleRevisionResponse` | ✅ |
| `interfaces/cms/article_category.rs` | `CreateCategoryRequest`, `UpdateCategoryRequest`, `ReorderCategoriesRequest`, `CategoryResponse` | ✅ |
| `interfaces/cms/article_tag.rs` | `CreateTagRequest`, `SearchTagsRequest`, `TagResponse` | ✅ |
| `interfaces/cms/media.rs` | `UploadMediaRequest`, `ListMediaRequest`, `MediaAssetResponse`, `MediaListResponse` | ✅ |

### Shared Interfaces

| File | Types | Status |
|------|-------|--------|
| `interfaces/shared/article.rs` | `ArticleAuthorInfo`, `ArticleCategoryInfo`, `ArticleTagInfo`, `PublicArticleResponse`, `PublicArticleListResponse`, `ListPublicArticlesRequest` | ✅ |

### Support Interfaces

| File | Types | Status |
|------|-------|--------|
| `interfaces/support/chat.rs` | `CreateConversationRequest`, `SendMessageRequest`, `ConversationResponse`, `MessageResponse` | ⬜ |

- ✅ CMS interfaces registered in `interfaces/cms.rs` and re-exported via `interfaces.rs` under the `cms` feature
- ✅ Shared sub-types (`ArticleAuthorInfo`, etc.) live in `interfaces/shared/article.rs` so they're available regardless of feature flags
- ⬜ Support chat interfaces not yet created

---

## Phase D: Services (server-only, business logic)

### CMS Services

| File | Functions | Status |
|------|-----------|--------|
| `services/cms/article.rs` | CRUD, publish (creates revision + sets status/published_at + invalidates Redis cache), auto-save (update content), list with filters (by type, status, category), get by slug, helper functions for building response sub-types (author info, category info, tag infos), **batch helpers** (`batch_get_author_infos`, `batch_get_category_infos`, `batch_get_tag_infos`) for N+1-free list endpoints | ✅ |
| `services/cms/article_category.rs` | CRUD, list by article_type, reorder (single batched SQL via `UPDATE ... FROM (VALUES ...)`) | ✅ |
| `services/cms/article_tag.rs` | CRUD, search/autocomplete, manage article-tag links (`sync_article_tags` — single source of truth for join table sync) | ✅ |
| `services/cms/media.rs` | Upload to MinIO + create DB record, list with pagination, delete (MinIO + DB) | ✅ |
| `services/cms/article_revision.rs` | List revisions for article, get specific revision, restore revision (copy content back to article as new draft) | ✅ |

### Support Services

| File | Functions | Status |
|------|-----------|--------|
| `services/support/chat.rs` | Create conversation, send message, list conversations (by status, by user), mark messages read, assign agent, resolve/close, list messages for conversation | ⬜ |

### Shared Services

| File | Functions | Status |
|------|-----------|--------|
| `services/shared/article.rs` | Public read-only: get published article by slug (check Redis cache first, fallback to DB + cache result), list published articles by type with pagination, filtering by category slug and tag slug | ✅ |

---

## Phase E: Providers (server functions / API endpoints)

### CMS Providers

| File | Endpoints | Status |
|------|-----------|--------|
| `providers/cms/article.rs` | `create_article`, `update_article`, `publish_article`, `get_article`, `list_articles`, `delete_article`, `auto_save`, `list_article_revisions`, `restore_revision` | ✅ |
| `providers/cms/article_category.rs` | `create_category`, `list_categories`, `update_category`, `delete_category`, `reorder_categories` | ✅ |
| `providers/cms/article_tag.rs` | `create_tag`, `search_tags`, `delete_tag` | ✅ |
| `providers/cms/media.rs` | `upload_media`, `list_media`, `delete_media` | ✅ |

### Support Providers

| File | Endpoints | Status |
|------|-----------|--------|
| `providers/support/chat.rs` | `create_conversation`, `send_message`, `list_conversations`, `get_conversation_messages`, `assign_agent`, `resolve_conversation` | ⬜ |

### Shared Providers

| File | Endpoints | Status |
|------|-----------|--------|
| `providers/shared/article.rs` | `get_published_article` (by slug, cached), `list_published_articles` (by type, paginated, filterable by category_slug and tag_slug) | ✅ |

---

## Phase F: MinIO Integration (server-only) ✅

MinIO is running in Docker (`compose.yml`) as a self-hosted S3-compatible object store. The Rust client layer is implemented.

### Docker Setup ✅

- **Container**: `campaign_center_minio` (image: `minio/minio`)
- **API port**: `9000`, **Console port**: `9001`
- **Credentials**: `MINIO_ROOT_USER` / `MINIO_ROOT_PASSWORD` (defaults: `campaign_minio` / `campaign_minio_secret`)
- **Init container** (`minio-init`): Creates default buckets on first run via `mc` CLI
- **Buckets**: `avatars`, `events`, `exports`, `media` (all created by `minio-init`)
- **Public access**: `avatars` and `events` buckets have anonymous download enabled

### Rust Implementation ✅

| File | Description | Status |
|------|-------------|--------|
| `api/src/minio.rs` | MinIO client (OnceLock pattern like postgres/redis) | ✅ |

#### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MINIO_ENDPOINT` | MinIO API URL | `http://localhost:9000` |
| `MINIO_ACCESS_KEY` | Access key | `campaign_minio` |
| `MINIO_SECRET_KEY` | Secret key | `campaign_minio_secret` |
| `MINIO_BUCKET_MEDIA` | Media library bucket name | `media` |
| `MINIO_PUBLIC_URL` | Public-facing URL for presigned links (in production, may differ from internal endpoint) | Same as `MINIO_ENDPOINT` |

#### `minio.rs` — Implemented Functions

```
initialize_minio_client() — Build and store S3 client (OnceLock pattern)
is_minio_initialized() — Check if client is ready
get_minio_client() — Get reference to initialized client

minio_upload_object(bucket, key, data, content_type) — Upload bytes to MinIO, returns storage key
minio_delete_object(bucket, key) — Remove object from MinIO
get_minio_presigned_url(bucket, key, expires_in) — Generate time-limited download URL (replaces internal endpoint with MINIO_PUBLIC_URL)
object_exists(bucket, key) — Check if object exists (HeadObject)

minio_upload_media(key, data, content_type) — Convenience wrapper using MINIO_BUCKET_MEDIA
minio_delete_media(key) — Convenience wrapper using MINIO_BUCKET_MEDIA
get_minio_media_url(key) — Presigned URL with 1-hour expiry for media bucket
```

#### Dependencies (in `api/Cargo.toml`) ✅

```toml
aws-sdk-s3 = { workspace = true, optional = true }
aws-config = { workspace = true, optional = true }
aws-credential-types = { workspace = true, optional = true }
```

All three are gated behind the `server` feature.

#### Integration with `lib.rs` ✅

`initialize_services()` calls all three initializers:
```rust
pub fn initialize_services() -> Result<(), AppError> {
    if !is_postgres_initialized() { initialize_postgres_pool()?; }
    if !is_redis_initialized() { initialize_redis_pool()?; }
    if !is_minio_initialized() { initialize_minio_client()?; }
    Ok(())
}
```

### Media Upload Flow

```
Client                    Provider                  Service                   MinIO + DB
  │                          │                         │                          │
  │  UploadMediaRequest      │                         │                          │
  │  (file bytes + metadata) │                         │                          │
  │─────────────────────────▶│                         │                          │
  │                          │  upload_media()          │                          │
  │                          │────────────────────────▶│                          │
  │                          │                         │  1. Generate unique key   │
  │                          │                         │  2. Upload to MinIO       │
  │                          │                         │─────────────────────────▶│
  │                          │                         │  3. Create DB record      │
  │                          │                         │  (NewMediaAsset with      │
  │                          │                         │   storage_key, metadata)  │
  │                          │                         │                          │
  │  MediaAssetResponse      │                         │                          │
  │  (with presigned URL)    │                         │                          │
  │◀─────────────────────────│◀────────────────────────│                          │
```

When serving media URLs in responses, generate short-lived presigned URLs (1 hour) from the storage key. This keeps the bucket private while allowing time-limited access.

**Note**: The media upload provider currently passes `vec![]` for file bytes as a placeholder — actual file data delivery needs to be wired via multipart upload or a two-step presigned URL flow.

---

## Redis Integration ✅

Article caching functions are implemented in `redis.rs`:

| Function | Description | Status |
|----------|-------------|--------|
| `redis_cache_article_by_slug(slug, json)` | Cache serialized article response with `ARTICLE_CACHE_EXPIRY_SECONDS` (24h) TTL | ✅ |
| `get_redis_cached_article_by_slug(slug)` | Retrieve cached article JSON by slug | ✅ |
| `invalidate_redis_cached_article(slug)` | Delete cached article on publish/update/delete | ✅ |

Cache invalidation is called from:
- `services/cms/article.rs` — on publish, update (when slug changes), and delete
- `services/shared/article.rs` — caches article on first public read

---

## API Audit & Fixes (7 February 2026)

### Auth Audit

Full endpoint-by-endpoint auth review completed. All endpoints that should require authentication use `auth.require_auth()`. The only unauthenticated endpoints (besides `register` and `login`) are:

- `get_published_article` — intentionally public (serves blog/support articles to marketing, support, and events websites)
- `list_published_articles` — intentionally public (same as above)

Organization endpoints additionally verify membership and role where appropriate (view, invite, remove, update-role).

### Bug: `publish_article` Missing `updated_at`

The `publish_article` service set `status` and `published_at` but did **not** set `updated_at`. Fixed to include `updated_at: Some(now)` in the `ArticleUpdate` changeset, consistent with `update_article`.

### Bug: `restore_revision` Missing `updated_at`

The `restore_revision` service restored title/excerpt/content/status but did **not** set `updated_at`. Fixed to include `updated_at: Some(Utc::now())`.

### Bug: `update_redis_cached_session_active_organization_membership_id` Resetting TTL

When a user switched their active organization, this function re-cached the session with `None` expiry, which reset the Redis TTL to the full `SESSION_EXPIRY_SECONDS` (7 days). This bypassed the middleware's sliding session threshold logic. Fixed to preserve the remaining TTL by calling `get_redis_session_expiry()` before re-caching. The sliding session extension is now handled exclusively by the middleware.

### Fix: `search_tags` ILIKE Wildcard Escaping

The `search_tags` service used user input directly in an ILIKE pattern without escaping `%`, `_`, and `\` characters. A search for `%` would match all tags. Fixed to escape ILIKE special characters before constructing the pattern.

---

## Recent Fixes (7 February 2026)

### N+1 Queries Resolved — Batch Loading

Both `list_published_articles` (shared service) and `list_articles` (CMS provider) now use batch loading instead of per-article loops. Three batch helper functions were added to `services/cms/article.rs`:

- `batch_get_author_infos(author_ids)` — single query, returns `HashMap<i32, ArticleAuthorInfo>`
- `batch_get_category_infos(category_ids)` — single query, returns `HashMap<i32, ArticleCategoryInfo>`
- `batch_get_tag_infos(article_ids)` — 2 queries (join table + tags), returns `HashMap<i32, Vec<ArticleTagInfo>>`

Each list endpoint now collects unique IDs, calls the batch helpers, then assembles responses via HashMap lookup. The single-article helpers (`get_article_author_info`, etc.) are preserved for use in create/update/publish/get endpoints.

### `article_categories.slug` — Composite Unique Constraint

Changed from globally `UNIQUE` on `slug` to `UNIQUE(slug, article_type)`. Blog and support categories can now share the same slug. The migration (`up.sql`) was modified directly. **Run `diesel migration redo` then `diesel print-schema` to regenerate `schema.rs`.**

### `reorder_categories` — Batched Single SQL

Replaced the per-row `UPDATE` loop with a single `UPDATE article_categories AS ac SET sort_order = v.new_order FROM (VALUES ...) AS v(id, new_order) WHERE ac.id = v.id` using `diesel::sql_query`.

### `ArticleUpdate` — Added `updated_at`

The `ArticleUpdate` changeset now includes `updated_at: Option<DateTime<Utc>>`. The `update_article` service explicitly sets `updated_at` to `Utc::now()` on every edit. (The DB trigger also handles this, providing a safety net.)

---

## Known Issues & Future Improvements

### Media Upload Placeholder

The `upload_media` provider currently passes empty bytes — actual file upload needs to be wired via Dioxus multipart upload support or a presigned URL flow where the client uploads directly to MinIO.

### Deferred Items (lower priority, to be addressed as features are built)

| Item | Notes |
|------|-------|
| **Input length validation** | Article titles, slugs, tag/category names — no max length checks. DB constraints will catch overflows with cryptic errors. Add validation when building frontend forms. |
| **TOCTOU slug uniqueness** | Slug check-then-insert is not atomic for articles, tags, categories, and orgs. DB unique constraints catch duplicates, but error messages fall back to generic Diesel `UniqueViolation`. Consider catching `UniqueViolation` and returning friendly errors. |
| **`SESSION_COOKIE_DOMAIN` placeholder** | Hardcoded to `.domain.com` — must be replaced before production deployment. Has existing TODO. |
| **No rate limiting** | Auth endpoints (`register`, `login`, `change_password`) have no brute-force protection. Already on roadmap (Phase 2). |
| **No CSRF protection** | Cookie-based sessions use `SameSite=Lax` but no CSRF token validation. Already on roadmap (Phase 2). |
| **No password reset flow** | `password_reset_tokens` table exists in schema but no models/services/providers. |
| **No scheduled article publishing** | `scheduled_publish_at` field exists but nothing checks/triggers it. Needs a background worker. |
| **No email sending for invitations** | Invitation records are created but no email is dispatched. |
| **Media file validation** | No max file size or allowlisted MIME type checks in `upload_media`. |
| **Unused `get_article_by_slug` service** | Defined in `services/cms/article.rs` but not called by any provider. Remove or wire up when needed. |

---

## Recent Fixes (6 February 2026)

### Code Review Fixes

Comprehensive code review identified and fixed the following issues across the API:

#### Critical: `expect()` Panic Replaced with Safe Fallback

`build_public_article_response` and `batch_build_public_article_responses` in `services/shared/article.rs` used `.expect("published article must have published_at")` which would crash the server if a published article somehow had `NULL` `published_at`. Replaced with `.unwrap_or_else()` that logs a warning and falls back to `Utc::now()`.

#### Critical: Cross-Org Member Operations Fixed

`remove_organization_member` and `update_organization_member_role` in `providers/web_app/organization.rs` accepted a `member_id` without verifying it belonged to the `organization_id` in the URL. An admin of org A could remove/modify a member in org B. Both endpoints now verify the target member exists within the specified organization.

#### High: N+1 Queries Resolved

- **`get_organization_members`** — replaced per-member `get_user_by_id` loop with a single batch `users::table.filter(users::id.eq_any(...))` query.
- **`get_current_user`** — replaced per-organization `count_members` loop with `batch_count_members` (single `GROUP BY` query). New function added to `services/shared/organization.rs`.
- **`list_article_revisions`** — replaced per-revision `get_article_author_info` loop with `batch_get_author_infos`.

#### Fixed: Redis Session TTL Now Preserved on Metadata Updates

`update_redis_cached_session_active_organization_membership_id` previously reset the Redis TTL to the full `SESSION_EXPIRY_SECONDS` when re-caching session data. This was updated on 7 February 2026 to preserve the remaining TTL instead. Sliding session extension is now handled exclusively by the session middleware, not by metadata update functions.

#### Medium: Silenced Redis Errors Now Logged

`redis_cache_session(...).await.ok()` in `register` and `login` providers now uses `if let Err(error)` with `tracing::warn!()` instead of silently discarding errors.

#### Medium: Role Checking via Enum Instead of Raw Strings

`invite_member`, `remove_organization_member`, and `update_organization_member_role` in `providers/web_app/organization.rs` compared `membership.role` against raw strings like `"owner"`. Now uses `membership.get_role()` which returns the typed `MemberRole` enum.

#### Low: `_existing` Variable Naming

`update_category` in `services/cms/article_category.rs` used `_existing` (suggesting unused) but actually referenced it later. Renamed to `existing`.

#### Low: Event Enums Missing Derives

`EventType`, `EventVisibility`, `SignupStatus` in `enums/shared/event.rs` were missing `Clone`, `Debug`, `Serialize`, `Deserialize` derives, making them unusable over the wire. Now consistent with all other shared enums.

#### Low: `avatar_url` Always `None`

`get_current_user` in `providers/shared/auth.rs` hardcoded `avatar_url: None` instead of using `user.avatar_url`. Fixed.

---

## Fixes — Known Issues Resolution (6 February 2026, Part 2)

Resolved the remaining known issues identified during prior code review.

### Staff Authorization for CMS Endpoints

**Migration:** `2026-02-06-000001_add_is_staff_to_users` adds `is_staff BOOLEAN NOT NULL DEFAULT false` to the `users` table.

**Schema / Models:**
- `schema.rs` — added `is_staff -> Bool` to the `users` table definition.
- `models/user.rs` — `User` struct gains `pub is_staff: bool`; `UserUpdate` gains `pub is_staff: Option<bool>`.

**Session Caching:**
- `redis.rs` — `CachedSession` gains `#[serde(default)] pub is_staff: bool` (backward-compatible with existing cached sessions that lack the field).
- `http/middleware.rs` — `ValidatedSession` gains `pub is_staff: bool`. Both the Redis cache path and the Postgres fallback path now populate `is_staff`.
- `providers/shared/auth.rs` — `register` and `login` include `is_staff: user.is_staff` in the `CachedSession`.

**Authorization:**
- `http/middleware.rs` — new `AuthSession::require_staff()` method returns `ServerFnError` if `!session.is_staff`.
- All CMS providers (`providers/cms/article.rs`, `article_category.rs`, `article_tag.rs`, `media.rs`) changed from `auth.require_auth()` to `auth.require_staff()`.

### Transactional Deletes

- **`delete_article`** (`services/cms/article.rs`) — wrapped in `connection.transaction()`. Explicitly deletes `articles_tags` rows, then `article_revisions`, then the article. `ON DELETE CASCADE` (already in the initial migration) provides a safety net.
- **`delete_tag`** (`services/cms/article_tag.rs`) — wrapped in `connection.transaction()`. Deletes `articles_tags` links, then the tag.

### `delete_media` — DB-First with MinIO Retry

`services/cms/media.rs` — deletes the DB record first, then retries MinIO deletion up to 3 times (`MINIO_DELETE_MAX_RETRIES`). On retry failure, logs an error about an orphaned S3 object but returns `Ok` (the DB record is already removed). Rationale: an orphaned file in S3 is preferable to a DB record pointing at a deleted file.

### `invite_member` Duplicate Check & Service Extraction

New service function `create_invitation(org_id, email, role, invited_by)` in `services/shared/organization.rs`:
- Checks for an existing pending (non-expired) invitation to the same email/org → returns an error if found.
- Checks if the user is already a member of the organization → returns an error if found.
- Deletes expired pending invitations for the same email/org before inserting a new one.

`providers/web_app/organization.rs` — `invite_member` now delegates to `create_invitation` instead of containing inline Diesel queries.

### `get_organization_members` Service Extraction

New service function `get_members_with_user_info(org_id)` in `services/shared/organization.rs`:
- Returns `Vec<(OrganizationMember, String, String, String)>` (member + email, first name, last name) via an inner join with `users`.

`providers/web_app/organization.rs` — `get_organization_members` now delegates to this service function.

### `CreateOrganizationRequest.description` Passed Through

- `providers/web_app/organization.rs` — `create_organization` now passes `request.description` to the service.
- `services/shared/organization.rs` — `create_organization` accepts `description: Option<String>` and calls `new_org.set_description(desc)` when present.
- `models/organization.rs` — added `set_description()` method to `NewOrganization`.

### Unified `build_public_article_responses`

`services/shared/article.rs` — replaced the single-article `build_public_article_response` and the batch `batch_build_public_article_responses` with a single unified function:
```rust
pub async fn build_public_article_responses(articles: &[Article]) -> Result<Vec<PublicArticleResponse>, ServerFnError>
```
Both `get_published_article_by_slug` and `list_published_articles` now call this single batch function. The single-article case passes a one-element slice.

---

## Implementation Order

All CMS content phases (A–F) are complete. Remaining work:

1. **Chat enums** (Phase A) — `enums/shared/chat.rs`
2. **Chat models** (Phase B) — `models/chat_conversation.rs`, `chat_message.rs`, `chat_participant.rs`
3. **Chat interfaces** (Phase C) — `interfaces/support/chat.rs`
4. **Chat services** (Phase D) — `services/support/chat.rs`
5. **Chat providers** (Phase E) — `providers/support/chat.rs`
6. **Media upload wiring** — Replace placeholder with actual file upload flow