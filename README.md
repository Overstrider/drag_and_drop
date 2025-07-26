# Drag & Drop

It’s a one-page app just for uploading files, with a search engine that searches inside the files.

## Tech Stack

### Frontend
- **React + TypeScript**
- **Tailwind CSS**
- **Vite**
- **Brutalism UI**

### Backend
- **Python + FastAPI**
- **Rust File Parser** (Custom library built with PyO3 for efficient multi-format file parsing)
- **SQLite FTS5** (Full-text search database with excellent performance for content indexing)

### Infrastructure
- **Docker**
- **MinIO** (S3-compatible object storage for local file management)


## How to run it locally.

1. Make sure you have Docker and Docker Compose installed.

2. Start the Docker container:
```bash
docker-compose up --build
```

3. Access the app:
- Frontend: http://localhost:3000
- API Backend: http://localhost:8000/docs