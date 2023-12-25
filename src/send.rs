use std::io::Write;
use std::net::TcpStream;

use eyre::{bail, Context};
use rustyline::error::ReadlineError;
use stblib::colors::*;

use crate::{SERVER_CONFIG, STRING_LOADER};
use crate::utilities::delete_last_line;

pub fn send(mut stream: TcpStream) -> eyre::Result<()> {
    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    if SERVER_CONFIG.autologin {
        stream.write_all(SERVER_CONFIG.credentials.username.as_bytes()).context(STRING_LOADER.str("StreamWriteError"))?;

        stblib::utilities::ms_sleep(500);

        stream.write_all(SERVER_CONFIG.credentials.password.as_bytes()).context(STRING_LOADER.str("StreamWriteError"))?;
    }

    loop {
        let input: String = match line_reader.readline("") {
            Ok(i) => i,
            Err(ReadlineError::Interrupted) => {
                stream.write_all(b"/exit")?;
                bail!("Interrupted")
            }
            Err(e) => bail!(e),
        };

        line_reader.add_history_entry(&input).unwrap();
        stream.write_all(input.as_bytes()).context(format!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.str("StreamWriteError")))?;

        delete_last_line();
    }
}
