use std::net::TcpListener;
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
    pub fn run(&self) {
        println!("Listening on {}!", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    let a = 5;
                    println!("OK");
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
