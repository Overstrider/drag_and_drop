use pyo3::prelude::*;
use std::path::Path;

mod parsers;
mod utils;
mod db;

use parsers::*;
use db::{init_db_pool, insert_file_data, file_exists};
use crate::utils::is_binary_file;

/// extract text from file
#[pyfunction]
fn parse_file(file_path: String) -> PyResult<String> {
    let path = Path::new(&file_path);
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "txt" | "log" | "text" => parse_text_file(&file_path),
        "csv" => parse_csv_file(&file_path),
        "json" => parse_json_file(&file_path),
        "md" | "markdown" => parse_markdown_file(&file_path),
        "xml" | "html" | "htm" => parse_xml_file(&file_path),
        "yaml" | "yml" => parse_yaml_file(&file_path),
        "toml" => parse_toml_file(&file_path),
        "ini" | "cfg" | "conf" => parse_ini_file(&file_path),
        "sql" => parse_sql_file(&file_path),
        "pdf" => parse_pdf_file(&file_path),
        _ => {
            // any non-binary file will be treated as text
            if is_binary_file(&file_path) {
                let filename = path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("binary_file");
                Ok(format!("[Binary file: {}]", filename))
            } else {
                parse_text_file(&file_path)
            }
        }
    }
}

// initialize database connection pool
#[pyfunction]
fn init_database(db_path: String) -> PyResult<()> {
    init_db_pool(db_path)
}

// parse file and insert into database
#[pyfunction]
fn parse_and_store_file(
    file_path: String,
    file_id: String,
    original_name: String,
    size: i64,
    upload_date: String,
) -> PyResult<()> {
    // check if file already exists
    if file_exists(file_id.clone())? {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("File {} already exists in the database", file_id)
        ));
    }
    
    // parse file
    let content = parse_file(file_path)?;
    
    // insert into database
    insert_file_data(file_id, original_name, size, upload_date, content)?;
    
    Ok(())
}

// check if file exists in database
#[pyfunction]
fn check_file_exists(file_id: String) -> PyResult<bool> {
    file_exists(file_id)
}



// Python module
#[pymodule]
fn rust_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_file, m)?)?;
    m.add_function(wrap_pyfunction!(init_database, m)?)?;
    m.add_function(wrap_pyfunction!(parse_and_store_file, m)?)?;
    m.add_function(wrap_pyfunction!(check_file_exists, m)?)?;
    Ok(())
} 