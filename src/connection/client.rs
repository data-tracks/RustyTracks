use std::net::TcpStream;
use crate::connection::Connection;

pub struct Client{
    host: String,
    port: u16,
}


impl Client {
    pub fn new(host: &str, port: u16) -> Self {
        Client{ host: host.to_string(), port }
    }

    pub fn connect(&self) -> Result<Connection, String> {
        let stream = TcpStream::connect((self.host.clone(), self.port)).unwrap();
        Connection::new(&self.host, self.port, stream)
    }

}

