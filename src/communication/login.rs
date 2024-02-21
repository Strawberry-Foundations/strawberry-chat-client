use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use rustyline::error::ReadlineError;
use crate::global::STRING_LOADER;


pub fn login() -> (String, String) {
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

    let username: String = match line_reader.readline(STRING_LOADER.str("Username").as_str()) {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    let password: String = match line_reader.readline(STRING_LOADER.str("Password").as_str()) {
        Ok(i) => i,
        Err(ReadlineError::Interrupted) => {
            sleep(Duration::from_millis(300));
            exit(0);
        }
        Err(ReadlineError::Eof) => exit(0),
        Err(_) => exit(1),
    };

    (username, password)
}