use std::io::Write;
use std::net::TcpStream;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::Receiver;

use eyre::{bail, Context};
use rustyline::error::ReadlineError;
use stblib::colors::*;

use crate::{SERVER_CONFIG, STRING_LOADER};
use crate::utilities::delete_last_line;


pub fn send(mut stream: TcpStream, rx: Receiver<()>) -> eyre::Result<()> {
    let _ = rx.recv().unwrap();

    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    if SERVER_CONFIG.autologin {
        println!("{GREEN}{BOLD}{}{C_RESET}\n", STRING_LOADER.str("AutologinActive"));

        stream.write_all(SERVER_CONFIG.credentials.username.as_bytes()).context(STRING_LOADER.str("StreamWriteError"))?;

        stblib::utilities::ms_sleep(500);

        stream.write_all(SERVER_CONFIG.credentials.password.as_bytes()).context(STRING_LOADER.str("StreamWriteError"))?;
    }
    else {
        println!("{GREEN}{BOLD}{}{C_RESET}\n", STRING_LOADER.str("AutologinNotActive"));
    }

    loop {
        let input: String = match line_reader.readline("") {
            Ok(i) => i,
            Err(ReadlineError::Interrupted) => {
                stream.write_all(b"/exit")?;
                sleep(Duration::from_millis(300));
                exit(0);
            }
            Err(ReadlineError::Eof) => exit(0),
            Err(e) => bail!(e),
        };

        line_reader.add_history_entry(&input).unwrap();
        stream.write_all(input.as_bytes()).context(STRING_LOADER.str("StreamWriteError"))?;

        delete_last_line();
    }
}
