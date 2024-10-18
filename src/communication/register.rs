use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use rustyline::error::ReadlineError;
use stblib::colors::{BOLD, C_RESET, GREEN, RED};
use crate::global::STRINGS;

pub fn register() -> (String, String, String) {
    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    println!("{GREEN}{BOLD}{}{C_RESET}", STRINGS.load("RegisterNewUser"));

    let username: String = match line_reader.readline(STRINGS.load("Username").as_str()) {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    let password = password_input();

    let role_color: String = match line_reader.readline(STRINGS.load("RoleColor").as_str()) {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    (username, password, role_color)
}

fn password_input() -> String {
    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    let password: String = match line_reader.readline(STRINGS.load("Password").as_str()) {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    let repeat_password: String = match line_reader.readline(STRINGS.load("RepeatPassword").as_str()) {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    if password != repeat_password {
        eprintln!("{RED}{BOLD}{}{C_RESET}\n", STRINGS.load("PasswordNotMatch"));
        drop(line_reader);
        password_input();
    }

    password
}