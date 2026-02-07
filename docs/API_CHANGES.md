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

| # | Item | Severity | Notes |
|---|------|----------|-------|
| D1 | **TOCTOU slug uniqueness** | Low | Slug check-then-insert is not atomic for articles, tags, categories, and orgs. DB unique constraints catch duplicates, but error messages fall back to generic `UniqueViolation`. Consider catching `UniqueViolation` and returning friendly "slug already taken" errors. |
| D2 | **`SESSION_COOKIE_DOMAIN` placeholder** | Medium | Hardcoded to `.domain.com` — must be replaced before production deployment. Has existing TODO. |
| D3 | **No rate limiting** | Medium | Auth endpoints (`register`, `login`, `change_password`, `request-password-reset`) have no brute-force protection. Already on roadmap (Phase 2). |
| D4 | **No CSRF protection** | Medium | Cookie-based sessions use `SameSite=Lax` but no CSRF token validation. Already on roadmap (Phase 2). |
| D5 | **No scheduled article publishing** | Low | `scheduled_publish_at` field exists but nothing checks/triggers it. Needs a background worker. |
| D6 | **No email sending** | Medium | Invitations and password reset tokens are created but no emails are dispatched. Requires AWS SES integration. Password reset tokens are logged in development. |
| D7 | **Unused `get_article_by_slug` service** | Low | Defined in `services/cms/article.rs` but not called by any provider. Remove or wire up when needed. |
| D9 | **Periodic cleanup not scheduled** | Medium | `cleanup_expired_sessions()` and `cleanup_expired_reset_tokens()` exist but are never called. Need a background worker or startup task. |
| D10 | **Hardcoded timezone defaults** | Low | New users and organizations default to `"America/New_York"`. Has existing TODOs. |
| D11 | **Hardcoded subscription defaults** | Low | New organizations always get `Events` subscription. Has existing TODO. |
| D12 | **`validate_media_file` trusts client-provided MIME type** | Medium | MIME type check relies on the client-provided string. A malicious user could claim `mime_type: "image/png"` for a non-image. After real upload is wired, verify MIME via file magic bytes. |
| D13 | **Media upload placeholder** | Medium | The `upload_media` provider currently passes empty bytes — actual file upload needs to be wired via Dioxus multipart upload support or a presigned URL flow where the client uploads directly to MinIO. |

---

## Chat Implementation Order (deferred)

1. **Chat enums** (Phase A) — `enums/shared/chat.rs`
2. **Chat models** (Phase B) — `models/chat_conversation.rs`, `chat_message.rs`, `chat_participant.rs`
3. **Chat interfaces** (Phase C) — `interfaces/support/chat.rs`
4. **Chat services** (Phase D) — `services/support/chat.rs`
5. **Chat providers** (Phase E) — `providers/support/chat.rs`