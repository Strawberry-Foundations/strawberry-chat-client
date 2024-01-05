use std::io::Read;
use std::net::TcpStream;

#[allow(dead_code)]
pub fn legacy_recv(mut stream: TcpStream) -> eyre::Result<()> {
    let mut buffer = String::new();

    loop {
        match stream.read_to_string(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                println!("{}", buffer);
                buffer.clear();
            }
            Err(e) => {
                eprintln!("err while reading: {}", e);
                break;
            }
        }
    }

    Ok(())
}