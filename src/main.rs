#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        println!("{:?}", stream);
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                loop {
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    let buf = b"+PONG\r\n";
                    stream.write_all(buf).unwrap();
                    stream.flush().unwrap();
                }
                // happens after client disconnects
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
