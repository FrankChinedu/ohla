# Ohla Monorepo

A monorepo containing the Ohla frontend (Next.js) and backend (Rust/Axum).

## Structure

```
ohla/
├── frontend/          # Next.js frontend application
├── backend/           # Rust Axum backend API
├── package.json       # Root package.json with workspace configuration
└── README.md          # This file
```

## Prerequisites

- Node.js >= 20.0.0
- npm >= 10.0.0
- Rust (latest stable version)
- Cargo

## Getting Started

### Install Dependencies

Install all dependencies for the monorepo:

```bash
npm install
```

This will install dependencies for both the frontend workspace and the root workspace.

### Development

Run both frontend and backend in development mode:

```bash
npm run dev
```

Or run them individually:

```bash
# Frontend only (Next.js dev server)
npm run dev:frontend

# Backend only (Rust with hot reload)
npm run dev:backend
```

### Building

Build both projects:

```bash
npm run build
```

Or build individually:

```bash
# Frontend only
npm run build:frontend

# Backend only
npm run build:backend
```

### Running Production Builds

```bash
# Frontend
npm run start:frontend

# Backend
npm run start:backend
```

### Other Commands

```bash
# Lint frontend code
npm run lint

# Clean all build artifacts and dependencies
npm run clean
```

## Frontend

The frontend is a Next.js application built with:
- React 19
- TypeScript
- Tailwind CSS
- Next.js 16

See [frontend/README.md](frontend/README.md) for more details.

## Backend

The backend is a Rust application built with:
- Axum web framework
- Tokio async runtime
- Tower middleware

See [backend/README.md](backend/README.md) for more details.

## Workspace Management

This monorepo uses npm workspaces. The frontend is configured as a workspace in the root [package.json](package.json).

To add dependencies to a specific workspace:

```bash
# Add to frontend
npm install <package> --workspace=frontend

# Add to root
npm install <package> -w root
```

## License

[Your License Here]
