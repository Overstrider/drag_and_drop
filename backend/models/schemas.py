from pydantic import BaseModel

class UploadResponse(BaseModel):
    file_id: str
    presigned_url: str

class NotifyUploadRequest(BaseModel):
    file_id: str
    file_name: str

class FileInfo(BaseModel):
    file_id: str
    original_name: str
    size: int
    upload_date: str

class DownloadResponse(BaseModel):
    presigned_url: str

class SuccessResponse(BaseModel):
    status: str
    message: str 