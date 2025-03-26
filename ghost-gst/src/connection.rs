use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(host: &str, port: u16) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect((host, port))?;
        Ok(Connection { stream })
    }

    pub fn send(&mut self, message: &str) -> Result<(), std::io::Error> {
        self.stream.write_all(message.as_bytes())?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = [0; 1024];
        let bytes_read = self.stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }
}
