# Drafly Frontend

A modern, professional email management interface built with Next.js, React, Tailwind CSS, and shadcn/ui.

## Features

- 📧 **Email Management**: View and manage your emails from Gmail
- ✍️ **AI-Powered Drafts**: Generate professional email replies using AI
- 🎨 **Modern UI**: Beautiful, responsive interface with shadcn/ui components
- 🔐 **Secure Authentication**: Google OAuth integration with JWT tokens
- 📱 **Responsive Design**: Works seamlessly on desktop and mobile

## Tech Stack

- **Next.js 16** - React framework with App Router
- **React 19** - UI library
- **TypeScript** - Type safety
- **Tailwind CSS** - Utility-first CSS framework
- **shadcn/ui** - High-quality component library
- **Lucide React** - Icon library

## Getting Started

### Prerequisites

- Node.js 20.9.0 or higher (recommended)
- npm or yarn
- Backend API running on `http://localhost:8000`

### Installation

1. Install dependencies:
```bash
npm install
```

2. Create a `.env.local` file:
```bash
cp .env.local.example .env.local
```

3. Update `.env.local` with your API URL:
```
NEXT_PUBLIC_API_URL=http://localhost:8000
```

4. Run the development server:
```bash
npm run dev
```

5. Open [http://localhost:3000](http://localhost:3000) in your browser

## Project Structure

```
frontend/
├── app/                    # Next.js app directory
│   ├── page.tsx           # Inbox page
│   ├── drafts/            # Drafts page
│   ├── login/             # Login page
│   ├── settings/          # Settings page
│   ├── layout.tsx         # Root layout
│   └── globals.css        # Global styles
├── components/            # React components
│   ├── ui/               # shadcn/ui components
│   ├── Sidebar.tsx       # Navigation sidebar
│   ├── EmailList.tsx     # Email list view
│   ├── EmailPreview.tsx   # Email preview pane
│   └── DraftEditor.tsx   # Draft editor
├── lib/                   # Utilities
│   ├── api.ts            # API client
│   └── utils.ts           # Utility functions
└── middleware.ts          # Next.js middleware
```

## Features Overview

### Inbox Page (`/`)
- View list of emails
- Fetch unread emails from Gmail
- Preview email content
- Generate AI-powered draft replies

### Drafts Page (`/drafts`)
- View and edit drafts
- Save draft changes
- Approve drafts
- Send approved drafts

### Settings Page (`/settings`)
- View account information
- Logout functionality

### Login Page (`/login`)
- Google OAuth authentication
- JWT token management

## API Integration

The frontend communicates with the backend API through the `lib/api.ts` client. All API calls include JWT authentication tokens automatically.

## Development

### Build for Production

```bash
npm run build
```

### Start Production Server

```bash
npm start
```

### Linting

```bash
npm run lint
```

## Environment Variables

- `NEXT_PUBLIC_API_URL` - Backend API URL (default: `http://localhost:8000`)

## Authentication Flow

1. User clicks "Sign in with Google" on login page
2. Redirected to Google OAuth consent screen
3. After consent, redirected back with authorization code
4. Frontend exchanges code for JWT token
5. JWT token stored in localStorage
6. All subsequent API calls include JWT in Authorization header

## Notes

- The app requires the backend API to be running
- JWT tokens are stored in localStorage
- Token expiration is handled by redirecting to login page
- All protected routes require valid JWT token
