pub mod config;
use config::SERVER_ADDRESS;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!\r\n";
    stream.write_all(response)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Create listener
    let listener = TcpListener::bind(SERVER_ADDRESS)?;

    // Handle connections
    println!("Listening for incoming connections.");
    for stream in listener.incoming() {
        println!("{:?}", &stream);
        handle_client(stream?)?;
    }

    Ok(())
}
