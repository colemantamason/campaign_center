# API Changes

> **Last Updated**: 6 February 2026 (audit v2, Phase 4 complete)

Tracks API issues, audit findings, and improvement plans. Each item is marked with its completion status.

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Implemented |
| ⬜ | Not started |

---

## Support Chat (deferred)

### Phase A: Enums (shared)

| File | Types | Status |
|------|-------|--------|
| `enums/shared/chat.rs` | `ConversationStatus { Open, Assigned, Resolved, Closed }`, `ChatParticipantRole { Customer, Agent }`, `ChatMessageType { Text, System, Attachment }` | ⬜ |

- ⬜ Chat enums not yet registered

### Phase B: Models (server-only)

| File | Types | Status |
|------|-------|--------|
| `models/chat_conversation.rs` | `ChatConversation`, `NewChatConversation`, `ChatConversationUpdate` | ⬜ |
| `models/chat_message.rs` | `ChatMessage`, `NewChatMessage` | ⬜ |
| `models/chat_participant.rs` | `ChatParticipant`, `NewChatParticipant` | ⬜ |

- ⬜ Chat models not yet registered
- Note: Chat tables already exist in the database migration and `schema.rs` — only the Rust model structs are needed

### Phase C: Interfaces (DTOs)

| File | Types | Status |
|------|-------|--------|
| `interfaces/support/chat.rs` | `CreateConversationRequest`, `SendMessageRequest`, `ConversationResponse`, `MessageResponse` | ⬜ |

- ⬜ Support chat interfaces not yet created

### Phase D: Services (server-only, business logic)

| File | Functions | Status |
|------|-----------|--------|
| `services/support/chat.rs` | Create conversation, send message, list conversations (by status, by user), mark messages read, assign agent, resolve/close, list messages for conversation | ⬜ |

### Phase E: Providers (server functions / API endpoints)

| File | Endpoints | Status |
|------|-----------|--------|
| `providers/support/chat.rs` | `create_conversation`, `send_message`, `list_conversations`, `get_conversation_messages`, `assign_agent`, `resolve_conversation` | ⬜ |

---

## Known Issues & Future Improvements

Issues are grouped by category and ordered by priority within each group. Tackle **Code Duplication** first — the helper functions created there will simplify many of the fixes in later categories.

---

### A. Code Duplication — Helper Functions

These are the highest-value refactors. Each one eliminates a pattern that is repeated 10–40+ times across the codebase.

| # | Item | Occurrences | Plan |
|---|------|-------------|------|
| A1 | **Repetitive Postgres `.map_err` boilerplate** | ~40+ | ✅ Created `postgres_error`, `redis_error`, `minio_error` helpers in `error.rs`. Applied across all service files, `postgres.rs`, `redis.rs`, and `minio.rs`. |
| A2 | **Repetitive `ServerFnError::new(error.to_string())` in providers** | ~30+ | ✅ Updated all providers to use `?` directly, leveraging the existing `impl From<AppError> for ServerFnError` which serializes errors as JSON. Clients now receive structured error types instead of display strings. |
| A3 | **Article ownership check duplicated in 5 CMS providers** | 5 | ✅ Extracted `require_article_ownership(article_id, user_id)` helper in `providers/cms/article.rs`. Applied to `update_article`, `publish_article`, `delete_article`, `auto_save`, and `restore_revision`. |
| A4 | **Membership check + role guard duplicated in 6 org providers** | 6 | ✅ Extracted `require_membership(org_id, user_id)` and `require_membership_with_role(org_id, user_id, min_role)` helpers in `providers/web_app/organization.rs`. Applied to `set_active_organization`, `get_organization`, `get_organization_members`, `invite_member`, `remove_organization_member`, and `update_organization_member_role`. |
| A5 | **Pagination defaults repeated in every list provider** | 5+ | ✅ Created `PaginationParams::resolve(page, per_page)` in `interfaces/shared/pagination.rs`. Applied to `list_articles`, `list_published_articles`, and `list_media` providers. |
| A6 | **Enum `as_str` / `from_str` / `display_name` / `Display` boilerplate** | All 10+ enums | ✅ Created `define_enum!` macro in `enums.rs` that generates all four items from `(VariantName, "db_value", "Display Name")` tuples. Applied to all 10 enums across 7 files, cutting ~500 lines of repetitive code. Enums with extra methods (e.g. `MemberRole::can_manage`) or derives (e.g. `SubscriptionType: Eq, Hash`) keep those in separate `impl` blocks or `#[derive]` attributes passed through the macro. |
| A7 | **`batch_build_article_responses` duplicated in two places** | 2 | ✅ Moved `batch_build_article_responses` and added `build_article_response` (single-article convenience wrapper) to `services/cms/article.rs`. The CMS provider now imports from the service layer instead of defining its own copy. `batch_build_public_article_responses` in `services/shared/article.rs` remains separate (different output type) but shares the same batch-load helpers. |
| A8 | **Single-article `build_article_response` makes N+1 queries** | 1 | ✅ Replaced the N+1 `build_article_response` in the CMS provider with `build_article_response` in the service layer, which delegates to `batch_build_article_responses` with a single-element vec. All single-article endpoints now use batch-loading. |

