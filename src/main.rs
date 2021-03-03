// Updated example from http://rosettacode.org/wiki/Hello_world/Web_server#Rust
// to work with Rust 1.0 beta

use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0x0; 4096];
    match stream.read(&mut buf) {
        Ok(_) => println!("Reading request"),
        Err(e) => println!("Error while reading request: {}", e),
    }

    let client_ip = match stream.peer_addr() {
        Ok(a) => format!("{}", a),
        Err(e) => format!("{}", e),
    };

    let req_ptr = String::from_utf8_lossy(&buf);
    let mut req = format!("{}\r\n{}\r\n", client_ip, String::from(req_ptr));

    let end_index = req.as_bytes().iter().position(|&c| c == 0x0).unwrap();
    req.truncate(end_index);

    // save req to file
    let file_name = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    match File::create(format!("{}.http", file_name)) {
        Ok(mut f) => {
            match f.write(req.as_bytes()) {
                Err(e) => println!("{}", e),
                _ => {}
            };
        }
        Err(_) => panic!("Collision {}.http already exists", file_name),
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html>{}</html>\r\n",
        req
    );

    println!("{}", response);

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent\r\n"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening for connections on port {}", 80);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
