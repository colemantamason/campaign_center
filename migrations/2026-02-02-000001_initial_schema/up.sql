-- Campaign Center Initial Schema
-- Users, Organizations, Members, Sessions, Invitations, Events, Notifications

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-------------------------------------------------------------------------------
-- USERS
-------------------------------------------------------------------------------
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    
    -- Credentials
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified_at TIMESTAMPTZ,
    password_hash VARCHAR(255) NOT NULL,
    
    -- Profile
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone_number VARCHAR(20),
    phone_number_verified_at TIMESTAMPTZ,
    avatar_url TEXT,
    
    -- Preferences
    timezone VARCHAR(50) NOT NULL DEFAULT 'America/New_York',
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users(email);

-------------------------------------------------------------------------------
-- ORGANIZATIONS
-------------------------------------------------------------------------------
CREATE TABLE organizations (
    id SERIAL PRIMARY KEY,
    
    -- Basic Info
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    organization_type VARCHAR(50) NOT NULL,
    description TEXT,
    avatar_url TEXT,
    website_url TEXT,
    
    -- Contact
    email VARCHAR(255),
    phone_number VARCHAR(20),
    
    -- Address
    address_line_1 VARCHAR(255),
    address_line_2 VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(50),
    zip_code VARCHAR(20),
    country VARCHAR(2) DEFAULT 'US',
    
    -- Settings
    timezone VARCHAR(50) NOT NULL DEFAULT 'America/New_York',
    subscriptions TEXT[] NOT NULL DEFAULT '{}',
    
    -- Metadata
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_organizations_slug ON organizations(slug);
CREATE INDEX idx_organizations_created_by ON organizations(created_by);

-------------------------------------------------------------------------------
-- ORGANIZATION MEMBERS
-------------------------------------------------------------------------------
CREATE TABLE organization_members (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Role: 'owner', 'admin', 'manager', 'member'
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    
    -- Metadata
    invited_by INTEGER REFERENCES users(id),
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_active_at TIMESTAMPTZ,
    
    UNIQUE(organization_id, user_id)
);

CREATE INDEX idx_org_members_org ON organization_members(organization_id);
CREATE INDEX idx_org_members_user ON organization_members(user_id);

-------------------------------------------------------------------------------
-- SESSIONS
-------------------------------------------------------------------------------
CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    
    -- Token (UUID string, stored in cookie)
    token UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    
    -- User
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Current organization membership context
    active_organization_membership_id INTEGER REFERENCES organization_members(id) ON DELETE SET NULL,
    
    -- Session info
    user_agent TEXT,
    ip_address INET,
    platform VARCHAR(20) NOT NULL DEFAULT 'web',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    last_accessed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_token ON sessions(token);
CREATE INDEX idx_sessions_user ON sessions(user_id);
CREATE INDEX idx_sessions_expires ON sessions(expires_at);
CREATE INDEX idx_sessions_platform ON sessions(platform);

-------------------------------------------------------------------------------
-- INVITATIONS
-------------------------------------------------------------------------------
CREATE TABLE invitations (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    
    -- Invitee
    email VARCHAR(255) NOT NULL,
    
    -- Role to assign
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    
    -- Token for accepting
    token UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    
    -- Status: 'pending', 'accepted', 'expired'
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    
    -- Timestamps
    invited_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    
    -- Only one pending invitation per email per org
    UNIQUE(organization_id, email)
);

CREATE INDEX idx_invitations_token ON invitations(token);
CREATE INDEX idx_invitations_org ON invitations(organization_id);
CREATE INDEX idx_invitations_email ON invitations(email);

-------------------------------------------------------------------------------
-- PASSWORD RESET TOKENS
-------------------------------------------------------------------------------
CREATE TABLE password_reset_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ
);

CREATE INDEX idx_password_reset_token ON password_reset_tokens(token);
CREATE INDEX idx_password_reset_user ON password_reset_tokens(user_id);

-------------------------------------------------------------------------------
-- EVENTS
-------------------------------------------------------------------------------
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    
    -- Basic Info
    name VARCHAR(255) NOT NULL,
    event_type VARCHAR(50) NOT NULL DEFAULT 'other',
    visibility VARCHAR(20) NOT NULL DEFAULT 'draft',
    description TEXT,
    attendee_message TEXT,
    image_url TEXT,
    
    -- Location
    location_in_person TEXT,
    location_online TEXT,
    
    -- Communication Settings
    communication_bring_a_friend BOOLEAN NOT NULL DEFAULT false,
    communication_other_events BOOLEAN NOT NULL DEFAULT false,
    communication_confirmation BOOLEAN NOT NULL DEFAULT true,
    communication_check_in BOOLEAN NOT NULL DEFAULT true,
    
    -- Contact Info
    contact_name VARCHAR(255) NOT NULL,
    contact_email VARCHAR(255),
    contact_phone VARCHAR(20),
    
    -- Related Entities (stored as arrays of IDs or slugs)
    co_hosts TEXT[] NOT NULL DEFAULT '{}',
    invite_groups TEXT[] NOT NULL DEFAULT '{}',
    
    -- Metadata
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_events_org ON events(organization_id);
CREATE INDEX idx_events_type ON events(event_type);
CREATE INDEX idx_events_visibility ON events(visibility);

-------------------------------------------------------------------------------
-- EVENT SHIFTS
-------------------------------------------------------------------------------
CREATE TABLE event_shifts (
    id SERIAL PRIMARY KEY,
    event_id INTEGER NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    
    -- Shift Info
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) NOT NULL DEFAULT 'America/New_York',
    capacity INTEGER,
    notes TEXT,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_event_shifts_event ON event_shifts(event_id);
CREATE INDEX idx_event_shifts_times ON event_shifts(start_time, end_time);

-------------------------------------------------------------------------------
-- EVENT SIGNUPS
-------------------------------------------------------------------------------
CREATE TABLE event_signups (
    id SERIAL PRIMARY KEY,
    event_shift_id INTEGER NOT NULL REFERENCES event_shifts(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Signup Info
    status VARCHAR(20) NOT NULL DEFAULT 'signed_up',
    notes TEXT,
    
    -- Timestamps
    signed_up_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    checked_in_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    
    -- Only one signup per user per shift
    UNIQUE(event_shift_id, user_id)
);

CREATE INDEX idx_event_signups_shift ON event_signups(event_shift_id);
CREATE INDEX idx_event_signups_user ON event_signups(user_id);
CREATE INDEX idx_event_signups_status ON event_signups(status);

-------------------------------------------------------------------------------
-- NOTIFICATIONS
-------------------------------------------------------------------------------
CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id INTEGER REFERENCES organizations(id) ON DELETE CASCADE,
    
    -- Notification Content
    notification_type VARCHAR(50) NOT NULL DEFAULT 'info',
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    link TEXT,
    
    -- Status
    read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON notifications(user_id);
CREATE INDEX idx_notifications_org ON notifications(organization_id);
CREATE INDEX idx_notifications_unread ON notifications(user_id, read) WHERE read = false;

-------------------------------------------------------------------------------
-- TRIGGER: Update updated_at timestamp
-------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_organizations_updated_at
    BEFORE UPDATE ON organizations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_events_updated_at
    BEFORE UPDATE ON events
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_event_shifts_updated_at
    BEFORE UPDATE ON event_shifts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
