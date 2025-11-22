-- Add migration script here
ALTER TABLE drafts
    ADD COLUMN updated_at TIMESTAMP;