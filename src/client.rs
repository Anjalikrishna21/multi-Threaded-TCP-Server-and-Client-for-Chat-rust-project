use std::io::{Read, Write};// to read and write into streams 
use std::net::TcpStream; //to establish a socket connection 

pub fn main() {
    //to establish a tcp connection with server
let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
   //read message from user
  let mut messages = Vec::new();
  loop {
    let mut input = String::new();
    println!("enter the message             (type 'over'if finished):");
    std::io::stdin().read_line(&mut           input).expect("fail to read input       ");
       stream.write(input.as_bytes()).expect("Failed to send message to server");

        // Receive and print response from the server
    let trimmed = input.trim();
        if trimmed == "over" {
            break;
    }
    messages.push(trimmed.to_string());
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to receive data from server");
        let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received from server: {}", received_data);
    
            
    }
      }         