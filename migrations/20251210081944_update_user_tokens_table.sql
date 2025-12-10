-- Add migration script here
ALTER TABLE user_tokens
ADD CONSTRAINT user_tokens_email_unique UNIQUE (email);
