use pyo3::prelude::*;
use std::fs;
use encoding_rs::UTF_8;
use csv::ReaderBuilder;
use quick_xml::events::Event;
use quick_xml::Reader;
use pdf_extract::extract_text;
use crate::utils::{extract_text_from_json, regex_replace_links};

// in general, we dont need a lot of comments here
// function names are self-explanatory

// aux function to parse lines
fn parse_lines<F>(file_path: &str, mut filter: F) -> PyResult<String>
where
    F: FnMut(&str) -> Option<String>,
{
    let content = parse_text_file(file_path)?;
    Ok(content
        .lines()
        .filter_map(|line| filter(line))
        .collect::<Vec<_>>()
        .join("\n"))
}


pub fn parse_text_file(file_path: &str) -> PyResult<String> {
    match fs::read(file_path) {
        Ok(bytes) => {
            let (cow, _, had_errors) = UTF_8.decode(&bytes);
            if had_errors {
                // so, just fix bug if utf-8 fails
                if let Some(encoding) = encoding_rs::Encoding::for_label(b"windows-1252") {
                    let (text, _, _) = encoding.decode(&bytes);
                    Ok(text.into_owned())
                } else {
                    Ok(String::from_utf8_lossy(&bytes).into_owned())
                }
            } else {
                Ok(cow.into_owned())
            }
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(
            format!("Erro ao ler arquivo: {}", e)
        ))
    }
}

pub fn parse_csv_file(file_path: &str) -> PyResult<String> {
    let content = parse_text_file(file_path)?;
    let mut result = String::new();
    let mut reader = ReaderBuilder::new().has_headers(true).flexible(true).from_reader(content.as_bytes());
    if let Ok(headers) = reader.headers() {
        result.push_str(&headers.iter().collect::<Vec<_>>().join(" "));
        result.push('\n');
    }
    for record in reader.records() {
        if let Ok(record) = record {
            result.push_str(&record.iter().collect::<Vec<_>>().join(" "));
            result.push('\n');
        }
    }
    Ok(result)
}


pub fn parse_json_file(file_path: &str) -> PyResult<String> {
    let content = parse_text_file(file_path)?;
    Ok(serde_json::from_str::<serde_json::Value>(&content)
        .map(|json| extract_text_from_json(&json))
        .unwrap_or(content))
}

pub fn parse_markdown_file(file_path: &str) -> PyResult<String> {
    parse_lines(file_path, |line| {
        let cleaned = line
            .trim_start_matches('#')
            .replace("**", "")
            .replace("*", "")
            .replace("`", "");
        let cleaned = regex_replace_links(cleaned.trim());
        if !cleaned.trim().is_empty() {
            Some(cleaned)
        } else { None }
    })
}


pub fn parse_xml_file(file_path: &str) -> PyResult<String> {
    let content = parse_text_file(file_path)?;
    let mut reader = Reader::from_str(&content);
    let mut result = String::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                if let Ok(text) = std::str::from_utf8(&e) {
                    let text = text.trim();
                    if !text.is_empty() {
                        result.push_str(text);
                        result.push(' ');
                    }
                }
            }
            Ok(Event::CData(e)) => {
                if let Ok(text) = std::str::from_utf8(&e) {
                    let text = text.trim();
                    if !text.is_empty() {
                        result.push_str(text);
                        result.push(' ');
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    
    Ok(result)
}

pub fn parse_yaml_file(file_path: &str) -> PyResult<String> {
    parse_lines(file_path, |line| {
        let line = line.split('#').next().unwrap_or("").trim();
        line.split_once(':').and_then(|(_, val)| {
            let val = val.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() && !val.starts_with('[') && !val.starts_with('{') {
                Some(val.to_owned())
            } else { None }
        })
    })
}

pub fn parse_key_value_file(file_path: &str) -> PyResult<String> {
    parse_lines(file_path, |line| {
        let line = line.split('#').next().unwrap_or("");
        let line = line.split(';').next().unwrap_or("").trim();
        line.split_once('=').and_then(|(_, val)| {
            let val = val.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() { Some(val.to_owned()) } else { None }
        })
    })
}

pub fn parse_toml_file(file_path: &str) -> PyResult<String> {
    parse_key_value_file(file_path)
}

pub fn parse_ini_file(file_path: &str) -> PyResult<String> {
    parse_key_value_file(file_path)
}

pub fn parse_sql_file(file_path: &str) -> PyResult<String> {
    let content = parse_text_file(file_path)?;
    let mut result = String::new();
    let mut in_multiline_comment = false;
    
    for line in content.lines() {
        let mut line_str = line.to_string();
        
        if in_multiline_comment {
            if let Some(pos) = line_str.find("*/") {
                line_str = line_str[pos+2..].to_string();
                in_multiline_comment = false;
            } else {
                continue;
            }
        }
        
        while let Some(start) = line_str.find("/*") {
            if let Some(end) = line_str[start+2..].find("*/") {
                line_str.replace_range(start..start+end+4, "");
            } else {
                line_str.truncate(start);
                in_multiline_comment = true;
                break;
            }
        }
        
        if let Some(pos) = line_str.find("--") {
            line_str.truncate(pos);
        }
        
        let trimmed = line_str.trim();
        if !trimmed.is_empty() {
            result.push_str(trimmed);
            result.push('\n');
        }
    }
    
    Ok(result)
}

pub fn parse_pdf_file(file_path: &str) -> PyResult<String> {
    extract_text(file_path)
        .map(|text| text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n"))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Erro ao ler PDF: {}", e)))
}