use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

pub const MSG_SIZE: usize = 128;
pub struct TcpHandler {
    stream: Option<TcpStream>,
    pub server: Option<bool>,
}

impl TcpHandler {
    pub fn new() -> Self {
        Self {
            stream: None,
            server: None,
        }
    }

    pub fn run_server(&mut self, bind_addr: &str) -> io::Result<()> {
        let listener = TcpListener::bind(bind_addr)?;
        let (stream, sock_addr) = listener.accept()?;
        println!("Client connected from {sock_addr}");
        stream
            .set_nonblocking(true)
            .expect("set_nonblocking call failed");
        self.stream = Some(stream);
        self.server = Some(true);
        Ok(())
    }

    pub fn run_client(&mut self, remote_addr: &str) -> io::Result<()> {
        let stream = TcpStream::connect(remote_addr)?;
        println!("Connected to server at {remote_addr}");
        stream
            .set_nonblocking(true)
            .expect("set_nonblocking call failed");
        self.stream = Some(stream);
        self.server = Some(false);

        Ok(())
    }

    pub fn _write(&mut self, msg: &str) -> io::Result<()> {
        if let Some(stream) = &mut self.stream {
            stream.write_all(msg.as_bytes())?;
            stream.flush()?;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "barabingbaraboom",
            ))
        }
    }

    pub fn _read(&mut self) -> io::Result<String> {
        if let Some(stream) = &mut self.stream {
            let mut buf = [0u8; MSG_SIZE];
            stream.read_exact(&mut buf)?;

            let msg = String::from_utf8_lossy(&buf).to_string();
            Ok(msg)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "barabingbaraboom",
            ))
        }
    }
}
