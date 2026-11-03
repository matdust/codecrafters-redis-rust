#![allow(unused_imports)]
use std::io::{Read, Write};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BUFFER_SIZE: usize = 512;
const ADDR: &str = "127.0.0.1:6379";

#[tokio::main]
async fn main() {
    println!("Server starting on {}", ADDR);

    let listener = tokio::net::TcpListener::bind(ADDR)
        .await
        .expect("Failed to bind to address");

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}

async fn handle_client(mut stream: tokio::net::TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let bytes_read = stream
            .read(&mut buffer)
            .await
            .expect("Failed to read from stream");

        if bytes_read == 0 {
            break;
        }

        let buf = b"+PONG\r\n";
        stream.write_all(buf).await.unwrap();
    }
}
