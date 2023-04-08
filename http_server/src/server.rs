use crate::http::Request;
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn listen(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);
        for stream in listener.incoming() {
            match stream {
                Ok(mut tcp_stream) => {
                    let mut buffer = Vec::new();
                    // let mut buffer = [0u8; 1024];
                    match tcp_stream.read_to_end(&mut buffer) {
                        Ok(_) => match Request::try_from(&buffer) {
                            Ok(request) => {
                                println!("This is parsed data {}", request.path)
                            }
                            Err(e) => println!("Error {}", e),
                        },
                        Err(err) => {
                            println!("Cannot read the buffer {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }
}
