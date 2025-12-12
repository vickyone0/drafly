# Drafly - AI-Powered Email Assistant

Drafly is a modern email management application that helps you manage your Gmail inbox and generate professional email replies using AI. Built with Rust (Actix-web) backend and Next.js frontend.

## ✨ Features

- 📧 **Gmail Integration**: Fetch and manage emails from your Gmail account
- 🤖 **AI-Powered Drafts**: Generate professional email replies using Groq AI
- ✍️ **Draft Management**: Create, edit, approve, and send email drafts
- 🔐 **Secure Authentication**: Google OAuth 2.0 with JWT token-based authentication
- 🎨 **Modern UI**: Beautiful, responsive interface built with Next.js, React, and Tailwind CSS
- 📱 **Responsive Design**: Works seamlessly on desktop and mobile devices

## 🛠️ Tech Stack

### Backend
- **Rust** - Systems programming language
- **Actix-web** - High-performance web framework
- **SQLx** - Async SQL toolkit with compile-time checked queries
- **PostgreSQL** - Database
- **jsonwebtoken** - JWT token handling
- **reqwest** - HTTP client for Gmail API integration
- **Groq AI** - AI-powered draft generation

### Frontend
- **Next.js 16** - React framework with App Router
- **React 19** - UI library
- **TypeScript** - Type safety
- **Tailwind CSS** - Utility-first CSS framework
- **shadcn/ui** - High-quality component library
- **Lucide React** - Icon library

## 📋 Prerequisites

- **Rust** (latest stable version)
- **Node.js** 20.9.0 or higher
- **PostgreSQL** 12 or higher
- **Google Cloud Project** with Gmail API enabled
- **Groq API Key** for AI draft generation

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone <repository-url>
cd drafly
```

### 2. Backend Setup

#### Install Dependencies

```bash
# Rust dependencies will be installed automatically with cargo
```

#### Database Setup

1. Create a PostgreSQL database:
```bash
createdb drafly
```

2. Run migrations:
```bash
# Check migrations directory for SQL files
# Or use sqlx-cli if installed:
sqlx migrate run
```

#### Environment Variables

Create a `.env` file in the root directory:

```env
# Database
DATABASE_URL=postgresql://user:password@localhost/drafly

# JWT Secret (generate a random string)
JWT_SECRET=your-super-secret-jwt-key-here

# Google OAuth
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URI=http://localhost:8000/auth/google/callback

# Groq AI
GROQ_API_KEY=your-groq-api-key

# Frontend URL (for OAuth redirect)
FRONTEND_URL=http://localhost:3000
```

#### Run Backend

```bash
cargo run
```

The backend will start on `http://localhost:8000`

### 3. Frontend Setup

#### Install Dependencies

```bash
cd frontend
npm install
```

#### Environment Variables

Create a `.env.local` file in the `frontend` directory:

```env
NEXT_PUBLIC_API_URL=http://localhost:8000
```

#### Run Frontend

```bash
npm run dev
```

The frontend will start on `http://localhost:3000`

## 📖 API Documentation

### Authentication Endpoints

- `GET /auth/google/start` - Start Google OAuth flow
- `GET /auth/google/callback` - OAuth callback (returns JWT token)

### Email Endpoints

- `GET /emails` - List user's emails (requires JWT)
- `GET /emails/{id}` - Get specific email (requires JWT)
- `POST /internal/fetch-unread` - Fetch unread emails from Gmail (requires JWT)
- `POST /internal/fetch/{gmail_id}` - Fetch specific email by Gmail ID (requires JWT)

### Draft Endpoints

- `GET /drafts` - List all drafts (requires JWT)
- `POST /drafts/generate` - Generate AI draft reply (requires JWT)
- `GET /drafts/{id}` - Get draft by ID (requires JWT)
- `PATCH /drafts/{id}` - Update draft content (requires JWT)
- `POST /drafts/{id}/approve` - Approve draft (requires JWT)
- `POST /drafts/{id}/send` - Send approved draft (requires JWT)

For detailed API documentation with curl examples, see [API_ENDPOINTS.md](./API_ENDPOINTS.md)

## 🔐 Authentication Flow

1. User clicks "Sign in with Google" on the frontend
2. Frontend calls `/auth/google/start` to get OAuth URL
3. User is redirected to Google OAuth consent screen
4. After consent, Google redirects to backend callback URL
5. Backend exchanges authorization code for tokens
6. Backend generates JWT token and redirects to frontend
7. Frontend stores JWT token in localStorage
8. All subsequent API calls include JWT in Authorization header

## 📁 Project Structure

```
drafly/
├── src/                    # Backend Rust source code
│   ├── routes/            # API route handlers
│   │   ├── auth.rs        # Authentication routes
│   │   ├── gmail.rs       # Email management routes
│   │   └── drafts.rs      # Draft management routes
│   ├── services/          # Business logic
│   │   ├── jwt.rs         # JWT token handling
│   │   ├── google_oauth.rs # Google OAuth integration
│   │   ├── gmail_fetcher.rs # Gmail API integration
│   │   ├── gmail_sender.rs  # Send email via Gmail
│   │   └── groq_ai.rs      # AI draft generation
│   ├── db/                # Database utilities
│   ├── middleware.rs      # JWT authentication middleware
│   └── main.rs            # Application entry point
├── frontend/              # Next.js frontend
│   ├── app/               # Next.js app directory
│   │   ├── page.tsx       # Inbox page
│   │   ├── drafts/        # Drafts page
│   │   ├── login/         # Login page
│   │   └── settings/      # Settings page
│   ├── components/        # React components
│   │   ├── ui/            # shadcn/ui components
│   │   ├── Sidebar.tsx    # Navigation sidebar
│   │   ├── EmailList.tsx  # Email list view
│   │   ├── EmailPreview.tsx # Email preview
│   │   └── DraftEditor.tsx # Draft editor
│   └── lib/               # Utilities
│       ├── api.ts         # API client
│       └── utils.ts       # Helper functions
├── migrations/            # Database migrations
└── Cargo.toml            # Rust dependencies
```

## 🧪 Development

### Backend Development

```bash
# Run with hot reload (requires cargo-watch)
cargo watch -x run

# Run tests
cargo test

# Check for errors
cargo check
```

### Frontend Development

```bash
cd frontend

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Lint code
npm run lint
```

## 🚢 Deployment

### Backend Deployment

1. Build the release binary:
```bash
cargo build --release
```

2. Set environment variables on your hosting platform
3. Run migrations on your production database
4. Start the server:
```bash
./target/release/drafly
```

### Frontend Deployment (Vercel)

1. Connect your repository to Vercel
2. Set environment variables:
   - `NEXT_PUBLIC_API_URL` - Your backend API URL
3. Deploy

The frontend is configured with Suspense boundaries for proper SSR support.

## 🔧 Configuration

### Google OAuth Setup

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select existing one
3. Enable Gmail API
4. Create OAuth 2.0 credentials
5. Add authorized redirect URI: `http://localhost:8000/auth/google/callback`
6. Copy Client ID and Client Secret to `.env`

### Groq AI Setup

1. Sign up at [Groq](https://groq.com/)
2. Get your API key
3. Add to `.env` as `GROQ_API_KEY`

## 📝 License

[Add your license here]

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📧 Support

For issues and questions, please open an issue on GitHub.

## 🙏 Acknowledgments

- Built with [Actix-web](https://actix.rs/)
- Frontend powered by [Next.js](https://nextjs.org/)
- UI components from [shadcn/ui](https://ui.shadcn.com/)
- AI powered by [Groq](https://groq.com/)
pulling

