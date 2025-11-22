# Quick Start Guide

## Issues Fixed

✅ Created `.env.local.example` file

## Node.js Version Issue

**Current:** Node.js v18.17.0  
**Required:** Node.js >=20.9.0

### Quick Fix - Upgrade Node.js

If you have `nvm` installed:
```bash
nvm install 20
nvm use 20
```

If you don't have `nvm`:
```bash
# Install nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# Reload shell
source ~/.bashrc

# Install and use Node.js 20
nvm install 20
nvm use 20
```

### Alternative - Downgrade Next.js (if you can't upgrade Node)

```bash
cd frontend
npm install next@15 react@18 react-dom@18 @types/react@18 @types/react-dom@18
```

## Setup Steps

1. **Create environment file:**
```bash
cd frontend
cp .env.local.example .env.local
# Edit .env.local if needed (default should work)
```

2. **Start the app:**
```bash
npm run dev
```

3. **Open browser:**
Navigate to http://localhost:3000

## Verify Setup

```bash
# Check Node version (should be >=20.9.0)
node --version

# Check if .env.local exists
ls -la .env.local

# Check if backend is running
curl http://localhost:8000/auth/google/start
```

## Troubleshooting

- **Node version error:** See NODE_VERSION.md for detailed solutions
- **Port 3000 in use:** Change port in package.json or kill the process
- **API connection error:** Make sure backend is running on port 8000

