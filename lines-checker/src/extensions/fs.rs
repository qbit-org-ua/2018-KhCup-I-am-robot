use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn read_utf8_with_limit<P: AsRef<Path>>(path: P, limit: usize) -> io::Result<String> {
    let truncation_message = "... (truncated output)";
    let file_size = fs::metadata(&path)?.len() as usize;
    let (read_buffer_size, read_limit, is_truncated) = if file_size <= limit {
        (file_size, file_size, false)
    } else {
        (limit, limit - truncation_message.len(), true)
    };
    let mut read_buffer = Vec::with_capacity(read_buffer_size);
    read_buffer.resize(read_buffer_size, 0);
    fs::File::open(&path)?.read_exact(&mut read_buffer[..read_limit])?;
    if is_truncated {
        read_buffer[read_limit..].copy_from_slice(truncation_message.as_bytes());
    }
    Ok(String::from_utf8_lossy(&read_buffer).into_owned())
}
