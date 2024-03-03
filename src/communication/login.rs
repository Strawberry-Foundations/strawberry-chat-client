use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use rustyline::error::ReadlineError;
use crate::communication::register::register;
use crate::global::STRING_LOADER;

pub enum PreAuthEvent {
    Login(String, String),
    Register(String, String, String)
}

pub fn login() -> PreAuthEvent {
    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    /* let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let username: String = rprompt::prompt_reply_from_bufread(
        &mut stdin.lock(), &mut stdout.lock(), format!("{GREEN}{BOLD}Username: {C_RESET}")
    ).unwrap().parse().unwrap_or_else(|_| {
        eprintln!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.str("InvalidInput"));
        exit(1);
    });

    let password: String = rprompt::prompt_reply_from_bufread(
        &mut stdin.lock(), &mut stdout.lock(), format!("{GREEN}{BOLD}Password: {C_RESET}")
    ).unwrap().parse().unwrap_or_else(|_| {
        eprintln!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.str("InvalidInput"));
        exit(1);
    }); */
    // format!("{}", "Username: ".green()

    let username: String = match line_reader.readline(STRING_LOADER.load("Username").as_str()) {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };
    
    if username == "register" {
        let (username, password, role_color) = register();
        return PreAuthEvent::Register(username, password, role_color)
    }

    let password: String = match line_reader.readline(STRING_LOADER.load("Password").as_str()) {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    PreAuthEvent::Login(username, password)
}