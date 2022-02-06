use super::http::RawRequest;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,

}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr: addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let mut stream = match listener.accept() {
                Ok((stream, socket_addr)) => stream,
                Err(_) => continue
            };

            let mut buffer = [0; 1024];
            let result = stream.read(&mut buffer);

            match result {
                Ok(n) => {
                    println!("Read {} bytes. Assuming they're UTF8, they read:\n{}", n, String::from_utf8_lossy(&buffer));
                    println!("------------------------------");

                    // could also do this: `RawRequest::try_from(&buffer as &[u8])` but we have a
                    // more concise syntax for creating a slice from another slice.
                    let raw_request = match RawRequest::try_from(&buffer[..]) {
                        Ok(req) => req,
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    };
                }
                Err(e) => {
                    println!("Failed for read stream {}", e);
                    continue;
                }
            }


        }
    }
}