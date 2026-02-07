# API Changes

> **Last Updated**: 6 February 2026

Tracks implementation of the support chat as well as assorted API issues and audit findings. Each item is marked with its completion status.

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

## Recently Completed

| Item | Details |
|------|---------|
| ✅ **Input length validation** | Centralized validation module (`services/shared/validation.rs`) with `validate_required_string`, `validate_optional_string`, `validate_slug`, `validate_optional_slug`, and `validate_max_length`. Constants match DB column limits. Applied to: article create/update, category create/update, tag create, organization create, user registration, media upload. |
| ✅ **Password reset flow** | Full implementation: model (`models/password_reset_token.rs`), service (`services/shared/password_reset.rs`), interfaces (`interfaces/shared/password_reset.rs`), and providers (`providers/shared/password_reset.rs`). Endpoints: `request-password-reset`, `reset-password`, `validate-reset-token`. Invalidates existing tokens on new request. Email sending still TODO (logged in dev). |
| ✅ **Media file validation** | `validate_media_file()` in validation module. Checks: file size > 0, max 50 MB, allowlisted MIME types (JPEG, PNG, GIF, WebP, SVG, AVIF, PDF, MP4, WebM), dangerous extension blocklist. |
| ✅ **Security: `is_staff` removed from RegisterRequest** | Users can no longer self-register as staff. `is_staff` is hardcoded to `false` during registration. Staff promotion must be done via admin operation. |
| ✅ **Session invalidation on credential change** | `change_password` and `reset_password` now call `delete_all_user_sessions()` to invalidate all sessions after a credential change. |
| ✅ **Invitation role validation** | `invite_member` provider now validates the role string against `MemberRole::from_str()` and prevents inviting as `Owner`. |
| ✅ **Organization avatar_url fix** | `get_current_user` now returns the actual `organization.avatar_url` instead of hardcoded `None`. |
| ✅ **`publish_article` wrapped in transaction** | The revision insert + status update are now atomic. |
| ✅ **`sync_article_tags` wrapped in transaction** | The delete + insert for tag associations are now atomic. |
| ✅ **Org member operations used full list scan** | `remove_organization_member` and `update_organization_member_role` now use a direct `get_member_by_id()` service instead of loading all members. Provider also validates `organization_id` ownership. |
| ✅ **Sequential MinIO presigned URL generation** | `list_media` now uses `futures::try_join_all()` to generate presigned URLs concurrently instead of one at a time. |
| ✅ **Event shift no time validation** | `NewEventShift::new()` now returns `Result<Self, AppError>` and validates `end_time > start_time`. |
| ✅ **`update_organization` input validation** | `update_organization` service now validates all string field lengths against DB column limits before writing. Also rejects empty name. |

---

## Known Issues & Future Improvements

### Media Upload Placeholder

The `upload_media` provider currently passes empty bytes — actual file upload needs to be wired via Dioxus multipart upload support or a presigned URL flow where the client uploads directly to MinIO. The `validate_media_file` function validates the declared metadata, but actual uploaded data is currently a 0-byte placeholder.

### Deferred Items (to be addressed as features are built)

