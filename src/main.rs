use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

use crate::resp::{data::Raw2, token::RespTokens};

mod command;
mod resp;

#[allow(unused)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut buf = [0; 64];

                    loop {
                        match stream.read(&mut buf) {
                            Ok(n) => {
                                // Parser::parse(&String::from_utf8(buf.to_vec()).unwrap());
                                let input_str = String::from_utf8(buf.to_vec()).unwrap();
                                let tokens = RespTokens::try_from(input_str).unwrap();
                                let parsed = Raw2::try_from(tokens).unwrap();
                                let clean: Vec<String> = Raw2::try_into(parsed).unwrap();
                                let command = command::Command::try_from(clean).unwrap();
                                let res: String = command.try_into().unwrap();

                                stream.write_all(res.as_bytes());
                                stream.flush().unwrap();

                                // RequestParserV1::parse(buf.to_vec());
                                // process(&std::str::from_utf8(&buf.to_vec()).unwrap());
                            }
                            Err(e) => {
                                println!("Unknown error: {}", e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
