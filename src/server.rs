use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    println!("new client connected ");
  let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from socket");
        if bytes_read == 0 {
            // Connection closed
            break;
        }
        
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        let response = match message.trim() {
            "Hello" => "Hello, client!\n",
            "how are you"=>"Fine what about you?",
           "good mrng"=>"good mrng have a nice day \n",
            "bye" => "Goodbye!\n",
            _ => "Unknown command\n",
        };
        
        stream.write_all(response.as_bytes()).expect("Failed to write to socket");
    }
}

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
