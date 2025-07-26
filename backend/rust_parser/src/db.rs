use pyo3::prelude::*;
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use std::collections::VecDeque;

// sqlite connection pool
pub struct SqlitePool {
    connections: Arc<Mutex<VecDeque<Connection>>>,
    db_path: String,
    max_connections: usize,
}

// aux function to get connection from pool
fn with_db_conn<F, T>(f: F) -> PyResult<T>
where
    F: FnOnce(&Connection) -> Result<T, rusqlite::Error>,
{
    let pool = get_db_pool()?;
    let conn = pool.get_connection()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("DB conn error: {}", e)))?;
    let result = f(&conn)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("DB error: {}", e)));
    pool.return_connection(conn);
    result
}


impl SqlitePool {
    pub fn new(db_path: String, max_connections: usize) -> Result<Self> {
        let mut connections = VecDeque::new();
        
        // create initial connections
        for _ in 0..max_connections {
            let conn = Connection::open(&db_path)?;
            connections.push_back(conn);
        }
        
        Ok(SqlitePool {
            connections: Arc::new(Mutex::new(connections)),
            db_path,
            max_connections,
        })
    }
    
    pub fn get_connection(&self) -> Result<Connection> {
        let mut connections = self.connections.lock().unwrap();
        
        if let Some(conn) = connections.pop_front() {
            Ok(conn)
        } else {
            // if pool is empty, create new connection
            Connection::open(&self.db_path)
        }
    }
    
    pub fn return_connection(&self, conn: Connection) {
        let mut connections = self.connections.lock().unwrap();
        
        // if pool is not full, return connection to pool
        if connections.len() < self.max_connections {
            connections.push_back(conn);
        }
        // otherwise, drop the connection
    }
}

// global connection pool
static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

// initialize connection pool and create table if not exists
pub fn init_db_pool(db_path: String) -> PyResult<()> {
    let pool = SqlitePool::new(db_path, 5)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("Error creating connection pool: {}", e)
        ))?;
    
    // create table if not exists, just for reliability
    {
        let conn = pool.get_connection()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Error getting connection for DDL: {}", e)
            ))?;
        
        conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS files USING fts5(
                file_id UNINDEXED,
                original_name,
                size UNINDEXED,
                upload_date UNINDEXED,
                content,
                tokenize='unicode61'
            )",
            []
        ).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("Error creating FTS5 table: {}", e)
        ))?;
        
        pool.return_connection(conn);
    }
    
    DB_POOL.set(pool)
        .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "Pool already initialized"
        ))?;
    
    Ok(())
}

// get connection pool
fn get_db_pool() -> PyResult<&'static SqlitePool> {
    DB_POOL.get().ok_or_else(|| 
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "Connection pool not initialized. Call init_db_pool() first."
        )
    )
}

// insert processed file into database
pub fn insert_file_data(
    file_id: String,
    original_name: String,
    size: i64,
    upload_date: String,
    content: String,
) -> PyResult<()> {
    with_db_conn(|conn| {
        conn.execute(
            "INSERT INTO files (file_id, original_name, size, upload_date, content) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![file_id, original_name, size, upload_date, content],
        ).map(|_| ())
    })
}

// check if file already exists in database
pub fn file_exists(file_id: String) -> PyResult<bool> {
    with_db_conn(|conn| {
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM files WHERE file_id = ?1")?;
        let count: i64 = stmt.query_row(rusqlite::params![file_id], |row| row.get(0))?;
        Ok(count > 0)
    })
}

 