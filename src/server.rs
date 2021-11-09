use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::{TryFrom};

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
        println!("Server is listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let result = listener.accept();

            match result {
                Ok((mut stream, addr)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(bytesRead) => {
                            println!("Received a requestL {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]){
                                Ok(req) => {},
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },
                Err(err) => println!("Failed to extablish connection: {}", err)
            }
        }
    }
}