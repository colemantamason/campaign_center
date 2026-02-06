# Campaign Center - Development Roadmap

> **Last Updated**: February 2026

---

## Table of Contents

1. [Overview](#overview)
2. [Phase 0: Foundation](#phase-0-foundation)
3. [Phase 1: Events Platform](#phase-1-events-platform)
4. [Phase 2: Infrastructure & Optimization](#phase-2-infrastructure--optimization)
5. [Phase 3: Campaign Platform](#phase-3-campaign-platform)
6. [Phase 4: Website Builder](#phase-4-website-builder)

---

## Overview

### Strategic Approach

1. **Build Events Platform** - Complete the full Events feature set (events, actions, groups, analytics) before public launch
2. **Optimize & Harden** - After launch, focus on infrastructure, performance, and security before adding new features
3. **Expand to Campaign Tools** - Add voter data, fieldwork, and outreach tools progressively
4. **Apply Lessons Learned** - Port optimization patterns to new features as they're built

### Phase Order

Phases are listed in priority order. Each phase should be fully complete before moving to the next.

---

## Phase 0: Foundation

**Status**: 🟡 In Progress

### Completed

- [x] Project scaffolding (packages, feature flags)
- [x] Basic web app structure (routes, sidebar, layout)
- [x] Shared UI component library
- [x] Mock data and permission system
- [x] Hot reload configuration
- [x] Backend infrastructure (Diesel + PostgreSQL, Redis sessions, connection pooling)
- [x] Authentication system (registration, login/logout, password management)
  - [x] Secure session token handling (HttpOnly cookies for web, X-Session-Token header for mobile)
  - [x] Session tokens NOT exposed in JSON responses (XSS prevention)
  - [x] Redis/Postgres session synchronization on logout
  - [x] Proper database initialization with error handling
  - [x] User-Agent and IP capture on login
  - [x] Login timing attack mitigation
  - [x] Subdomain cookie configuration support

## Future Improvements (Added from plan)

- **Rate limiting on auth endpoints**: Prevent brute force attacks (consider tower-governor or similar)
- **CSRF tokens**: For authenticated endpoints, implement Double Submit Cookie pattern
- **Scheduled session cleanup**: Run `cleanup_expired_sessions()` periodically (cron job or tokio task)
- **Sliding session expiration**: Extend session on activity (update both Redis TTL and Postgres expires_at)
- **Audit logging**: Log all auth events (login, logout, failed attempts) with IP and User-Agent

### Remaining

- [ ] Organization management (creation, settings, team, permissions)
- [ ] Development environment (Docker Compose, seeding, CI/CD)

### Deliverables

- Working authentication
- Organization and team management
- Permission-based route protection (real, not mock)
- Local development environment with Docker

---

## Phase 1: Events Platform

**Status**: ⬜ Not Started

This is the initial product launch. All features in this phase are part of the "Events" subscription plan. Nothing ships until everything is complete.

### Event Management

- Event CRUD (create, edit, duplicate, cancel, publish/draft)
- Attendee management (list, add, status, check-in, export, waitlist)
- Recurring events and volunteer shifts
- Co-hosting with other organizations
- Post-event surveys

### Action Pages

- Petition pages
- Signup forms
- Email-your-rep actions
- Action tracking and analytics

### Event & Action Discovery Website

- Public event search and discovery
- Event detail pages with RSVP
- Organization profiles
- Embeddable event widgets

### Surveys Website

- Public survey/poll response platform for voters
- Survey detail pages with response submission
- No authentication required
- Duplicate response prevention (IP/fingerprint-based)
- Progress saving for multi-page surveys
- Organization branding on surveys

### Groups

- Contact group management
- Dynamic group rules
- Group-based messaging

### Notifications

- Email notifications (AWS SES) - confirmations, reminders, updates, cancellations
- SMS notifications (Twilio) - opt-in/opt-out, reminders, templates

### Payments

- Stripe Connect integration
- Ticketing (free and paid events, ticket types)
- Checkout, refunds, and payouts

### Analytics

- Event performance metrics
- Attendee analytics
- Organization dashboard
- Exportable reports

### Support System

- Help center with articles
- Knowledge base / FAQ
- Chat widget and agent inbox

### Marketing Website

- Landing page
- Features and pricing pages
- Contact form
- Blog

### Deliverables

- Complete event creation and management system
- Public event discovery website
- Public surveys website for poll responses
- Action pages (petitions, signups)
- Groups and contact organization
- Full notification system (email + SMS)
- Ticketing and payment processing
- Analytics dashboard
- Support website with help center
- Marketing website
- **Ready for public launch**

---

## Phase 2: Infrastructure & Optimization

**Status**: ⬜ Not Started

After the Events Platform launches, this phase focuses on making the system production-ready at scale. Lessons learned here will inform how new features are built.

### Performance Optimization

- **Database Optimization**
  - Query analysis and optimization (EXPLAIN ANALYZE, slow query logging)
  - Index tuning and query plan optimization
  - Connection pool tuning and monitoring
  - Read replica configuration for analytics queries
- **Caching Strategy**
  - Redis caching layer for frequently accessed data
  - Cache invalidation patterns and TTL policies
  - Session and rate limit data in Redis
  - Static asset caching headers
- **Application Performance**
  - Response time profiling and optimization
  - Memory usage analysis and reduction
  - Async task queuing for email/SMS
  - Database query batching and N+1 elimination
- **CDN & Asset Delivery**
  - CDN setup for static assets
  - Image optimization and responsive delivery
  - Gzip/Brotli compression
  - Edge caching for public pages

### Deployment & DevOps

- **Automation**
  - VPS provisioning automation (Infrastructure as Code)
  - Docker deployment scripts and container orchestration
  - SSL certificate automation (Let's Encrypt)
  - Database backup automation and restore testing
- **Environments**
  - Staging environment matching production
  - Blue-green or rolling deployments
  - Database migration strategy for zero-downtime deploys
  - Feature flags for gradual rollouts
- **Observability**
  - Centralized logging and log aggregation
  - Application performance monitoring (APM)
  - Error tracking and alerting
  - Uptime monitoring and status page
  - Custom dashboards for key metrics

### Security Hardening

- **Application Security**
  - Security headers (CSP, HSTS, X-Frame-Options, etc.)
  - Rate limiting and DDoS protection
  - Input validation and sanitization audit
  - SQL injection and XSS prevention review
- **Authentication & Authorization**
  - Session security audit
  - Password policy enforcement
  - Brute force protection
  - API authentication review
- **Compliance & Audit**
  - Audit logging for sensitive operations
  - GDPR compliance (data export, deletion)
  - Privacy policy and terms of service
  - Data retention policies
- **Testing & Verification**
  - Penetration testing
  - Dependency vulnerability scanning
  - Security-focused code review process

### Scalability Preparation

- **Load Testing**
  - Baseline performance benchmarks
  - Load testing under expected peak traffic
  - Stress testing to find breaking points
  - Capacity planning documentation
- **Database Scaling**
  - Horizontal scaling strategy
  - Sharding considerations for large tables
  - Archive strategy for historical data
- **Infrastructure Scaling**
  - Auto-scaling policies
  - Multi-region considerations
  - Disaster recovery planning

### Deliverables

- Optimized database queries and caching
- Automated deployment pipeline
- Comprehensive monitoring and alerting
- Security hardening complete
- Load tested with documented capacity limits
- Documented patterns for building scalable features

---

## Phase 3: Campaign Platform

**Status**: ⬜ Not Started

This phase expands from an Events-only platform to a full campaign management suite. All features in this phase launch together as a unified product.

### Voter Data & Modeling

- Voter file import and management
- Voter search, filtering, and targeting
- Custom voter attributes and tagging
- Voter history and interaction tracking
- Targeting presets for common universes
- Propensity scoring (support, turnout)
- Issue-based modeling
- Persuasion and turnout targeting
- Model updates based on contact results

### Mobile App

- Push notifications
- Offline data sync
- Event check-in via mobile
- Canvassing mobile interface

### Fieldwork

- **Canvassing**
  - Canvass universe creation from voter data
  - Route optimization
  - Scripts and survey questions
  - Door-knock tracking and result recording
  - Real-time sync with mobile app
- **Phone Banking**
  - Call list generation from voter data
  - Click-to-call integration
  - Call scripts
  - Call result tracking
  - Predictive dialer

### Texting & Polling

Grouped together as they share infrastructure (phone number management, compliance, response tracking).

- **Texting**
  - P2P texting campaigns
  - Volunteer texting interface
  - Response tracking and conversation sync
  - Opt-out compliance (TCPA, carrier requirements)
  - Broadcast and targeted messaging
  
- **Polling**
  - Survey/poll builder (creates surveys displayed on surveys website)
  - IVR (phone) polling
  - Online polling (via surveys website)
  - Results analysis with crosstabs
  - Weighting and demographic adjustments
  - Response tracking and analytics

### Communications Platform

- Bulk email campaigns
- Email builder (WYSIWYG)
- Bulk SMS campaigns
- Audience segmentation
- Campaign analytics (opens, clicks, responses)
- Unsubscribe management

### Advanced Analytics

- Cross-feature dashboards
- Predictive analytics
- AI-assisted content suggestions
- Automated A/B testing

### Marketing & Brand Transition

This phase marks the transition from an Events-focused product to a full Campaign platform. Marketing efforts required:

- **Brand Evolution**
  - Evaluate rebrand vs. brand extension (e.g., "RallyUp" → "RallyUp for Campaigns" or new unified brand)
  - Updated visual identity reflecting full platform capabilities
  - Messaging shift from "event management" to "campaign management"
- **Website & Positioning**
  - Redesigned marketing website with campaign-focused messaging
  - New feature pages for voter data, fieldwork, texting, polling
  - Updated pricing page with campaign-tier plans
  - Case studies and testimonials from campaign customers
- **Launch Campaign**
  - Announcement strategy for existing Events customers
  - Migration path and upgrade incentives
  - Press outreach and industry publication coverage
  - Demo videos and onboarding materials for new features
- **Sales Enablement**
  - Updated pitch decks and sales materials
  - Competitive positioning against existing campaign tools
  - Training for sales team on campaign workflows

### Deliverables

- Complete voter data management and modeling
- Mobile app for fieldwork
- Canvassing and phone banking tools
- P2P and broadcast texting
- Self-service polling
- Full communications platform
- Advanced analytics and insights
- Rebranded marketing presence
- **Full Campaign Platform launch**

---

## Phase 4: Website Builder

**Status**: ⬜ Not Started

Self-service website creation for campaigns.

### Website Builder

- Template library
- Drag-and-drop page builder
- Custom domain support
- Static site generation
- Integration with signup forms
- A/B testing framework

### Deliverables

- Complete website builder
- Template library
- Custom domain management

---

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

- [AGENTS.md](AGENTS.md) - AI coding assistant context
