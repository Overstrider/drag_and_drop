from datetime import datetime, UTC

def process_file_from_temp(temp_path: str, file_id: str, original_name: str, size: int, upload_date: str = None):
    # this is not a good option
    # but, at start, it will be ok
    # TODO: generate a presigned url in python, and send it to rust download and process
    try:
        import rust_parser
    except ImportError:
        raise RuntimeError("Rust parser not available")
    
    if upload_date is None:
        upload_date = datetime.now(UTC).isoformat()
    
    try:
        print(f"Processing file: {original_name} ({file_id})")
        rust_parser.parse_and_store_file(temp_path, file_id, original_name, size, upload_date)
        print(f"File processed and stored: {original_name}")
    except Exception as e:
        print(f"Error processing file with Rust: {e}")
        import traceback
        traceback.print_exc()
        raise RuntimeError(f"Rust processing failed: {str(e)}") 