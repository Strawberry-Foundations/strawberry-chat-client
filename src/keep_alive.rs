use std::io::Write;
use std::net::TcpStream;

pub fn keep_alive(mut stream: TcpStream) {
    loop {
        stblib::utilities::sleep(30);
        stream
            .write_all(b"[#<keepalive.event.sent>]")
            .expect("Failed to send Keep Alive");
        // println!("Heartbeat sent");
    }
}
