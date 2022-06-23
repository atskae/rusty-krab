use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

use redisish::{parse, Command, Error};

fn read_command(stream: &mut TcpStream) -> Result<Command, Error> {
    let mut read_buffer = String::new();
    stream.read_to_string(&mut read_buffer).unwrap();
    parse(&read_buffer)
}

fn send_message(stream: &mut TcpStream, message: &str) -> Result<(), io::Error> {
    let _ = stream.write_all(message.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {

    // Create a deque to store messages
    let mut message_queue = VecDeque::new();

    // Accept connections and process them serially
    let port = 7878;
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address)?;
    println!("Accepting connections at address: {}", &address);

    for mut stream in listener.incoming() {
        let command = read_command(stream.as_mut().unwrap());
        match command {
            Ok(Command::Publish(message)) => message_queue.push_back(message),
            Ok(Command::Retrieve) => {
                if let Some(message) = message_queue.pop_front() {
                    let _ = send_message(stream.as_mut().unwrap(), &message);
                }
            }
            Err(err) => {
                println!("An error occurred: {}", err.to_string());
            }
        }
    }

    Ok(())
}
