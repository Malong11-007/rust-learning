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
                    let mut buffer = [0u8; 1024];
                    match tcp_stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received payload {}", String::from_utf8_lossy(&mut buffer));
                        }
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
