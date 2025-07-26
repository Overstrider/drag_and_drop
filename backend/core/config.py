from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    # MinIO Configuration
    minio_endpoint: str = "localhost:9000"
    minio_access_key: str = "minioadmin"
    minio_secret_key: str = "minioadmin"
    minio_secure: bool = False
    external_minio_endpoint: str = "localhost:9000"
    bucket_name: str = "files"
    
    # Database Configuration
    database_path: str = "./data/files.db"
    
    # Application Configuration
    sync_interval_minutes: int = 5

# global instance of the settings
settings = Settings() 