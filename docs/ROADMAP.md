# Campaign Center - Development Roadmap

> **Last Updated**: February 2026  
> **Target MVP Launch**: Q2 2026  
> **Full Platform Launch**: Q4 2026

---

## Table of Contents

1. [Overview](#overview)
2. [Phase 0: Foundation](#phase-0-foundation-current)
3. [Phase 1: Events MVP](#phase-1-events-mvp)
4. [Phase 2: Growth Features](#phase-2-growth-features)
5. [Phase 3: Campaign Platform](#phase-3-campaign-platform)
6. [Phase 4: Advanced Features](#phase-4-advanced-features)
7. [Ongoing: Infrastructure & Operations](#ongoing-infrastructure--operations)

---

## Overview

### Strategic Approach

1. **Launch Events MVP** under separate branding (e.g., "RallyUp" or similar)
2. **Build infrastructure** alongside events (auth, payments, emails, etc.)
3. **Add campaign features** progressively
4. **Merge platforms** once feature parity achieved

### Timeline Summary

```
2026 Q1      Q2        Q3        Q4        2027 Q1
  │          │         │         │          │
  ├──────────┼─────────┼─────────┼──────────┤
  │ Phase 0  │Phase 1  │ Phase 2 │ Phase 3  │ Phase 4
  │Foundation│Events   │ Growth  │ Campaign │ Advanced
  │          │MVP      │Features │ Platform │ Features
  │          │         │         │          │
  └──────────┴─────────┴─────────┴──────────┘
             ▲                   ▲
             │                   │
        Events MVP          Full Platform
         Launch              Launch
```

---

## Phase 0: Foundation (Current)

**Duration**: Now - End of Q1 2026  
**Status**: 🟡 In Progress

### Objectives

- [x] Project scaffolding (packages, feature flags)
- [x] Basic web app structure (routes, sidebar, layout)
- [x] Shared UI component library (button, input, etc.)
- [x] Mock data and permission system
- [ ] Backend infrastructure setup
- [ ] Database schema and migrations
- [ ] Authentication system
- [ ] Development environment automation

### 0.1 Backend Infrastructure

| Task | Status | Priority |
|------|--------|----------|
| Set up Diesel with PostgreSQL | ⬜ Not Started | P0 |
| Create initial migrations (users, orgs, sessions) | ⬜ Not Started | P0 |
| Implement session management with Redis | ⬜ Not Started | P0 |
| Create database connection pooling | ⬜ Not Started | P0 |
| Set up environment configuration | ⬜ Not Started | P0 |

### 0.2 Authentication System

| Task | Status | Priority |
|------|--------|----------|
| User registration flow | ⬜ Not Started | P0 |
| Login/logout flows | ⬜ Not Started | P0 |
| Password hashing (Argon2) | ⬜ Not Started | P0 |
| Session creation/validation | ⬜ Not Started | P0 |
| Password reset flow | ⬜ Not Started | P1 |
| Email verification | ⬜ Not Started | P1 |

### 0.3 Organization Foundation

| Task | Status | Priority |
|------|--------|----------|
| Create organization flow | ⬜ Not Started | P0 |
| Organization settings page | ⬜ Not Started | P1 |
| Team member invitation flow | ⬜ Not Started | P1 |
| Role-based permissions | ⬜ Not Started | P0 |

### 0.4 Development Environment

| Task | Status | Priority |
|------|--------|----------|
| Docker Compose for local dev | ⬜ Not Started | P0 |
| Database seeding scripts | ⬜ Not Started | P1 |
| Hot reload configuration | ✅ Done | P0 |
| CI/CD pipeline (GitHub Actions) | ⬜ Not Started | P2 |

### Deliverables

- [ ] Working authentication (register, login, logout)
- [ ] Create organization and invite team members
- [ ] Permission-based route protection (real, not mock)
- [ ] Local development with Docker

---

## Phase 1: Events MVP

**Duration**: Q2 2026  
**Status**: ⬜ Not Started

### Objectives

- Core event CRUD functionality
- Public event discovery
- RSVP and attendee management
- Email notifications
- Co-hosting with other organizations
- Basic payment/ticketing

### 1.1 Event Management (Web App)

| Task | Status | Priority |
|------|--------|----------|
| Create event form (all fields) | ⬜ Not Started | P0 |
| Event list view with filters | ⬜ Not Started | P0 |
| Event detail/edit view | ⬜ Not Started | P0 |
| Event duplication | ⬜ Not Started | P2 |
| Event cancellation flow | ⬜ Not Started | P1 |
| Draft/publish workflow | ⬜ Not Started | P0 |
| Cover image upload | ⬜ Not Started | P1 |

### 1.2 Attendee Management (Web App)

| Task | Status | Priority |
|------|--------|----------|
| Attendee list view | ⬜ Not Started | P0 |
| Manual attendee add | ⬜ Not Started | P1 |
| Attendee status management | ⬜ Not Started | P0 |
| Check-in functionality | ⬜ Not Started | P1 |
| Export attendee list (CSV) | ⬜ Not Started | P1 |
| Waitlist management | ⬜ Not Started | P2 |

### 1.3 Event Discovery (Events Website)

| Task | Status | Priority |
|------|--------|----------|
| Homepage with featured events | ⬜ Not Started | P0 |
| Event search (location, date, type) | ⬜ Not Started | P0 |
| Event detail page | ⬜ Not Started | P0 |
| RSVP form | ⬜ Not Started | P0 |
| Organization profile page | ⬜ Not Started | P1 |
| Event embed widget | ⬜ Not Started | P2 |
| Share functionality | ⬜ Not Started | P1 |

### 1.4 Email Notifications

| Task | Status | Priority |
|------|--------|----------|
| AWS SES integration | ⬜ Not Started | P0 |
| RSVP confirmation email | ⬜ Not Started | P0 |
| Event reminder emails (24h, 1h) | ⬜ Not Started | P0 |
| Event update emails | ⬜ Not Started | P1 |
| Event cancellation email | ⬜ Not Started | P1 |
| Email template system | ⬜ Not Started | P0 |

### 1.5 Co-hosting

| Task | Status | Priority |
|------|--------|----------|
| Invite co-host organization | ⬜ Not Started | P1 |
| Accept/reject co-host invitation | ⬜ Not Started | P1 |
| Co-host permission management | ⬜ Not Started | P1 |
| Shared attendee access | ⬜ Not Started | P1 |
| Co-host branding on event page | ⬜ Not Started | P2 |

### 1.6 Ticketing & Payments

| Task | Status | Priority |
|------|--------|----------|
| Stripe integration (Connect) | ⬜ Not Started | P0 |
| Free vs. paid event toggle | ⬜ Not Started | P0 |
| Ticket types (general, VIP, etc.) | ⬜ Not Started | P1 |
| Checkout flow | ⬜ Not Started | P0 |
| Refund processing | ⬜ Not Started | P1 |
| Payout to organizations | ⬜ Not Started | P1 |

### 1.7 Marketing Website

| Task | Status | Priority |
|------|--------|----------|
| Landing page | ⬜ Not Started | P0 |
| Features page | ⬜ Not Started | P1 |
| Pricing page | ⬜ Not Started | P0 |
| Contact form | ⬜ Not Started | P2 |
| Blog (static initially) | ⬜ Not Started | P2 |

### Deliverables

- [ ] Fully functional event creation and management
- [ ] Public event discovery website
- [ ] RSVP and ticketing system
- [ ] Email notification system
- [ ] Co-hosting capability
- [ ] Marketing website
- [ ] Ready for limited public launch

---

## Phase 2: Growth Features

**Duration**: Q3 2026  
**Status**: ⬜ Not Started

### Objectives

- SMS notifications (Twilio)
- Support system (help center + chat)
- Analytics dashboard
- Action pages (petitions, signups)
- Groups for organizing contacts
- Enhanced event features

### 2.1 SMS Notifications

| Task | Status | Priority |
|------|--------|----------|
| Twilio integration | ⬜ Not Started | P0 |
| SMS opt-in flow | ⬜ Not Started | P0 |
| Event reminder SMS | ⬜ Not Started | P0 |
| Opt-out handling | ⬜ Not Started | P0 |
| SMS templates | ⬜ Not Started | P1 |

### 2.2 Support System

| Task | Status | Priority |
|------|--------|----------|
| Help center (articles) | ⬜ Not Started | P1 |
| Article editor | ⬜ Not Started | P1 |
| Chat widget | ⬜ Not Started | P2 |
| Agent inbox | ⬜ Not Started | P2 |
| FAQ/knowledge base | ⬜ Not Started | P1 |

### 2.3 Analytics

| Task | Status | Priority |
|------|--------|----------|
| Event performance metrics | ⬜ Not Started | P1 |
| Attendee analytics | ⬜ Not Started | P1 |
| Organization dashboard | ⬜ Not Started | P1 |
| Export reports | ⬜ Not Started | P2 |

### 2.4 Action Pages

| Task | Status | Priority |
|------|--------|----------|
| Petition pages | ⬜ Not Started | P1 |
| Email-your-rep actions | ⬜ Not Started | P2 |
| Signup forms | ⬜ Not Started | P1 |
| Action tracking/analytics | ⬜ Not Started | P2 |

### 2.5 Groups

| Task | Status | Priority |
|------|--------|----------|
| Group creation | ⬜ Not Started | P1 |
| Dynamic group rules | ⬜ Not Started | P2 |
| Manual group membership | ⬜ Not Started | P1 |
| Group-based messaging | ⬜ Not Started | P2 |

### 2.6 Enhanced Events

| Task | Status | Priority |
|------|--------|----------|
| Recurring events | ⬜ Not Started | P2 |
| Volunteer shifts | ⬜ Not Started | P1 |
| Event check-in app | ⬜ Not Started | P2 |
| Post-event surveys | ⬜ Not Started | P2 |

### Deliverables

- [ ] SMS notification system
- [ ] Support website with help center
- [ ] Analytics dashboard
- [ ] Action pages (petitions, signups)
- [ ] Groups functionality
- [ ] Recurring events and shifts

---

## Phase 3: Campaign Platform

**Duration**: Q4 2026  
**Status**: ⬜ Not Started

### Objectives

- Communications platform (bulk email/SMS)
- Voter data integration
- Canvassing tools
- Phone banking
- Website builder

### 3.1 Communications Platform

| Task | Status | Priority |
|------|--------|----------|
| Bulk email campaigns | ⬜ Not Started | P0 |
| Email builder (WYSIWYG) | ⬜ Not Started | P1 |
| Bulk SMS campaigns | ⬜ Not Started | P1 |
| Audience segmentation | ⬜ Not Started | P1 |
| Campaign analytics (opens, clicks) | ⬜ Not Started | P1 |
| Unsubscribe management | ⬜ Not Started | P0 |

### 3.2 Voter Data

| Task | Status | Priority |
|------|--------|----------|
| Voter file import | ⬜ Not Started | P0 |
| Voter search/filter | ⬜ Not Started | P0 |
| Custom voter attributes | ⬜ Not Started | P1 |
| Voter history tracking | ⬜ Not Started | P1 |
| One-click voter targeting presets | ⬜ Not Started | P2 |

### 3.3 Canvassing (Field)

| Task | Status | Priority |
|------|--------|----------|
| Canvass universe creation | ⬜ Not Started | P0 |
| Route optimization | ⬜ Not Started | P1 |
| Canvass scripts | ⬜ Not Started | P1 |
| Door-knock tracking | ⬜ Not Started | P0 |
| Canvasser assignment | ⬜ Not Started | P1 |
| Real-time sync | ⬜ Not Started | P1 |

### 3.4 Phone Banking

| Task | Status | Priority |
|------|--------|----------|
| Call list generation | ⬜ Not Started | P0 |
| Click-to-call integration | ⬜ Not Started | P1 |
| Call scripts | ⬜ Not Started | P1 |
| Call result tracking | ⬜ Not Started | P0 |
| Predictive dialer (future) | ⬜ Not Started | P3 |

### 3.5 Website Builder

| Task | Status | Priority |
|------|--------|----------|
| Template library | ⬜ Not Started | P1 |
| Drag-and-drop builder | ⬜ Not Started | P2 |
| Custom domain support | ⬜ Not Started | P1 |
| Static site generation | ⬜ Not Started | P0 |
| A/B testing framework | ⬜ Not Started | P2 |

### Deliverables

- [ ] Full communications platform
- [ ] Voter data management
- [ ] Canvassing tools
- [ ] Phone banking system
- [ ] Basic website builder
- [ ] Full platform rebrand/merge

---

## Phase 4: Advanced Features

**Duration**: Q1 2027+  
**Status**: ⬜ Not Started

### Objectives

- P2P texting
- Automated voter modeling
- Self-service polling
- Mobile app (post-Dioxus 1.0)
- Advanced analytics & AI

### 4.1 P2P Texting

| Task | Status | Priority |
|------|--------|----------|
| Texting campaign creation | ⬜ Not Started | P1 |
| Volunteer texting interface | ⬜ Not Started | P1 |
| Response tracking | ⬜ Not Started | P1 |
| Opt-out compliance | ⬜ Not Started | P0 |
| Conversation sync | ⬜ Not Started | P1 |

### 4.2 Voter Modeling

| Task | Status | Priority |
|------|--------|----------|
| Automated propensity scoring | ⬜ Not Started | P2 |
| Issue-based modeling | ⬜ Not Started | P2 |
| Persuasion targets | ⬜ Not Started | P2 |
| Turnout modeling | ⬜ Not Started | P2 |

### 4.3 Self-Service Polling

| Task | Status | Priority |
|------|--------|----------|
| Survey builder | ⬜ Not Started | P1 |
| IVR polling | ⬜ Not Started | P2 |
| Online polling | ⬜ Not Started | P1 |
| Results analysis | ⬜ Not Started | P1 |
| Crosstabs & weighting | ⬜ Not Started | P2 |

### 4.4 Mobile App

| Task | Status | Priority |
|------|--------|----------|
| Push notifications | ⬜ Not Started | P1 |
| Canvassing mobile UI | ⬜ Not Started | P1 |
| Event check-in via mobile | ⬜ Not Started | P2 |
| Offline mode | ⬜ Not Started | P2 |

### 4.5 Advanced Analytics & AI

| Task | Status | Priority |
|------|--------|----------|
| Cross-feature dashboards | ⬜ Not Started | P1 |
| Predictive analytics | ⬜ Not Started | P2 |
| AI-assisted content | ⬜ Not Started | P3 |
| Automated A/B testing | ⬜ Not Started | P2 |

---

## Ongoing: Infrastructure & Operations

### Deployment & DevOps

| Task | Status | Priority |
|------|--------|----------|
| VPS provisioning automation | ⬜ Not Started | P0 |
| Docker deployment scripts | ⬜ Not Started | P0 |
| SSL certificate automation | ⬜ Not Started | P0 |
| Database backup automation | ⬜ Not Started | P0 |
| Monitoring & alerting | ⬜ Not Started | P1 |
| Log aggregation | ⬜ Not Started | P1 |
| Staging environment | ⬜ Not Started | P1 |
| Blue-green deployments | ⬜ Not Started | P2 |

### Security

| Task | Status | Priority |
|------|--------|----------|
| Security headers | ⬜ Not Started | P0 |
| Rate limiting | ⬜ Not Started | P0 |
| Audit logging | ⬜ Not Started | P1 |
| GDPR compliance | ⬜ Not Started | P1 |
| Penetration testing | ⬜ Not Started | P2 |
| SOC 2 preparation | ⬜ Not Started | P3 |

### Performance

| Task | Status | Priority |
|------|--------|----------|
| CDN setup (static assets) | ⬜ Not Started | P1 |
| Database query optimization | ⬜ Not Started | P1 |
| Redis caching layer | ⬜ Not Started | P1 |
| Load testing | ⬜ Not Started | P2 |

---

## Priority Legend

| Priority | Meaning |
|----------|---------|
| P0 | Critical - blocks launch |
| P1 | High - required for good launch |
| P2 | Medium - nice to have for launch |
| P3 | Low - post-launch |

## Status Legend

| Status | Meaning |
|--------|---------|
| ⬜ Not Started | Work not begun |
| 🟡 In Progress | Currently being worked on |
| 🔵 In Review | Implementation complete, under review |
| ✅ Done | Completed and deployed |
| ❌ Blocked | Cannot proceed due to dependency |

---

## Related Documentation

- [AGENTS.md](../AGENTS.md) - AI coding assistant context
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical architecture
- [Features Documentation](features/) - Detailed feature specs
