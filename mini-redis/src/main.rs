mod handler;
mod commands;
mod cache;
mod utils;

use std::{
    net::TcpListener,
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                thread::spawn(|| handler::handle_client(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
