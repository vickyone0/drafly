-- Add migration script here
CREATE TABLE emails (
    id SERIAL PRIMARY KEY,
    gmail_id TEXT UNIQUE NOT NULL,
    thread_id TEXT,
    user_email TEXT,
    sender TEXT,
    to_recipients TEXT,
    subject TEXT,
    snippet TEXT,
    body_text TEXT,
    body_html TEXT,
    labels TEXT[],
    fetched_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE drafts (
    id SERIAL PRIMARY KEY,
    user_email TEXT,
    email_id INTEGER REFERENCES emails(id),
    content TEXT,
    tone TEXT,
    status TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);