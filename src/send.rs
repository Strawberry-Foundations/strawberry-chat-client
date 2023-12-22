use std::io::Write;
use std::net::TcpStream;
use crate::config::{Config, ServerValues};

pub(crate) fn send(mut stream: TcpStream, _config: Config, server_config: ServerValues) -> ! {
    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    if server_config.autologin == true {
        stream.write(server_config.credentials.username.as_bytes()).expect("Error writing stream");

        stblib::utilities::ms_sleep(500);

        stream.write(server_config.credentials.password.as_bytes()).expect("Error writing stream");
    }

    loop {
        let input: String = match line_reader.readline("") {
            Ok(inp) => inp,
            Err(_) => std::process::exit(1), 
        };

        line_reader.add_history_entry(&input).unwrap();
        stream.write(input.as_bytes()).expect("Error writing stream");
    }
}