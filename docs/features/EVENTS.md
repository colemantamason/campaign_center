# Events Feature Specification

> **Feature Area**: Events Management  
> **Status**: Requirements Complete  
> **Phase**: MVP (Phase 1)

---

## Table of Contents

1. [Overview](#overview)
2. [User Stories](#user-stories)
3. [Data Models](#data-models)
4. [Event Creation](#event-creation)
5. [Event Management](#event-management)
6. [Attendee Management](#attendee-management)
7. [Public Event Discovery](#public-event-discovery)
8. [RSVP & Registration](#rsvp--registration)
9. [Co-hosting](#co-hosting)
10. [Ticketing & Payments](#ticketing--payments)
11. [Notifications](#notifications)
12. [API Specification](#api-specification)

---

## Overview

### Purpose

The Events feature enables organizations to:
- Create and manage events (rallies, meetings, canvasses, phone banks, etc.)
- Accept RSVPs and manage attendees
- Send automated reminders via email and SMS
- Co-host events with partner organizations
- Optionally sell tickets via Stripe

### User Roles

| Role | Permissions |
|------|-------------|
| **Public User** | Browse events, RSVP, view event details |
| **Attendee** | Manage their RSVP, receive notifications |
| **Organization Member** | View events for their organization |
| **Event Manager** | Create/edit events, manage attendees |
| **Organization Admin** | All event permissions + manage co-hosts |
| **Co-host** | View attendees, send messages (based on permissions) |

---

## User Stories

### Organization Staff

```
As an event organizer, I want to:
- Create events with all necessary details (title, description, date, location)
- Set capacity limits and enable waitlists
- Publish events to make them discoverable
- View and manage RSVPs/attendees
- Check in attendees on the day of the event
- Send updates to all attendees
- See analytics about event performance
- Duplicate past events for recurring activities
- Cancel events with automatic attendee notification
```

### Co-hosting Organizations

```
As a co-host organization, I want to:
- Receive and accept co-host invitations
- Access attendee lists for my contributed signups
- Send messages to attendees on behalf of my organization
- Have my organization displayed on the event page
```

### Public Users / Attendees

```
As a potential attendee, I want to:
- Browse upcoming events by location, date, or organization
- See all event details before committing
- RSVP with minimal friction (name, email, phone optional)
- Receive a confirmation email after signing up
- Get reminders before the event
- Easily cancel or modify my RSVP
- Add the event to my calendar
- Share the event with others
```

---

## Data Models

### Event

```rust
/// Core event model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub organization_id: i32,
    
    // Basic Info
    pub title: String,                    // Required, max 255 chars
    pub slug: String,                     // URL-safe, unique per org
    pub description: Option<String>,       // Rich text (HTML), max 50KB
    pub short_description: Option<String>, // Plain text, max 500 chars
    pub cover_image_url: Option<String>,   // MinIO URL
    pub event_type: EventType,
    
    // Timing
    pub start_time: DateTime<Utc>,        // Required
    pub end_time: DateTime<Utc>,          // Required
    pub timezone: String,                  // IANA timezone, default "America/New_York"
    
    // Location
    pub location_type: LocationType,       // Required
    pub address: Option<EventAddress>,     // Required if in_person or hybrid
    pub virtual_link: Option<String>,      // Required if virtual or hybrid
    pub virtual_platform: Option<String>,  // "zoom", "google_meet", "custom", etc.
    
    // Capacity & Registration
    pub capacity: Option<i32>,             // None = unlimited
    pub waitlist_enabled: bool,            // Default: false
    pub registration_deadline: Option<DateTime<Utc>>,
    pub requires_approval: bool,           // Default: false
    
    // Visibility
    pub is_published: bool,                // Default: false (draft)
    pub is_featured: bool,                 // Default: false
    pub visibility: EventVisibility,       // Default: public
    
    // Contact Info
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    
    // Metadata
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    
    // Computed (not stored)
    pub attendee_count: i32,
    pub waitlist_count: i32,
    pub available_spots: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    Rally,
    Meeting,
    Canvass,
    PhoneBank,
    TextBank,
    Fundraiser,
    Training,
    Social,
    Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LocationType {
    InPerson,
    Virtual,
    Hybrid,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventVisibility {
    Public,    // Discoverable on events website
    Unlisted,  // Accessible via direct link only
    Private,   // Only visible to organization members
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventAddress {
    pub line_1: String,
    pub line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,           // ISO 2-letter code, default "US"
    pub latitude: Option<f64>,     // For map display and search
    pub longitude: Option<f64>,
}
```

### Attendee

```rust
/// Event attendee/RSVP
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attendee {
    pub id: i32,
    pub event_id: i32,
    pub user_id: Option<i32>,      // None for guest signups
    
    // Contact Info
    pub email: String,              // Required
    pub first_name: String,         // Required
    pub last_name: String,          // Required
    pub phone: Option<String>,      // For SMS reminders
    pub phone_opted_in: bool,       // SMS consent
    
    // RSVP Details
    pub status: AttendeeStatus,
    pub guest_count: i32,           // Additional guests, default 0
    pub notes: Option<String>,      // Attendee-provided notes
    
    // Ticketing (if applicable)
    pub ticket_type_id: Option<i32>,
    pub order_id: Option<i32>,
    
    // Tracking
    pub source: AttendeeSource,
    pub referrer_id: Option<i32>,   // User who shared the event
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    
    // Timestamps
    pub registered_at: DateTime<Utc>,
    pub checked_in_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AttendeeStatus {
    Registered,    // Confirmed RSVP
    Waitlisted,    // On waitlist
    PendingApproval, // Requires org approval
    Cancelled,     // Attendee cancelled
    Attended,      // Checked in at event
    NoShow,        // Did not attend
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AttendeeSource {
    Direct,        // Direct event page visit
    Share,         // Shared link
    Embed,         // Embedded widget
    Email,         // Email campaign link
    Sms,           // SMS campaign link
    Search,        // Events website search
}
```

### Co-host

```rust
/// Event co-hosting relationship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventCohost {
    pub id: i32,
    pub event_id: i32,
    pub organization_id: i32,
    
    // Permissions
    pub can_edit: bool,              // Edit event details
    pub can_manage_attendees: bool,  // View/manage attendees
    pub can_send_messages: bool,     // Send to attendees
    
    // Status
    pub status: CohostStatus,
    pub invited_at: DateTime<Utc>,
    pub invited_by: i32,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CohostStatus {
    Pending,
    Accepted,
    Rejected,
}
```

---

## Event Creation

### Create Event Form

#### Required Fields

| Field | Type | Validation | Notes |
|-------|------|------------|-------|
| `title` | String | 1-255 chars | Plain text |
| `event_type` | Enum | Must be valid type | Dropdown |
| `start_time` | DateTime | Future date required | Date + time picker |
| `end_time` | DateTime | After start_time | Date + time picker |
| `timezone` | String | Valid IANA timezone | Dropdown, default org timezone |
| `location_type` | Enum | Must be selected | Radio buttons |

#### Conditional Fields

| Field | Condition | Validation |
|-------|-----------|------------|
| `address.*` | location_type = InPerson or Hybrid | All address fields required |
| `virtual_link` | location_type = Virtual or Hybrid | Valid URL |
| `virtual_platform` | location_type = Virtual or Hybrid | Optional selection |

#### Optional Fields

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `description` | Rich Text | None | WYSIWYG editor |
| `short_description` | String | None | Used in cards/previews |
| `cover_image_url` | File Upload | None | Max 5MB, JPG/PNG/WebP |
| `capacity` | Integer | None (unlimited) | Min 1 if set |
| `waitlist_enabled` | Boolean | false | Only if capacity set |
| `registration_deadline` | DateTime | None | Before start_time |
| `requires_approval` | Boolean | false | Manual approval required |
| `visibility` | Enum | Public | Dropdown |
| `contact_email` | String | Org default | Valid email |
| `contact_phone` | String | None | Valid phone format |

### Form Behavior

```
┌─────────────────────────────────────────────────────────────┐
│                     CREATE EVENT                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Event Title *                                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Rally for Education                                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  Event Type *                                                │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Rally                                           ▼   │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌──────────────────────┐  ┌──────────────────────┐        │
│  │ Start Date & Time *  │  │  End Date & Time *   │        │
│  │ Feb 15, 2026 2:00 PM │  │ Feb 15, 2026 5:00 PM │        │
│  └──────────────────────┘  └──────────────────────┘        │
│                                                              │
│  Timezone: America/New_York                                  │
│                                                              │
│  Location Type *                                             │
│  ( ) In-Person   ( ) Virtual   (•) Hybrid                   │
│                                                              │
│  ┌─ In-Person Location ────────────────────────────────┐   │
│  │  Address Line 1: 123 Main Street                     │   │
│  │  Address Line 2: Suite 100                           │   │
│  │  City: Springfield    State: IL    ZIP: 62701       │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─ Virtual Details ───────────────────────────────────┐   │
│  │  Meeting Link: https://zoom.us/j/123456789           │   │
│  │  Platform: Zoom                                      │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─ Capacity & Registration ───────────────────────────┐   │
│  │  Maximum Capacity: [  100  ]  ☑ Enable waitlist     │   │
│  │  Registration Deadline: Feb 14, 2026 11:59 PM       │   │
│  │  ☐ Require approval for registrations               │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─ Cover Image ───────────────────────────────────────┐   │
│  │  [Drag & drop or click to upload]                    │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  Description                                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ [Rich text editor with basic formatting]             │   │
│  │                                                       │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│           [Save as Draft]  [Save & Publish]                 │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Slug Generation

- Auto-generated from title: "Rally for Education" → "rally-for-education"
- Append date if duplicate: "rally-for-education-2026-02-15"
- Allow manual editing before publish
- Immutable after publish

### Expected Behavior

1. **Form Validation**: Real-time validation with inline error messages
2. **Auto-save Draft**: Save draft every 30 seconds while editing
3. **Geocoding**: Auto-geocode address on blur for map preview
4. **Image Upload**: Direct upload to MinIO via presigned URL
5. **Timezone**: Default to organization's timezone setting

---

## Event Management

### Event List View

```
┌─────────────────────────────────────────────────────────────┐
│  Events                                    [+ Create Event] │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Filters: [All ▼] [Upcoming ▼] [All Types ▼]  🔍 Search    │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ ⬤ Rally for Education                    Feb 15 2:00PM │  │
│  │   📍 Springfield, IL  │  👥 45/100  │  Draft          │  │
│  │                                   [Edit] [Publish ▼]  │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │ ⬤ Phone Bank: Get Out the Vote        Feb 18 6:00PM │  │
│  │   💻 Virtual  │  👥 12 registered  │  Published       │  │
│  │                           [Edit] [View Page] [More ▼] │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │ ⬤ Volunteer Training                   Feb 20 10:00AM │  │
│  │   📍 Chicago, IL  │  👥 8/25  │  Published            │  │
│  │                           [Edit] [View Page] [More ▼] │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Showing 3 of 3 events                                      │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Event Detail View (Admin)

```
┌─────────────────────────────────────────────────────────────┐
│  ← Back to Events                                           │
│                                                              │
│  Rally for Education                                        │
│  Saturday, February 15, 2026 • 2:00 PM - 5:00 PM EST       │
│                                                              │
│  [Edit Event] [View Public Page] [Duplicate] [Cancel ▼]    │
│                                                              │
├──────────────────────────────────────────────────────────────
│ [Overview] [Attendees] [Messages] [Analytics] [Settings]    │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Quick Stats                                                 │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│  │    45    │ │    5     │ │    50    │ │   90%    │       │
│  │Registered│ │ Waitlist │ │ Capacity │ │Filled    │       │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘       │
│                                                              │
│  Event Details                                               │
│  ─────────────                                               │
│  Type: Rally                                                 │
│  Location: 123 Main Street, Suite 100                       │
│            Springfield, IL 62701                            │
│  Virtual: https://zoom.us/j/123456789                       │
│                                                              │
│  Description                                                 │
│  ─────────────                                               │
│  Join us for an important rally to support education        │
│  funding in our community...                                │
│                                                              │
│  Co-hosts (2)                                               │
│  ─────────────                                               │
│  • Teachers United (accepted)                               │
│  • Parents for Education (pending)          [Invite Co-host]│
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Event Actions

| Action | Conditions | Result |
|--------|------------|--------|
| **Edit** | Draft or Published | Open edit form |
| **Publish** | Draft, all required fields | Set is_published = true, published_at = now |
| **Unpublish** | Published | Set is_published = false |
| **Duplicate** | Any | Create new draft with copied fields |
| **Cancel** | Not cancelled | Set cancelled_at, notify all attendees |
| **Delete** | Draft only | Hard delete event |

---

## Attendee Management

### Attendee List View

```
┌─────────────────────────────────────────────────────────────┐
│  Rally for Education > Attendees                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  [Export CSV] [Add Attendee] [Message All]                  │
│                                                              │
│  Filter: [All Statuses ▼]  🔍 Search by name or email      │
│                                                              │
│  ┌───┬──────────────────┬───────────────────┬────────────┐ │
│  │ ☐ │ Name             │ Email             │ Status     │ │
│  ├───┼──────────────────┼───────────────────┼────────────┤ │
│  │ ☐ │ Jane Doe         │ jane@email.com    │ Registered │ │
│  │ ☐ │ John Smith       │ john@email.com    │ Registered │ │
│  │ ☐ │ Sarah Johnson    │ sarah@email.com   │ Waitlisted │ │
│  │ ☐ │ Mike Brown       │ mike@email.com    │ Cancelled  │ │
│  └───┴──────────────────┴───────────────────┴────────────┘ │
│                                                              │
│  Bulk Actions: [Check In Selected] [Cancel Selected]        │
│                                                              │
│  Page 1 of 2  │ 45 registrations │ 5 waitlisted │ 3 cancelled│
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Attendee Detail Modal

```
┌─────────────────────────────────────────────────────────────┐
│  Attendee Details                                     [X]   │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Jane Doe                                                   │
│  jane@email.com                                             │
│  (555) 123-4567 • SMS opted in ✓                           │
│                                                              │
│  Status: Registered                                         │
│  Additional Guests: 2                                       │
│  Registered: Feb 10, 2026 3:45 PM                          │
│                                                              │
│  Source: Direct                                             │
│                                                              │
│  Notes from attendee:                                       │
│  "I'll be bringing my two kids, ages 8 and 10."            │
│                                                              │
│  Actions: [Check In] [Mark No-Show] [Cancel RSVP] [Message] │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Check-in Flow

1. **List View**: Click "Check In" on row or select multiple → "Check In Selected"
2. **Check-in Mode**: Toggle "Check-in Mode" for streamlined view
   - Search by name or scan QR code
   - Single-click check-in
   - Success sound/visual feedback
3. **Walk-up Registration**: Quick add for on-site signups

### Export Format (CSV)

```csv
first_name,last_name,email,phone,status,guest_count,registered_at,checked_in_at,source
Jane,Doe,jane@email.com,+15551234567,registered,2,2026-02-10T15:45:00Z,,direct
John,Smith,john@email.com,,registered,0,2026-02-11T09:30:00Z,2026-02-15T13:55:00Z,share
```

---

## Public Event Discovery

### Events Website Structure

```
events.campaigncenter.com/
├── /                        # Homepage with search & featured
├── /search                  # Search results
├── /e/{org-slug}/{event-slug}  # Event detail page
├── /o/{org-slug}            # Organization profile
└── /my-events               # User's RSVPs (if logged in)
```

### Homepage

```
┌─────────────────────────────────────────────────────────────┐
│  🎯 Campaign Events              [Sign In]                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│     Find events near you                                    │
│     ┌────────────────────┐ ┌───────────┐ ┌────────┐        │
│     │ 📍 Enter location  │ │ Any date ▼│ │ Search │        │
│     └────────────────────┘ └───────────┘ └────────┘        │
│                                                              │
│  Featured Events                                            │
│  ─────────────────                                          │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐│
│  │ [Cover Image]   │ │ [Cover Image]   │ │ [Cover Image]   ││
│  │                 │ │                 │ │                 ││
│  │ Rally for       │ │ Town Hall:      │ │ Volunteer       ││
│  │ Education       │ │ Healthcare      │ │ Training        ││
│  │                 │ │                 │ │                 ││
│  │ Feb 15 • 2PM    │ │ Feb 18 • 6PM    │ │ Feb 20 • 10AM   ││
│  │ Springfield, IL │ │ Virtual         │ │ Chicago, IL     ││
│  └─────────────────┘ └─────────────────┘ └─────────────────┘│
│                                                              │
│  Browse by Category                                         │
│  ─────────────────────                                      │
│  [Rally] [Meeting] [Canvass] [Phone Bank] [Training] [More] │
│                                                              │
│  Upcoming Near You (Chicago, IL)                           │
│  ─────────────────────────────────                          │
│  [Event cards...]                                           │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Event Detail Page

```
┌─────────────────────────────────────────────────────────────┐
│  🎯 Campaign Events    [Sign In]                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                    [COVER IMAGE]                       │  │
│  │                                                        │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Rally for Education                                        │
│  Hosted by Teachers United + Parents for Education          │
│                                                              │
│  📅 Saturday, February 15, 2026                             │
│     2:00 PM - 5:00 PM EST                                   │
│                                                              │
│  📍 123 Main Street, Suite 100                              │
│     Springfield, IL 62701                                   │
│     [View on Map]                                           │
│                                                              │
│  💻 Also streaming on Zoom                                  │
│     Join link provided after registration                   │
│                                                              │
│  ────────────────────────────────────────────────────────── │
│                                                              │
│  About this event                                           │
│                                                              │
│  Join us for an important rally to support education        │
│  funding in our community. We'll hear from local teachers,  │
│  parents, and students about the importance of investing    │
│  in our schools.                                            │
│                                                              │
│  • Speakers include Mayor Johnson and Superintendent Davis  │
│  • Free food and refreshments provided                      │
│  • Family-friendly - bring the kids!                        │
│  • Volunteer opportunities available                        │
│                                                              │
│  ────────────────────────────────────────────────────────── │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  👥 45 of 100 spots filled                            │  │
│  │                                                        │  │
│  │              [RSVP - It's Free]                       │  │
│  │                                                        │  │
│  │  Questions? Contact events@teachersunited.org         │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Share: [Facebook] [Twitter] [Copy Link]                    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Search & Filters

| Filter | Type | Options |
|--------|------|---------|
| Location | Text + Geocode | City, state, or zip |
| Distance | Dropdown | 5, 10, 25, 50, 100 miles |
| Date Range | Date Picker | Today, This Week, Custom |
| Event Type | Multi-select | All event types |
| Virtual/In-Person | Toggle | Show virtual only |

---

## RSVP & Registration

### RSVP Form

```
┌─────────────────────────────────────────────────────────────┐
│  Sign up for Rally for Education                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  First Name *                                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  Last Name *                                                │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  Email *                                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  Phone (for text reminders)                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│  ☑ Yes, send me text reminders about this event            │
│                                                              │
│  Bringing additional guests?                                │
│  ┌───────────────────────────────────────────────────┐     │
│  │ 0                                              ▼  │     │
│  └───────────────────────────────────────────────────┘     │
│                                                              │
│  Anything we should know?  (optional)                       │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│                    [Sign Up]                                │
│                                                              │
│  By signing up, you agree to receive emails about this     │
│  event and related communications from Teachers United.    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### RSVP Rules

| Scenario | Behavior |
|----------|----------|
| Capacity available | Register immediately |
| At capacity, waitlist enabled | Add to waitlist |
| At capacity, no waitlist | Show "Event Full" message |
| Past registration deadline | Show "Registration Closed" |
| Requires approval | Status = PendingApproval, notify org |
| Duplicate email | Show "Already registered" with options |

### Post-RSVP Flow

1. **Confirmation Screen**: Show success message with event details
2. **Confirmation Email**: Send immediately (see Notifications)
3. **Add to Calendar**: Offer .ics download and Google/Apple calendar links
4. **Share Prompt**: Encourage sharing with referral tracking

### Cancel/Modify RSVP

- Link in confirmation email: `/rsvp/{rsvp_token}`
- Options: Cancel, update guest count, update phone
- If cancelled from waitlist, auto-promote next waitlisted attendee

---

## Co-hosting

### Invite Flow

1. **Host initiates**: Search for organization by name or enter email
2. **Set permissions**: Select what co-host can do
3. **Send invitation**: Email sent to org admins
4. **Co-host responds**: Accept or reject via link

### Co-host Email Template

```
Subject: You're invited to co-host: Rally for Education

Teachers United has invited Parents for Education to co-host their upcoming event:

Rally for Education
Saturday, February 15, 2026 at 2:00 PM EST
Springfield, IL

As a co-host, you'll be able to:
✓ Access the attendee list
✓ Send messages to attendees
✗ Edit event details (not enabled)

[Accept Invitation]  [Decline]

This invitation expires in 7 days.
```

### Co-host Permissions

| Permission | Description | Default |
|------------|-------------|---------|
| `can_edit` | Edit event details | false |
| `can_manage_attendees` | View/export attendee list | true |
| `can_send_messages` | Send communications to attendees | true |

### Co-host Visibility

- Co-host logo displayed on event page
- Listed in "Hosted by" section
- Can filter their own recruited attendees

---

## Ticketing & Payments

### Ticket Configuration

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketType {
    pub id: i32,
    pub event_id: i32,
    pub name: String,                // e.g., "General Admission", "VIP"
    pub description: Option<String>,
    pub price_cents: i32,            // Price in cents (0 = free)
    pub quantity: Option<i32>,       // None = unlimited
    pub quantity_sold: i32,
    pub sale_start: Option<DateTime<Utc>>,
    pub sale_end: Option<DateTime<Utc>>,
    pub min_per_order: i32,          // Default 1
    pub max_per_order: i32,          // Default 10
    pub is_hidden: bool,             // For promo codes
    pub sort_order: i32,
}
```

### Checkout Flow

```
1. Select Tickets
   ┌─────────────────────────────────────────────────────────────┐
   │  Rally for Education - Get Tickets                          │
   ├─────────────────────────────────────────────────────────────┤
   │                                                              │
   │  General Admission                      $0.00 (Free)        │
   │  Standard entry to the event            [ − ] 2 [ + ]       │
   │                                                              │
   │  VIP Access                             $50.00              │
   │  Includes reserved seating + meet-and-greet  [ − ] 0 [ + ] │
   │                                                              │
   │  Subtotal: $0.00                                            │
   │                                    [Continue to Checkout]   │
   └─────────────────────────────────────────────────────────────┘

2. Attendee Info (for each ticket)
   - First name, last name, email

3. Payment (if subtotal > 0)
   - Stripe Checkout integration
   - Or Stripe Elements embedded

4. Confirmation
   - Email with tickets/QR codes
```

### Stripe Integration

- **Stripe Connect**: Organizations connect their Stripe account
- **Platform fee**: Configurable percentage or fixed fee
- **Refund handling**: Via Stripe Dashboard or API

---

## Notifications

### Email Templates

#### RSVP Confirmation

```
Subject: You're registered! Rally for Education

Hi Jane,

You're confirmed for Rally for Education!

📅 Saturday, February 15, 2026
   2:00 PM - 5:00 PM EST

📍 123 Main Street, Suite 100
   Springfield, IL 62701

💻 Zoom link: https://zoom.us/j/123456789

[Add to Calendar]

What to bring:
• A friend!
• Your enthusiasm

Can't make it? [Cancel RSVP]

See you there!
Teachers United
```

#### 24-Hour Reminder

```
Subject: Tomorrow: Rally for Education

Hi Jane,

Just a reminder - Rally for Education is tomorrow!

📅 Saturday, February 15, 2026 at 2:00 PM EST
📍 123 Main Street, Springfield, IL

[Get Directions] [View Event Details]

Can't make it anymore? [Update RSVP]

See you soon!
Teachers United
```

#### Event Update

```
Subject: Update: Rally for Education

Hi Jane,

There's been an update to Rally for Education:

[Description of what changed]

The event is still on for:
📅 Saturday, February 15, 2026 at 2:00 PM EST
📍 123 Main Street, Springfield, IL

If you have any questions, reply to this email.

Teachers United
```

### SMS Templates

#### Confirmation (if opted in)

```
You're signed up for Rally for Education on Feb 15 at 2PM!
Details: [short link]
Reply STOP to unsubscribe
```

#### 24-Hour Reminder

```
Reminder: Rally for Education is tomorrow at 2PM at 123 Main St, Springfield.
See you there!
Reply STOP to unsubscribe
```

### Notification Schedule

| Notification | Trigger | Channel |
|--------------|---------|---------|
| RSVP Confirmation | Immediately on signup | Email + SMS (if opted in) |
| 24-Hour Reminder | 24 hours before event | Email + SMS |
| 1-Hour Reminder | 1 hour before event | SMS only |
| Event Update | On event edit (if published) | Email |
| Event Cancelled | On cancellation | Email + SMS |
| Waitlist Promotion | When spot opens | Email + SMS |

---

## API Specification

### Event Endpoints

```rust
// Create event
#[server]
pub async fn create_event(req: CreateEventRequest) -> Result<Event, ServerFnError>;

// Get event by ID
#[server]
pub async fn get_event(event_id: i32) -> Result<Event, ServerFnError>;

// Get event by slug (public)
#[server]
pub async fn get_event_by_slug(org_slug: String, event_slug: String) -> Result<Event, ServerFnError>;

// List events for organization
#[server]
pub async fn list_organization_events(
    org_id: i32,
    filters: EventFilters,
    pagination: Pagination,
) -> Result<PaginatedResponse<Event>, ServerFnError>;

// Search public events
#[server]
pub async fn search_events(query: EventSearchQuery) -> Result<PaginatedResponse<Event>, ServerFnError>;

// Update event
#[server]
pub async fn update_event(event_id: i32, req: UpdateEventRequest) -> Result<Event, ServerFnError>;

// Publish event
#[server]
pub async fn publish_event(event_id: i32) -> Result<Event, ServerFnError>;

// Cancel event
#[server]
pub async fn cancel_event(event_id: i32, reason: Option<String>) -> Result<(), ServerFnError>;

// Delete event (draft only)
#[server]
pub async fn delete_event(event_id: i32) -> Result<(), ServerFnError>;
```

### Attendee Endpoints

```rust
// RSVP to event (public)
#[server]
pub async fn rsvp_to_event(event_id: i32, req: RsvpRequest) -> Result<Attendee, ServerFnError>;

// List attendees
#[server]
pub async fn list_attendees(
    event_id: i32,
    filters: AttendeeFilters,
    pagination: Pagination,
) -> Result<PaginatedResponse<Attendee>, ServerFnError>;

// Update attendee status
#[server]
pub async fn update_attendee_status(
    attendee_id: i32,
    status: AttendeeStatus,
) -> Result<Attendee, ServerFnError>;

// Check in attendee
#[server]
pub async fn check_in_attendee(attendee_id: i32) -> Result<Attendee, ServerFnError>;

// Cancel RSVP (by attendee)
#[server]
pub async fn cancel_rsvp(rsvp_token: String) -> Result<(), ServerFnError>;

// Export attendees
#[server]
pub async fn export_attendees(event_id: i32, format: ExportFormat) -> Result<String, ServerFnError>;
```

### Co-host Endpoints

```rust
// Invite co-host
#[server]
pub async fn invite_cohost(event_id: i32, org_id: i32, permissions: CohostPermissions) -> Result<EventCohost, ServerFnError>;

// Accept/reject co-host invitation
#[server]
pub async fn respond_to_cohost_invitation(
    invitation_id: i32,
    accept: bool,
) -> Result<EventCohost, ServerFnError>;

// Update co-host permissions
#[server]
pub async fn update_cohost_permissions(
    cohost_id: i32,
    permissions: CohostPermissions,
) -> Result<EventCohost, ServerFnError>;

// Remove co-host
#[server]
pub async fn remove_cohost(cohost_id: i32) -> Result<(), ServerFnError>;
```

### Request/Response Types

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub event_type: EventType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timezone: String,
    pub location_type: LocationType,
    pub address: Option<EventAddress>,
    pub virtual_link: Option<String>,
    pub virtual_platform: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub cover_image_url: Option<String>,
    pub capacity: Option<i32>,
    pub waitlist_enabled: bool,
    pub registration_deadline: Option<DateTime<Utc>>,
    pub requires_approval: bool,
    pub visibility: EventVisibility,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RsvpRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub phone_opted_in: bool,
    pub guest_count: i32,
    pub notes: Option<String>,
    // Tracking
    pub source: Option<AttendeeSource>,
    pub referrer_code: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventFilters {
    pub status: Option<EventStatus>,      // draft, published, cancelled
    pub event_type: Option<EventType>,
    pub start_after: Option<DateTime<Utc>>,
    pub start_before: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventSearchQuery {
    pub query: Option<String>,            // Text search
    pub location: Option<GeoLocation>,    // Lat/lng for proximity
    pub distance_miles: Option<i32>,
    pub event_types: Option<Vec<EventType>>,
    pub start_after: Option<DateTime<Utc>>,
    pub start_before: Option<DateTime<Utc>>,
    pub include_virtual: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i32,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}
```

---

## Related Documentation

- [AGENTS.md](../../AGENTS.md) - AI coding context
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Technical architecture
- [ROADMAP.md](../ROADMAP.md) - Development roadmap
- [ORGANIZATIONS.md](ORGANIZATIONS.md) - Organizations & teams feature
- [COMMUNICATIONS.md](COMMUNICATIONS.md) - Email/SMS communications
