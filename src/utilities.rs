use std::io::{self, Write};
use std::path::{Path, PathBuf};
use serde_json::Value;
use stblib::colors::{BOLD, C_RESET, RED};

pub fn delete_last_line() {
    print!("\x1b[1A");
    print!("\x1b[2K");
    io::stdout().flush().unwrap();
}

pub fn make_absolute_path(input_path: &str) -> PathBuf {
    let path = Path::new(input_path);

    if path.is_absolute() {
        PathBuf::from(path)
    } else {
        let mut absolute_path = std::env::current_dir().unwrap_or_else(|_| {
            eprintln!("{RED}{BOLD}Couldn't get current directory{C_RESET}");
            std::process::exit(1);
        });
        absolute_path.push(path);
        absolute_path
    }
}

pub fn serializer(text: &str) -> Result<Value, serde_json::Error> {
    let serializer = serde_json::from_str(text)?;
    Ok(serializer)
}