---

### B. Bugs

| # | Item | Severity | Details | Fix |
|---|------|----------|---------|-----|
| B1 | **`set_active_organization` parameter named `organization_id` but receives `membership_id`** | Medium | In `services/shared/session.rs`, the parameter `organization_id: Option<i32>` actually stores a membership ID (`set_active_organization_service(session.session_id, Some(membership.id))`). The `SessionUpdate` field is correctly named `active_organization_membership_id`. | ✅ Renamed parameter to `membership_id`. |
| B2 | **`OrganizationUpdate` can't set nullable fields to `NULL`** | Medium | `OrganizationUpdate` uses `Option<String>` for nullable DB columns (`description`, `avatar_url`, `website_url`, `email`, `phone_number`, address fields). With Diesel's `AsChangeset`, `None` means "don't update" — there's no way to clear a value once set. `ArticleUpdate` correctly uses `Option<Option<String>>` for the same pattern. | ✅ Changed all nullable-column fields in `OrganizationUpdate` to `Option<Option<String>>`. Applied the same fix to `UserUpdate` (`phone_number`, `avatar_url`) and `OrganizationMemberUpdate` (`last_active_at`). Added `validate_nested_optional_string` helper for validating `Option<Option<String>>` fields. |
| B3 | **`create_organization` not wrapped in a transaction** | Medium | Inserts the organization, then inserts the owner membership in separate queries. If the membership insert fails, an ownerless organization is left in the DB. | ✅ Wrapped both inserts in a `connection.transaction()` block. |
| B4 | **`create_invitation` stores email without lowercase normalization** | Low | `create_invitation` validates the email but stores it with original casing. User registration normalizes to lowercase (`email.to_lowercase()`). The existing-member lookup in the same function does use `.to_lowercase()`, so the comparison works — but the stored invitation email casing is inconsistent with the users table. | ✅ Added `.to_lowercase()` to the email at the start of `create_invitation`, before any lookups or storage. |
| B5 | **`reset_password` silently discards session deletion errors** | Low | Uses `delete_all_user_sessions(user_id, None).await.ok()` which swallows errors entirely. `change_password` in contrast logs failures with `tracing::warn!`. | ✅ Replaced `.ok()` with `tracing::warn!` log on error. |
| B6 | **`delete_article` manually deletes from join tables despite `ON DELETE CASCADE`** | Low | The migration defines `ON DELETE CASCADE` on `articles_tags` and `article_revisions` foreign keys. The `delete_article` service manually deletes from both tables inside a transaction before deleting the article itself. The manual deletion is redundant. | ✅ Simplified `delete_article` to delete only from the `articles` table. Removed the transaction wrapper and manual `articles_tags`/`article_revisions` deletes; the DB cascade handles cleanup. |
| B7 | **`invitations` unique constraint blocks re-inviting previously accepted users** | Low | The DB has `UNIQUE(organization_id, email)` on invitations. The service only checks for and deletes *pending* expired invitations. If a user was previously invited and accepted, their accepted invitation row still occupies the unique slot — a new invitation for the same email to the same org will hit a `UniqueViolation`. | ✅ Added deletion of any non-pending (accepted/expired) invitations for the same email + org before inserting a new invitation, clearing the unique constraint. |

