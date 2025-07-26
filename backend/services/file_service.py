import uuid
import tempfile

from models.schemas import (
    UploadResponse, DownloadResponse, 
    NotifyUploadRequest, SuccessResponse
)
from services.storage_service import storage_service
from services.processing_service import process_file_from_temp
from core.decorators import handle_http_errors, handle_file_not_found

class FileService:
    """Service for file operations"""
    
    @staticmethod
    @handle_http_errors
    async def get_upload_url() -> UploadResponse:
        """Generate presigned URL for direct upload to MinIO"""
        file_id = str(uuid.uuid4())
        
        # generate presigned URL for direct upload to MinIO
        presigned_url = storage_service.generate_presigned_upload_url(file_id)
        
        return UploadResponse(
            file_id=file_id,
            presigned_url=presigned_url
        )

    @staticmethod
    @handle_file_not_found
    async def notify_upload(request: NotifyUploadRequest) -> SuccessResponse:
        """Notify that upload is complete and process file"""
        # check if file exists in MinIO
        stat = storage_service.get_file_stats(request.file_id)
        print(f"File found in MinIO: {request.file_id}, size: {stat.size}")
        
        # download file temporarily to process
        with tempfile.NamedTemporaryFile() as tmp:
            storage_service.download_file_to_temp(request.file_id, tmp.name)
            print(f"File downloaded to: {tmp.name}")
            
            # Process file
            process_file_from_temp(
                temp_path=tmp.name,
                file_id=request.file_id,
                original_name=request.file_name,
                size=stat.size
            )
        
        print(f"File processed and indexed: {request.file_name}")
        return SuccessResponse(status="success", message="File processed successfully")
    
    @staticmethod
    @handle_file_not_found
    async def get_download_url(file_id: str) -> DownloadResponse:
        """Generate presigned URL for secure download (alternative method)"""
        presigned_url = storage_service.generate_presigned_download_url(file_id)
        return DownloadResponse(presigned_url=presigned_url)
    
file_service = FileService() 