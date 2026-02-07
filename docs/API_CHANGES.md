# API Changes

> **Last Updated**: 6 February 2026

Tracks implementation of the support chat as well as assorted API issues. Each item is marked with its completion status.

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
| **Org member operations use full list scan** | Low | `remove_organization_member` and `update_organization_member_role` load all org members to find one. Add a direct `get_member_by_id()` service. |
| **No pagination on `get_organization_members`** | Low | Returns all members with no limit. Add `page`/`per_page` parameters. |
| **Sequential MinIO presigned URL generation** | Low | `list_media` generates presigned URLs one at a time. Use `try_join_all()` for concurrency. |
| **Event shift no time validation** | Low | `NewEventShift::new()` doesn't validate `end_time > start_time`. |
| **No `update_organization` input validation** | Low | `OrganizationUpdate` fields are not validated for length before DB write. |
| **Periodic cleanup not scheduled** | Medium | `cleanup_expired_sessions()` and `cleanup_expired_reset_tokens()` exist but are never called. Need a background worker or startup task. |
| **Hardcoded timezone defaults** | Low | New users and organizations default to `"America/New_York"`. Has existing TODOs. |
| **Hardcoded subscription defaults** | Low | New organizations always get `Events` subscription. Has existing TODO. |

---

## Implementation Order (remaining)

1. **Chat enums** (Phase A) — `enums/shared/chat.rs`
2. **Chat models** (Phase B) — `models/chat_conversation.rs`, `chat_message.rs`, `chat_participant.rs`
3. **Chat interfaces** (Phase C) — `interfaces/support/chat.rs`
4. **Chat services** (Phase D) — `services/support/chat.rs`
5. **Chat providers** (Phase E) — `providers/support/chat.rs`
6. **Media upload wiring** — Replace placeholder with actual file upload flow
7. **Email integration** — Wire AWS SES for invitations and password reset emails