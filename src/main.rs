use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use crate::resp::{data::Raw2, token::RespTokens};
use crate::service::Service;

mod command;
mod resp;
mod service;

fn main() {
    let store: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let service = Service::new(Arc::clone(&store));

        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut buf = [0; 64];

                    loop {
                        match stream.read(&mut buf) {
                            Ok(_) => {
                                let input_str = String::from_utf8(buf.to_vec()).unwrap();
                                let tokens = RespTokens::try_from(input_str).unwrap();
                                let parsed = Raw2::try_from(tokens).unwrap();
                                let clean: Vec<String> = parsed.try_into().unwrap();
                                let command = command::Command::try_from(clean).unwrap();
                                // let res: String = command.try_into().unwrap();
                                let service_response = service.execute(command);

                                stream.write_all(service_response.as_bytes()).unwrap();
                                stream.flush().unwrap();
                            }
                            Err(e) => {
                                println!("Unknown error: {}", e);
                                break;
                            }
                        }

                        buf.fill(0)
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