---

### C. Consistency Fixes

| # | Item | Details | Fix |
|---|------|---------|-----|
| C1 | **Inconsistent `#[derive]` traits on enums** | Some enums derive `Debug` + `Copy` (e.g., `ArticleType`, `ArticleStatus`, `Platform`, `OrganizationType`, `InvitationStatus`) while others omit one or both (e.g., `EventType`, `MemberRole`, `SignupStatus`, `NotificationType`, `EventVisibility`). All are fieldless and can safely derive both. | ✅ Added `Copy` and `Debug` to all enums that were missing them: `EventType`, `EventVisibility`, `SignupStatus`, `MemberRole`, `SubscriptionType`, `NotificationType`. |
| C2 | **Inconsistent membership check style in org providers** | Some providers use `if membership.is_none() { return Err(...); }` (e.g., `get_organization`, `get_organization_members`) while others use `.ok_or_else(\|\| ServerFnError::new(...))?` (e.g., `set_active_organization`, `invite_member`). | ✅ Resolved by A4: all providers now use `require_membership()` / `require_membership_with_role()` helpers. |
| C3 | **Redundant `updated_at` setting in article services** | The migration defines an `update_articles_updated_at` trigger that sets `updated_at = NOW()` on every update. But `update_article`, `publish_article`, `auto_save_article`, and `restore_revision` also set `updated_at` manually. Other tables (organizations, categories) rely solely on the trigger. | ✅ Removed manual `updated_at` setting from `update_article`, `publish_article`, `auto_save_article`, and `restore_revision`. All tables now rely on the DB trigger consistently. |
| C4 | **`create_invitation` role not validated at service layer** | The provider (`invite_member`) validates the role string via `MemberRole::from_str()`, but the service function (`create_invitation`) accepts a raw `String` role and stores it without validation. Any direct caller of the service could insert an invalid role. | ✅ Changed `create_invitation` to accept `MemberRole` instead of `String`. The service now converts to string internally via `role.as_str()`. Updated the `invite_member` provider to pass the typed enum directly. |
| C5 | **`update_organization` has no provider endpoint** | The service function exists and validates input, but there is no provider/endpoint to expose it. Organizations can be created but not updated via the API. | ✅ Added `update_organization` provider in `providers/web_app/organization.rs` with `require_membership_with_role(Admin)` auth check. Created `UpdateOrganizationRequest` DTO in `interfaces/web_app/organization.rs`. |
| C6 | **`OrganizationResponse` omits most organization fields** | Only includes `id`, `name`, `slug`, `description`. Missing: `avatar_url`, `website_url`, `email`, `phone_number`, address fields, `timezone`, `organization_type`, `subscriptions`, `created_at`, `updated_at`. | ✅ Expanded `OrganizationResponse` with all fields (`organization_type`, `avatar_url`, `website_url`, `email`, `phone_number`, address fields, `timezone`, `created_at`, `updated_at`). Added `From<Organization>` impl for clean conversion. Updated all construction sites. |
| C7 | **`UserAccountResponse` doesn't include email** | `AuthResponse` includes `email`, but `UserAccountResponse` (from `get_current_user`) does not. If the client needs the email after initial auth (e.g., for a settings page), there's no dedicated endpoint for it. | ✅ Added `email` field to `UserAccountResponse`. Updated `get_current_user` provider to populate it. |
| C8 | **`get_members_with_user_info` returns unnamed tuple** | Returns `Vec<(OrganizationMember, String, String, String)>` — the three strings are `email`, `first_name`, `last_name` but this is not self-documenting. | ✅ Created `MemberWithUserInfo` struct in `services/shared/organization.rs` with named fields (`member`, `email`, `first_name`, `last_name`). Updated `get_members_with_user_info` and the `get_organization_members` provider. |
| C9 | **MinIO reads `MINIO_ENDPOINT` and `MINIO_PUBLIC_URL` env vars on every presigned URL request** | `get_minio_presigned_url` reads both env vars per call. Other services (Postgres, Redis, MinIO client) cache their config at initialization. | ✅ Cached both URLs in `OnceLock` statics during `initialize_minio_client()`. Added `get_minio_endpoint()` and `get_minio_public_url()` getter functions. `get_minio_presigned_url` now reads from the cache instead of `env::var`. |

