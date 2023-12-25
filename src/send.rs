use std::io::Write;
use std::net::TcpStream;

use stblib::strings::Strings;
use stblib::colors::*;

use crate::config::{get_config, Config, ServerValues};
use crate::utilities::delete_last_line;

pub(crate) fn send(mut stream: TcpStream, config: Config, server_config: ServerValues) -> ! {
    let string_loader = Strings::new(config.language.as_str(), get_config().as_str());

    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    if server_config.autologin == true {
        stream
            .write(server_config.credentials.username.as_bytes())
            .expect("Error writing stream");

        stblib::utilities::ms_sleep(500);

        stream
            .write(server_config.credentials.password.as_bytes())
            .expect("Error writing stream");
    }

    loop {
        let input: String = match line_reader.readline("") {
            Ok(inp) => inp,
            Err(_) => {
                eprintln!(
                    "{}",
                    format!("{BOLD}{YELLOW}{}{C_RESET}", string_loader.str("Aborted"))
                );
                std::process::exit(1)
            }
        };

        line_reader.add_history_entry(&input).unwrap();
        stream
            .write(input.as_bytes())
            .expect("Error writing stream");

        delete_last_line();
    }
}
