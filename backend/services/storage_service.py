from minio import Minio
from minio.error import S3Error
from datetime import timedelta
from core.config import settings

class StorageService:
    """Service for MinIO storage operations"""
    
    def __init__(self):
        self.client = Minio(
            settings.minio_endpoint,
            access_key=settings.minio_access_key,
            secret_key=settings.minio_secret_key,
            secure=settings.minio_secure
        )
        self.bucket_name = settings.bucket_name
    
    def ensure_bucket_exists(self):
        """Ensure bucket exists"""
        try:
            if not self.client.bucket_exists(self.bucket_name):
                self.client.make_bucket(self.bucket_name)
                print(f"Bucket '{self.bucket_name}' created successfully")
        except Exception as e:
            print(f"Error checking/creating bucket: {e}")
            raise
    
    def download_file(self, file_id: str) -> bytes:
        """Download file from MinIO"""
        try:
            response = self.client.get_object(self.bucket_name, file_id)
            return response.read()
        except S3Error as e:
            if e.code == 'NoSuchKey':
                raise FileNotFoundError(f"File {file_id} not found")
            raise
    
    def get_file_stats(self, file_id: str):
        """Get file statistics from MinIO"""
        try:
            return self.client.stat_object(self.bucket_name, file_id)
        except S3Error as e:
            if e.code == 'NoSuchKey':
                raise FileNotFoundError(f"File {file_id} not found")
            raise
    
    def generate_presigned_download_url(self, file_id: str) -> str:
        """Generate presigned URL for download"""
        try:
            # Check if file exists
            self.client.stat_object(self.bucket_name, file_id)
            
            # Generate presigned URL for download (valid for 1 hour)
            presigned_url = self.client.presigned_get_object(
                self.bucket_name,
                file_id,
                expires=timedelta(hours=1)
            )
            
            # Replace internal hostname with external for browser access
            presigned_url = presigned_url.replace(
                f"http://{settings.minio_endpoint}", 
                f"http://{settings.external_minio_endpoint}"
            )
            presigned_url = presigned_url.replace(
                f"https://{settings.minio_endpoint}", 
                f"https://{settings.external_minio_endpoint}"
            )
            
            return presigned_url
        except S3Error as e:
            if e.code == 'NoSuchKey':
                raise FileNotFoundError(f"File {file_id} not found")
            raise
    
    def generate_presigned_upload_url(self, file_id: str) -> str:
        """Generate presigned URL for direct upload"""
        try:
            # generate presigned URL for upload (valid for 1 hour)
            presigned_url = self.client.presigned_put_object(
                self.bucket_name,
                file_id,
                expires=timedelta(hours=1)
            )
            
            # replace internal hostname with nginx proxy for frontend access
            # only for local development, 
            # if use S3, 
            # we can use the S3 endpoint directly
            presigned_url = presigned_url.replace(
                f"http://{settings.minio_endpoint}", 
                "/minio"
            )
            presigned_url = presigned_url.replace(
                f"https://{settings.minio_endpoint}", 
                "/minio"
            )
            
            return presigned_url
        except Exception as e:
            print(f"Error generating upload URL: {e}")
            raise
    
    def list_all_files(self):
        """List all files in MinIO"""
        try:
            return list(self.client.list_objects(self.bucket_name))
        except Exception as e:
            print(f"Error listing objects in MinIO: {e}")
            return []
    
    def download_file_to_temp(self, file_id: str, temp_path: str):
        """Download file to temporary path"""
        try:
            self.client.fget_object(self.bucket_name, file_id, temp_path)
        except S3Error as e:
            if e.code == 'NoSuchKey':
                raise FileNotFoundError(f"File {file_id} not found")
            raise

storage_service = StorageService() 