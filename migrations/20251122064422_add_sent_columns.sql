-- Add migration script here
ALTER TABLE drafts
ADD COLUMN sent BOOLEAN DEFAULT FALSE,
ADD COLUMN sent_gmail_id TEXT;