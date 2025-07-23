use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use crate::connection::Connection;

pub struct Client{
    host: String,
    port: u16,
}


impl Client {
    pub fn new(host: &str, port: u16) -> Self {
        Client{ host: host.to_string(), port}
    }

    pub fn connect_timeout(&self, duration: Duration) -> Result<Connection, String> {
        let addr = SocketAddr::from_str(&format!("{}:{}", self.host.clone(), self.port)).map_err(|_| "Invalid address".to_string())?;
        let stream = TcpStream::connect_timeout(&addr, duration).unwrap();
        Connection::new(&self.host, self.port, stream)
    }

    pub fn connect(&self) -> Result<Connection, String> {
        let stream = TcpStream::connect((self.host.clone(), self.port)).unwrap();
        Connection::new(&self.host, self.port, stream)
    }

}