---

### D. Deferred Items (to be addressed as features are built)

| # | Item | Severity | Notes |
|---|------|----------|-------|
| D1 | **TOCTOU slug uniqueness** | Low | Slug check-then-insert is not atomic for articles, tags, categories, and orgs. DB unique constraints catch duplicates, but error messages fall back to generic `UniqueViolation`. Consider catching `UniqueViolation` and returning friendly "slug already taken" errors. |
| D2 | **`SESSION_COOKIE_DOMAIN` placeholder** | Medium | Hardcoded to `.domain.com` — must be replaced before production deployment. Has existing TODO. |
| D3 | **No rate limiting** | Medium | Auth endpoints (`register`, `login`, `change_password`, `request-password-reset`) have no brute-force protection. Already on roadmap (Phase 2). |
| D4 | **No CSRF protection** | Medium | Cookie-based sessions use `SameSite=Lax` but no CSRF token validation. Already on roadmap (Phase 2). |
| D5 | **No scheduled article publishing** | Low | `scheduled_publish_at` field exists but nothing checks/triggers it. Needs a background worker. |
| D6 | **No email sending** | Medium | Invitations and password reset tokens are created but no emails are dispatched. Requires AWS SES integration. Password reset tokens are logged in development. |
| D7 | **Unused `get_article_by_slug` service** | Low | Defined in `services/cms/article.rs` but not called by any provider. Remove or wire up when needed. |
| D8 | **No pagination on `get_organization_members`** | Low | Returns all members with no limit. Add `page`/`per_page` parameters. |
| D9 | **Periodic cleanup not scheduled** | Medium | `cleanup_expired_sessions()` and `cleanup_expired_reset_tokens()` exist but are never called. Need a background worker or startup task. |
| D10 | **Hardcoded timezone defaults** | Low | New users and organizations default to `"America/New_York"`. Has existing TODOs. |
| D11 | **Hardcoded subscription defaults** | Low | New organizations always get `Events` subscription. Has existing TODO. |
| D12 | **`validate_media_file` trusts client-provided MIME type** | Medium | MIME type check relies on the client-provided string. A malicious user could claim `mime_type: "image/png"` for a non-image. After real upload is wired, verify MIME via file magic bytes. |
| D13 | **Media upload placeholder** | Medium | The `upload_media` provider currently passes empty bytes — actual file upload needs to be wired via Dioxus multipart upload support or a presigned URL flow where the client uploads directly to MinIO. |
| D14 | **No failed login attempt logging** | Low | `authenticate_user` doesn't log failed login attempts. Useful for security auditing and future rate limiting integration. Add `tracing::warn!` on `InvalidCredentials` with the email (not the password). |

---

### Implementation Order

**Phase 1 — Helpers & Quick Fixes** ✅ (completed):