| Item | Severity | Notes |
|------|----------|-------|
| **TOCTOU slug uniqueness** | Low | Slug check-then-insert is not atomic for articles, tags, categories, and orgs. DB unique constraints catch duplicates, but error messages fall back to generic `UniqueViolation`. Consider catching `UniqueViolation` and returning friendly errors. |
| **`SESSION_COOKIE_DOMAIN` placeholder** | Medium | Hardcoded to `.domain.com` — must be replaced before production deployment. Has existing TODO. |
| **No rate limiting** | Medium | Auth endpoints (`register`, `login`, `change_password`, `request-password-reset`) have no brute-force protection. Already on roadmap (Phase 2). |
| **No CSRF protection** | Medium | Cookie-based sessions use `SameSite=Lax` but no CSRF token validation. Already on roadmap (Phase 2). |
| **No scheduled article publishing** | Low | `scheduled_publish_at` field exists but nothing checks/triggers it. Needs a background worker. |
| **No email sending** | Medium | Invitations and password reset tokens are created but no emails are dispatched. Requires AWS SES integration. Password reset tokens are logged in development. |
| **Unused `get_article_by_slug` service** | Low | Defined in `services/cms/article.rs` but not called by any provider. Remove or wire up when needed. |
| **No pagination on `get_organization_members`** | Low | Returns all members with no limit. Add `page`/`per_page` parameters. |
| **Periodic cleanup not scheduled** | Medium | `cleanup_expired_sessions()` and `cleanup_expired_reset_tokens()` exist but are never called. Need a background worker or startup task. |
| **Hardcoded timezone defaults** | Low | New users and organizations default to `"America/New_York"`. Has existing TODOs. |
| **Hardcoded subscription defaults** | Low | New organizations always get `Events` subscription. Has existing TODO. |
| **`batch_reorder_categories` uses raw SQL string interpolation** | Medium | `batch_reorder_categories` builds a raw SQL query via `format!()`. While inputs are typed as `i32`, this bypasses Diesel's parameterized query protection. Refactor to use parameterized updates in a loop within a transaction. |
| **`batch_reorder_categories` has no scope check** | Low | The raw SQL update applies to any category IDs passed. No validation that all IDs belong to the same `article_type` (blog vs. support). |
| **CMS article mutations lack ownership check** | Medium | All CMS article mutations (`update`, `publish`, `delete`, `auto_save`) only check `require_staff()`. Any staff user can modify any other staff user's articles. Consider adding an ownership or editor role check. |
| **`restore_revision` does not invalidate Redis cache** | Medium | Restoring a revision sets status to `Draft` but doesn't clear the Redis cache for the slug. If the article was previously published, stale content is served until 24h TTL expires. |
| **`auto_save_article` does not update `updated_at`** | Low | Only sets `articles::content` but never updates `articles::updated_at`, so the "last modified" timestamp becomes stale after auto-saves. |
| **`publish_article` always overwrites `published_at`** | Low | Re-publishing an edited article sets `published_at` to now, losing the original publication date. Should only set `published_at` if it is currently `None`. |
| **Orphaned MinIO files on DB insert failure** | Medium | `upload_media` uploads to MinIO first, then inserts the DB record. If the DB insert fails, the MinIO object is orphaned. Add a compensating delete in the error path. |
| **`create_invitation` does not validate email format** | Medium | The `email` parameter is stored without calling `validate_email()`. The function exists in `auth.rs` but is not invoked for invitations. |
| **Redis session update is not atomic (read-modify-write race)** | Medium | `update_redis_cached_session_active_organization_membership_id` reads, modifies, and writes back. A concurrent sliding session extension could overwrite the change. Also, parameter is named `organization_id` but receives a `membership_id`. |
| **`delete_all_user_sessions` uses sequential Redis invalidation** | Low | Each session token's Redis cache is invalidated in a serial loop. For users with many sessions, this creates N sequential round-trips. Use a Redis pipeline or multi-key `DEL`. |
| **`validate_media_file` trusts client-provided MIME type** | Medium | MIME type check relies on the client-provided string. A malicious user could upload an executable with `mime_type: "image/png"`. After real upload is wired, verify MIME via file magic bytes. Also add `.html`, `.js`, `.htm` to dangerous extensions. |
| **`change_password` invalidates the caller's own session** | Low | `delete_all_user_sessions()` invalidates all sessions including the current one. Consider excluding the current session token so the user isn't logged out. Also `.ok()` swallows session deletion errors. |
| **Tag filter loads all article IDs into memory** | Low | When filtering public articles by tag, all matching `article_id`s are loaded into a `Vec<i32>` then passed to `eq_any()`. Use a subquery or JOIN instead for large datasets. |

---

## Implementation Order (remaining)

1. **Chat enums** (Phase A) — `enums/shared/chat.rs`
2. **Chat models** (Phase B) — `models/chat_conversation.rs`, `chat_message.rs`, `chat_participant.rs`
3. **Chat interfaces** (Phase C) — `interfaces/support/chat.rs`
4. **Chat services** (Phase D) — `services/support/chat.rs`
5. **Chat providers** (Phase E) — `providers/support/chat.rs`
6. **Media upload wiring** — Replace placeholder with actual file upload flow
7. **Email integration** — Wire AWS SES for invitations and password reset emails