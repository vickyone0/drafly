# API Endpoints - cURL Examples

Base URL: `http://localhost:8000`

**Note:** All protected endpoints require a JWT token in the `Authorization` header:
```
Authorization: Bearer <your-jwt-token>
```

---

## Authentication Endpoints (Public)

### 1. Start Google OAuth Flow
Get the OAuth URL to initiate authentication.

```bash
curl -X GET http://localhost:8000/auth/google/start
```

**Response:**
```json
{
  "auth_url": "https://accounts.google.com/o/oauth2/v2/auth?...",
  "state": "uuid-here"
}
```

### 2. Google OAuth Callback
Complete the OAuth flow and receive a JWT token.

```bash
curl -X GET "http://localhost:8000/auth/google/callback?code=AUTH_CODE&state=STATE_VALUE"
```

**Response:**
```json
{
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "email": "user@example.com"
}
```

**Note:** Save the `jwt` token from this response to use in subsequent requests.

---

## Gmail Endpoints (Protected - Requires JWT)

### 3. List Emails
Get a list of emails for the authenticated user.

```bash
curl -X GET http://localhost:8000/emails \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response:**
```json
[
  {
    "id": 1,
    "gmail_id": "abc123",
    "thread_id": "thread123",
    "user_email": "user@example.com",
    "sender": "sender@example.com",
    "to_recipients": "user@example.com",
    "subject": "Email Subject",
    "snippet": "Email snippet...",
    "has_body": true,
    "fetched_at": "2024-01-01T00:00:00"
  }
]
```

### 4. Get Email by ID
Get a specific email by its database ID.

```bash
curl -X GET http://localhost:8000/emails/1 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response:**
```json
{
  "id": 1,
  "gmail_id": "abc123",
  "thread_id": "thread123",
  "user_email": "user@example.com",
  "sender": "sender@example.com",
  "to_recipients": "user@example.com",
  "subject": "Email Subject",
  "snippet": "Email snippet...",
  "body_text": "Full email body text...",
  "body_html": "<html>...</html>",
  "labels": ["INBOX", "UNREAD"],
  "fetched_at": "2024-01-01T00:00:00"
}
```

### 5. Fetch Unread Emails
Fetch and store unread emails from Gmail inbox.

```bash
curl -X POST http://localhost:8000/internal/fetch-unread \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"
```

**Response:**
```json
{
  "fetched": true
}
```

### 6. Fetch Specific Email by Gmail ID
Fetch and store a specific email by its Gmail ID.

```bash
curl -X POST http://localhost:8000/internal/fetch/gmail_message_id_here \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"
```

**Response:**
```json
{
  "ok": true
}
```

---

## Drafts Endpoints (Protected - Requires JWT)

### 7. Generate Draft
Generate an AI-powered draft reply for an email.

```bash
curl -X POST http://localhost:8000/drafts/generate \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "email_id": 1,
    "tone": "friendly"
  }'
```

**Request Body:**
- `email_id` (required): Database ID of the email to reply to
- `tone` (optional): Tone for the draft (default: "friendly")

**Response:**
```json
{
  "draft_id": 1,
  "content": "Generated draft content here..."
}
```

### 8. Get Draft by ID
Retrieve a specific draft by its ID.

```bash
curl -X GET http://localhost:8000/drafts/1 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response:**
```json
{
  "id": 1,
  "email_id": 1,
  "content": "Draft content...",
  "tone": "friendly",
  "status": "draft",
  "created_at": "2024-01-01T00:00:00"
}
```

### 9. Update Draft
Update the content of an existing draft.

```bash
curl -X PATCH http://localhost:8000/drafts/1 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Updated draft content here..."
  }'
```

**Request Body:**
- `content` (required): New draft content

**Response:**
```json
{
  "updated": true
}
```

### 10. Approve Draft
Mark a draft as approved (required before sending).

```bash
curl -X POST http://localhost:8000/drafts/1/approve \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"
```

**Response:**
```json
{
  "approved": true
}
```

### 11. Send Draft
Send an approved draft as an email reply via Gmail.

```bash
curl -X POST http://localhost:8000/drafts/1/send \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"
```

**Response:**
```json
{
  "sent": true,
  "sent_gmail_id": "gmail_message_id_here"
}
```

**Error Response (if draft not approved):**
```
Draft must be approved before sending
```

---

## Complete Workflow Example

Here's a complete workflow example:

```bash
# 1. Start OAuth flow
curl -X GET http://localhost:8000/auth/google/start

# 2. After OAuth callback, you'll receive a JWT token
# Save it to a variable for convenience
export JWT_TOKEN="your-jwt-token-here"

# 3. Fetch unread emails
curl -X POST http://localhost:8000/internal/fetch-unread \
  -H "Authorization: Bearer $JWT_TOKEN"

# 4. List emails
curl -X GET http://localhost:8000/emails \
  -H "Authorization: Bearer $JWT_TOKEN"

# 5. Get a specific email
curl -X GET http://localhost:8000/emails/1 \
  -H "Authorization: Bearer $JWT_TOKEN"

# 6. Generate a draft reply
curl -X POST http://localhost:8000/drafts/generate \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"email_id": 1, "tone": "professional"}'

# 7. Get the draft
curl -X GET http://localhost:8000/drafts/1 \
  -H "Authorization: Bearer $JWT_TOKEN"

# 8. Update the draft if needed
curl -X PATCH http://localhost:8000/drafts/1 \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Updated content"}'

# 9. Approve the draft
curl -X POST http://localhost:8000/drafts/1/approve \
  -H "Authorization: Bearer $JWT_TOKEN"

# 10. Send the draft
curl -X POST http://localhost:8000/drafts/1/send \
  -H "Authorization: Bearer $JWT_TOKEN"
```

---

## Error Responses

### 401 Unauthorized
Returned when JWT token is missing, invalid, or expired.

```bash
# Missing token
curl -X GET http://localhost:8000/emails
# Response: "Missing Authorization header"

# Invalid token
curl -X GET http://localhost:8000/emails \
  -H "Authorization: Bearer invalid_token"
# Response: "Invalid or expired token"
```

### 404 Not Found
Returned when a resource doesn't exist or doesn't belong to the user.

```json
"email not found"
```

### 400 Bad Request
Returned for invalid requests.

```json
"Draft must be approved before sending"
```

