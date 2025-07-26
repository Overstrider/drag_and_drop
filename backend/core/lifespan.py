from contextlib import asynccontextmanager
from fastapi import FastAPI
from apscheduler.schedulers.asyncio import AsyncIOScheduler
from apscheduler.triggers.interval import IntervalTrigger

from core.database import init_database
from services.storage_service import storage_service
from services.sync_service import sync_minio_database
from core.config import settings

# Scheduler para jobs em background
scheduler = AsyncIOScheduler()

@asynccontextmanager
async def lifespan(app: FastAPI):
    """Manage application lifecycle"""
    print("Starting application...")
    
    # initialize database
    print("Initializing database...")
    init_database()
    
    # ensure minio bucket exists
    print("Configuring storage...")
    storage_service.ensure_bucket_exists()
    
    # execute initial sync
    print("Executing initial sync...")
    await sync_minio_database()
    
    # schedule periodic sync
    print(f"Scheduling sync every {settings.sync_interval_minutes} minutes...")
    scheduler.add_job(
        sync_minio_database,
        trigger=IntervalTrigger(minutes=settings.sync_interval_minutes),
        id="sync_job",
        name="Sync MinIO and Database",
        replace_existing=True
    )
    scheduler.start()
    
    print("Application started successfully!")
    
    yield
    
    # SHUTDOWN
    print("Shutting down application...")
    scheduler.shutdown()
    print("Application shut down!") 