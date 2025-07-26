from typing import List, Optional, Tuple
from core.database import get_db_connection

def search_files_from_db(context: Optional[str] = None) -> List[Tuple[str, str, int, str]]:
    conn = get_db_connection()
    cursor = conn.cursor()
    
    try:
        if context:
            # search by name and content simultaneously using FTS5
            try:
                cursor.execute("""
                    SELECT file_id, original_name, size, upload_date 
                    FROM files 
                    WHERE original_name LIKE ? OR content MATCH ?
                    ORDER BY rank
                """, (f'%{context}%', f'{context}*'))
                results = cursor.fetchall()
            except Exception as e:
                print(f"Error in FTS5 search: {e}")
                # fallback to simple LIKE search
                cursor.execute("""
                    SELECT file_id, original_name, size, upload_date 
                    FROM files 
                    WHERE original_name LIKE ? OR content LIKE ?
                    ORDER BY upload_date DESC
                """, (f'%{context}%', f'%{context}%'))
                results = cursor.fetchall()
        else:
            cursor.execute("""
                SELECT file_id, original_name, size, upload_date 
                FROM files
                ORDER BY upload_date DESC
            """)
            results = cursor.fetchall()
        
        return results
    finally:
        conn.close()