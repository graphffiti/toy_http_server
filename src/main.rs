fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Server { addr: addr }
        }

        pub fn run(self) {
            println!("Bind to a TCP Port and listen {}", self.addr);
            println!("Accept the client's connection request");
            println!("Read the client's message");
        }
    }
}
