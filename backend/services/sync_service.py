import tempfile
import os
from services.storage_service import storage_service
from services.processing_service import process_file_from_temp

async def sync_minio_database():
    """Synchronize files between MinIO and database"""
    print("Starting MinIO <-> Database synchronization")
    
    # Import Rust parser
    try:
        import rust_parser
    except ImportError:
        print("Error: rust_parser not available for synchronization")
        return
    
    try:
        # Get all objects from MinIO
        minio_files = set()
        try:
            objects = storage_service.list_all_files()
            for obj in objects:
                minio_files.add(obj.object_name)
        except Exception as e:
            print(f"Error listing MinIO objects: {e}")
            return
        
        # Process files that are not in the database
        processed = 0
        for file_id in minio_files:
            try:
                # Check if file already exists in the database
                if rust_parser.check_file_exists(file_id):
                    continue
                
                # Get file information
                stat = storage_service.get_file_stats(file_id)
                
                # Download file temporarily to process
                with tempfile.NamedTemporaryFile(delete=False) as tmp:
                    storage_service.download_file_to_temp(file_id, tmp.name)
                    
                    # Process and store using Rust
                    process_file_from_temp(
                        temp_path=tmp.name, 
                        file_id=file_id, 
                        original_name=file_id,  # Use file_id as name if no metadata
                        size=stat.size, 
                        upload_date=stat.last_modified.isoformat()
                    )
                    
                    os.unlink(tmp.name)
                
                processed += 1
                print(f"Processed: {file_id}")
                
            except Exception as e:
                print(f"Error synchronizing file {file_id}: {e}")
        
        print(f"Synchronization completed. Processed: {processed}")
    
    except Exception as e:
        print(f"General error in synchronization: {e}")
        import traceback
        traceback.print_exc() 