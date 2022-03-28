fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    use std::io::Read;
    use std::net::TcpListener;
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Server { addr: addr }
        }

        pub fn run(self) {
            println!("Bind to a TCP Port and listen {}", self.addr);
            let listener = TcpListener::bind(self.addr).unwrap();

            loop {
                println!("Accept the client's connection request");
                match listener.accept() {
                    Ok((mut stream, addr)) => {
                        println!("Client Connected. {}", addr);
                        let mut buff = [0; 256];
                        println!("Read the client's message");
                        match stream.read(&mut buff) {
                            Ok(_) => {
                                println!("Message received: {}", String::from_utf8_lossy(&buff));
                            }
                            Err(_) => {}
                        }
                    }
                    Err(e) => {
                        println!("Error encountered {}", e);
                    }
                }
            }
        }
    }
}
