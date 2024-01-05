use std::io::Write;
use std::net::TcpStream;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use rustyline::error::ReadlineError;

pub fn login(mut stream: &TcpStream) -> (String, String) {
    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    let username: String = match line_reader.readline("Username: ") {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            stream.write_all(b"exit").unwrap_or_else(|_| eprintln!("Could not write to stream"));
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    let password: String = match line_reader.readline("Password: ") {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            stream.write_all(b"exit").unwrap_or_else(|_| eprintln!("Could not write to stream"));
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    (username, password)
}