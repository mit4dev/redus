use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("connection");
                let mut buf = [0; 64];

                match stream.read(&mut buf) {
                    Ok(n) => {
                        println!("Read bytes: {}", n);
                        stream.write("+PONG\r\n".as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                    Err(e) => {
                        println!("Unknown error: {}", e);
                        // break;
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
