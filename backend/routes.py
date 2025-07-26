from fastapi import APIRouter, Query
from typing import Optional, List

from models.schemas import (
    UploadResponse, DownloadResponse,
    NotifyUploadRequest, FileInfo, SuccessResponse
)
from services.file_service import file_service
from services.search_service import search_files

# Router principal
router = APIRouter()

@router.get("/upload", response_model=UploadResponse)
async def get_upload_url():
    """Generate presigned URL from MinIO for direct upload"""
    return await file_service.get_upload_url()

@router.post("/notify-upload", response_model=SuccessResponse)
async def notify_upload(request: NotifyUploadRequest):
    """Notify that upload is complete and process file"""
    return await file_service.notify_upload(request)

@router.get("/files", response_model=List[FileInfo])
async def list_files(context: Optional[str] = Query(None, description="Filter by name or content")):
    """List files with optional filter"""
    return search_files(context)

@router.get("/download/{file_id}", response_model=DownloadResponse)
async def get_download_url(file_id: str):
    """Generate presigned URL for secure download"""
    return await file_service.get_download_url(file_id)