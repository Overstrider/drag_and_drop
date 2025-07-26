import sqlite3
from pathlib import Path
from .config import settings

def init_database():
    """initialize rust db pool (also creates the table)"""
    Path(settings.database_path).parent.mkdir(parents=True, exist_ok=True)
    
    try:
        import rust_parser
    except ImportError:
        raise RuntimeError("Error importing rust_parser")
    
    try:
        rust_parser.init_database(settings.database_path)
        print(f"Rust DB pool and table initialized: {settings.database_path}")
    except Exception as e:
        print(f"Error initializing Rust DB pool: {e}")
        raise

def get_db_connection():
    """Get direct connection to SQLite (for Python queries)"""
    return sqlite3.connect(settings.database_path) 