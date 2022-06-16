use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut buffer = String::new();
    let _ = &stream.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    // Echo it back to client
    let _ = &stream.write_all(&buffer.as_bytes());
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Accept connections and process them serially
    println!("Accepting connections...");
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
