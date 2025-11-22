-- Add migration script here
CREATE TABLE user_tokens (
    id SERIAL PRIMARY KEY,
    email TEXT,
    refresh_token TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
