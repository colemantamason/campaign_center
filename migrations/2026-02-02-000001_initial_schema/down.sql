-- Rollback initial schema

DROP TRIGGER IF EXISTS update_event_shifts_updated_at ON event_shifts;
DROP TRIGGER IF EXISTS update_events_updated_at ON events;
DROP TRIGGER IF EXISTS update_organizations_updated_at ON organizations;
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP FUNCTION IF EXISTS update_updated_at_column();

DROP TABLE IF EXISTS notifications;
DROP TABLE IF EXISTS event_signups;
DROP TABLE IF EXISTS event_shifts;
DROP TABLE IF EXISTS events;
DROP TABLE IF EXISTS password_reset_tokens;
DROP TABLE IF EXISTS invitations;
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS organization_members;
DROP TABLE IF EXISTS organizations;
DROP TABLE IF EXISTS users;

DROP EXTENSION IF EXISTS "uuid-ossp";
