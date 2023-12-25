use std::io::Write;
use std::net::TcpStream;

use stblib::strings::Strings;
use stblib::colors::*;

use crate::config::{get_config, Config, ServerValues};
use crate::utilities::delete_last_line;

pub fn send(mut stream: TcpStream, config: Config, server_config: ServerValues) -> ! {
    let string_loader = Strings::new(config.language.as_str(), get_config().as_str());

    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    if server_config.autologin {
        stream.write_all(server_config.credentials.username.as_bytes()).expect(format!("{RED}{BOLD}{}{C_RESET}", string_loader.str("StreamWriteError")).as_str());

        stblib::utilities::ms_sleep(500);

        stream.write_all(server_config.credentials.password.as_bytes()).expect(format!("{RED}{BOLD}{}{C_RESET}", string_loader.str("StreamWriteError")).as_str());
    }

    loop {
        let input: String = line_reader.readline("").map_or_else(|_| {
                eprintln!("{BOLD}{YELLOW}{}{C_RESET}", string_loader.str("Aborted"));
                std::process::exit(1)
            }, |inp| inp);

        line_reader.add_history_entry(&input).unwrap();
        stream.write_all(input.as_bytes()).expect(format!("{RED}{BOLD}{}{C_RESET}", string_loader.str("StreamWriteError")).as_str());

        delete_last_line();
    }
}
