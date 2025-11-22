#!/bin/bash

# API Endpoints - Quick cURL Reference
# Base URL: http://localhost:8000
# Replace YOUR_JWT_TOKEN with your actual JWT token

BASE_URL="http://localhost:8000"
JWT_TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ2aWduZXNoczAuNjE4QGdtYWlsLmNvbSIsImV4cCI6MTc2NDQwNjEwOX0.EB1ZE6DFA5l0cALSE8E2asSyc8DasLZ1nWLLbqisR3U"

echo "=== Authentication Endpoints (Public) ==="
echo ""

echo "# 1. Start Google OAuth Flow"
curl -X GET "$BASE_URL/auth/google/start"
echo -e "\n"

echo "# 2. Google OAuth Callback"
curl -X GET "$BASE_URL/auth/google/callback?code=AUTH_CODE&state=STATE_VALUE"
echo -e "\n"

echo "=== Gmail Endpoints (Protected) ==="
echo ""

echo "# 3. List Emails"
curl -X GET "$BASE_URL/emails" \
  -H "Authorization: Bearer $JWT_TOKEN"
echo -e "\n"

echo "# 4. Get Email by ID"
curl -X GET "$BASE_URL/emails/1" \
  -H "Authorization: Bearer $JWT_TOKEN"
echo -e "\n"

echo "# 5. Fetch Unread Emails"
curl -X POST "$BASE_URL/internal/fetch-unread" \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json"
echo -e "\n"

echo "# 6. Fetch Specific Email by Gmail ID"
curl -X POST "$BASE_URL/internal/fetch/gmail_message_id_here" \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json"
echo -e "\n"

echo "=== Drafts Endpoints (Protected) ==="
echo ""

echo "# 7. Generate Draft"
curl -X POST "$BASE_URL/drafts/generate" \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"email_id": 1, "tone": "friendly"}'
echo -e "\n"

echo "# 8. Get Draft by ID"
curl -X GET "$BASE_URL/drafts/1" \
  -H "Authorization: Bearer $JWT_TOKEN"
echo -e "\n"

echo "# 9. Update Draft"
curl -X PATCH "$BASE_URL/drafts/1" \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Updated draft content here..."}'
echo -e "\n"

echo "# 10. Approve Draft"
curl -X POST "$BASE_URL/drafts/15/approve" \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json"
echo -e "\n"

echo "# 11. Send Draft"
curl -X POST "$BASE_URL/drafts/15/send" \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json"
echo -e "\n"

