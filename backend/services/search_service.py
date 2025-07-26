from typing import List, Optional
from models.schemas import FileInfo
from repositories.file_repository import search_files_from_db

def search_files(context: Optional[str] = None) -> List[FileInfo]:
    """Search files with optional filter"""
    results = search_files_from_db(context)
    
    files = []
    for row in results:
        files.append(FileInfo(
            file_id=row[0],
            original_name=row[1],
            size=row[2],
            upload_date=row[3]
        ))
    
    return files 