1. ✅ **A1: Postgres/Redis/MinIO error helpers** — created `postgres_error`, `redis_error`, `minio_error` in `error.rs`, applied across all services
2. ✅ **A2: Use `From<AppError>` in providers** — updated all providers to use `?` instead of `.map_err(|e| ServerFnError::new(e.to_string()))`
3. ✅ **A5: Pagination helper** — created `PaginationParams::resolve()` in `interfaces/shared/pagination.rs`
4. ✅ **B1: Fix `set_active_organization` parameter name** — renamed to `membership_id`
5. ✅ **B5: Log `reset_password` session deletion errors** — replaced `.ok()` with `tracing::warn!`

**Phase 2 — Consistency & Correctness** ✅ (completed):

6. ✅ **B2: Fix `OrganizationUpdate` Option wrapping** — changed nullable fields to `Option<Option<T>>` in `OrganizationUpdate`, `UserUpdate`, and `OrganizationMemberUpdate`; added `validate_nested_optional_string` helper
7. ✅ **B3: Wrap `create_organization` in transaction** — wrapped org + owner membership inserts in `connection.transaction()`
8. ✅ **B4: Normalize invitation email to lowercase** — added `.to_lowercase()` at start of `create_invitation`
9. ✅ **C1: Standardize enum derives** — added `Copy` + `Debug` to `EventType`, `EventVisibility`, `SignupStatus`, `MemberRole`, `SubscriptionType`, `NotificationType`
10. ✅ **C3: Pick `updated_at` strategy** — removed manual `updated_at` setting from all article services; all tables now rely on DB trigger
11. ✅ **C4: Type `create_invitation` role parameter** — changed from `String` to `MemberRole`; updated provider caller
12. ✅ **C9: Cache MinIO env vars at init** — cached `MINIO_ENDPOINT` and `MINIO_PUBLIC_URL` in `OnceLock` statics during initialization

**Phase 3 — Provider-Level Improvements** ✅ (completed):

13. ✅ **A3: Extract article ownership helper** — created `require_article_ownership(article_id, user_id)` in `providers/cms/article.rs`; applied to 5 CMS providers
14. ✅ **A4: Extract membership check helper** — created `require_membership(org_id, user_id)` and `require_membership_with_role(org_id, user_id, min_role)` in `providers/web_app/organization.rs`; applied to 6 org providers
15. ✅ **A7/A8: Consolidate article response building** — moved `batch_build_article_responses` + added `build_article_response` to `services/cms/article.rs`; single-article endpoints now use batch-loading (eliminates N+1)
16. ✅ **C2: Standardize membership check style** — resolved by A4 helpers
17. ✅ **C5: Add `update_organization` provider** — added endpoint with `UpdateOrganizationRequest` DTO and Admin role guard
18. ✅ **C6: Expand `OrganizationResponse` fields** — added all organization fields + `From<Organization>` impl
19. ✅ **C7: Add `email` to `UserAccountResponse`** — added `email` field and populated in `get_current_user`

**Phase 4 — Larger Refactors** ✅ (completed):

20. ✅ **A6: Enum macro** — created `define_enum!` macro in `enums.rs`; applied to all 10 enums across 7 files
21. ✅ **C8: Named struct for `get_members_with_user_info`** — created `MemberWithUserInfo` struct; updated service and provider
22. ✅ **B6: Simplify `delete_article`** — removed redundant manual cascade deletes and transaction wrapper
23. ✅ **B7: Fix invitation re-invite constraint** — added deletion of non-pending invitations before insert

**Deferred** (Phase 2 roadmap / feature-dependent):

24. Items D1–D14 as listed above

---

## Chat Implementation Order (deferred)

1. **Chat enums** (Phase A) — `enums/shared/chat.rs`
2. **Chat models** (Phase B) — `models/chat_conversation.rs`, `chat_message.rs`, `chat_participant.rs`
3. **Chat interfaces** (Phase C) — `interfaces/support/chat.rs`
4. **Chat services** (Phase D) — `services/support/chat.rs`
5. **Chat providers** (Phase E) — `providers/support/chat.rs`