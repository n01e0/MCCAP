// Multi thread TCP Server
// if received "Merry" then send "Christmas"
// if received "Chicken" then send "Chiken"

#![allow(non_snake_case)]
use anyhow::Result;
use clap::Parser;
use log::error;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const CHICKEN: &[u8] = r#"
          ████████        
        ██      ▒▒██      
      ██    ▒▒▒▒▒▒▓▓██    
    ██  ░░▒▒▒▒▒▒▒▒▒▒▓▓██  
    ██░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒██  
  ██  ░░▒▒▒▒▒▒▒▒░░▒▒▒▒▓▓██
  ██░░░░▒▒▒▒▒▒▒▒▒▒░░▒▒▓▓██
  ██░░▒▒▒▒▒▒▒▒▒▒░░░░▒▒▓▓██
    ██▒▒▒▒▒▒▒▒▒▒░░▒▒▒▒██  
  ░░██░░▒▒▒▒▒▒▒▒▒▒▒▒▓▓██  
    ░░██▒▒▒▒▒▒▒▒▒▒▓▓██    
        ██▒▒▒▒▒▒▓▓██      
        ██▒▒▒▒▒▒▓▓██      
          ████████        
          ██░░░░██        
          ██░░░░██        
          ██  ░░██        
        ██    ░░░░██      
      ██    ████░░░░██    
        ████    ████      
"#
.as_bytes();

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

#[derive(Debug)]
struct ServerBuilder {
    port: Option<u16>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder { port: None }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn build(self) -> Result<Server> {
        let port = self.port.unwrap_or(8080);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        Ok(Server { listener })
    }
}

struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn run(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        Self::handle_connection(stream);
                    });
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buffer = String::new();
        loop {
            buffer.clear();
            let _ = reader.read_line(&mut buffer);
            if buffer.trim() == "Merry" {
                let _ = stream.write(b"Christmas\n");
            } else if buffer.trim() == "Chicken" {
                let _ = stream.write(CHICKEN);
            } else {
                let _ = stream.write(b"?\n");
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let mut server = ServerBuilder::new().port(args.port).build()?;

    server.run();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, BufRead, BufReader, BufWriter};
    use std::net::TcpStream;
    use std::sync::{Once, Arc, atomic::{AtomicBool, Ordering}};

    // Initialize logging once for all tests
    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            let _ = env_logger::builder().is_test(true).try_init();
        });
    }

    #[test]
    fn test_server_builder_default_port() {
        initialize();

        // Build server with default port
        let server = ServerBuilder::new().build().unwrap();
        assert_eq!(server.listener.local_addr().unwrap().port(), 8080);
    }

    #[test]
    fn test_server_builder_custom_port() {
        initialize();

        // Build server with custom port
        let server = ServerBuilder::new().port(11111).build().unwrap();
        assert_eq!(server.listener.local_addr().unwrap().port(), 11111);
    }

    #[test]
    fn test_handle_connection_merry() {
        initialize();

        let server_started = Arc::new(AtomicBool::new(false));
        let server_started_clone = server_started.clone();

        // Start server in a separate thread
        thread::spawn(move || {
            let mut server = ServerBuilder::new().port(12345).build().unwrap();
            server_started_clone.store(true, Ordering::SeqCst);
            server.run();
        });

        // Wait for server to start
        while !server_started.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_millis(100));
        }

        // Simulate client connection and send "Merry"
        // This should be done on proper IP-Address and Port,
        // We are just using zero to compile test code.
        let stream = TcpStream::connect("127.0.0.1:12345").unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buffer = String::new();

        // Write to the server
        {
            let mut writer = BufWriter::new(&stream);
            writeln!(writer, "Merry").unwrap();
            writer.flush().unwrap();
        }

        // Read response
        reader.read_line(&mut buffer).unwrap();
        assert_eq!(&buffer, "Christmas\n");
    }

    #[test]
    fn test_handle_connection_chicken() {
        initialize();

        let server_started = Arc::new(AtomicBool::new(false));
        let server_started_clone = server_started.clone();

        // Start server in a separate thread
        thread::spawn(move || {
            let mut server = ServerBuilder::new().port(54321).build().unwrap();
            server_started_clone.store(true, Ordering::SeqCst);
            server.run();
        });

        // Wait for server to start
        while !server_started.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_millis(100));
        }

        // Simulate client connection and send "Chicken"
        // This should be done on proper IP-Address and Port,
        // We are just using zero to compile test code.
        let stream = TcpStream::connect("127.0.0.1:54321").unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buffer = [0u8; CHICKEN.len()];

        // Write to the server
        {
            let mut writer = BufWriter::new(&stream);
            writeln!(writer, "Chicken").unwrap();
            writer.flush().unwrap();
        }

        // Read response
        let len = reader.read(&mut buffer).unwrap();
        assert_eq!(len, CHICKEN.len());
        assert_eq!(&buffer, CHICKEN);
    }
}

