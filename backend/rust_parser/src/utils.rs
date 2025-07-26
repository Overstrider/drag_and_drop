use regex::Regex;

// Extract text recursively from JSON values
pub fn extract_text_from_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => arr.iter().map(extract_text_from_json).collect::<Vec<_>>().join(" "),
        serde_json::Value::Object(obj) => obj.values().map(extract_text_from_json).collect::<Vec<_>>().join(" "),
        _ => String::new()
    }
}

// remove links markdown
pub fn regex_replace_links(text: &str) -> String {
    let re = Regex::new(r"\[([^\[\]]+)\]\([^)]+\)").unwrap();
    re.replace_all(text, "$1").to_string()
}


pub fn is_binary_file(file_path: &str) -> bool {
    use std::fs::File;
    use std::io::Read;

    if let Ok(mut file) = File::open(file_path) {
        let mut buffer = [0; 1024];
        if let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read == 0 { return false; }
            let null_count = buffer[..bytes_read].iter().filter(|&&b| b == 0).count();
            let control_count = buffer[..bytes_read].iter().filter(|&&b| b < 32 && b != 9 && b != 10 && b != 13).count();
            let total = bytes_read as f64;
            let null_ratio = null_count as f64 / total;
            let ctrl_ratio = control_count as f64 / total;
            return null_ratio > 0.3 || ctrl_ratio > 0.3;
        }
    }
    false
}