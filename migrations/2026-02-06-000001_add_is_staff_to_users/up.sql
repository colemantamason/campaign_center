-- Add is_staff boolean to users table for internal employee/CMS access
ALTER TABLE users ADD COLUMN is_staff BOOLEAN NOT NULL DEFAULT false;
