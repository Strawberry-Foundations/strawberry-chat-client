use std::io::Write;
use std::net::TcpStream;

pub fn keep_alive(mut stream: TcpStream) {
    loop {
        stblib::utilities::sleep(10);
        stream.write_all(b"").expect("Failed to send Keep Alive");
        // println!("Heartbeat sent");
    }
}