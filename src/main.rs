use std::io::{self};
use std::net::TcpStream;
use std::thread;

mod recv;
mod send;

mod config;
mod formatter;
mod constants;

fn main() -> io::Result<()> {
    let config = config::Config::new();
    let server_config = config::Config::server_id(0);

    let send_config = config::Config::new();
    let send_server_config = config::Config::server_id(0);

    let host = format!("{}:{}", server_config.address, server_config.port);

    let stream = TcpStream::connect(host).expect("Error opening stream");

    let send_stream = stream.try_clone().unwrap();

    let string_loader = stblib::strings::Strings::new(config.language.as_str(), "C:\\Users\\Julian\\Desktop\\stbchat-rust\\target\\debug\\lang.yml");

    let handler = thread::spawn(|| recv::recv(stream, config, server_config, string_loader));
    thread::spawn(|| send::send(send_stream, send_config, send_server_config));

    handler.join().unwrap();

    Ok(())
}