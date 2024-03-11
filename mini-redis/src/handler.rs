use std::io::{Read, Write};
use std::str::from_utf8;
use std::net::TcpStream;

use crate::commands::{Commands};

pub fn handle_client(mut stream: TcpStream) {
    let commands = Commands::new();

    loop {
        let mut buf = [0; 512];
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let buffer = &buf[..n];
                let str_buffer = from_utf8(buffer).expect("Could not convert the stream to string");
                let response = commands.handle_command(str_buffer);
                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to write to client");
            }
            Err(e) => panic!("{}", e),
        };
    }
}

