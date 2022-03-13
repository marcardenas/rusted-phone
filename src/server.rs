use log::{error, info, trace};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

const BUFSIZ: usize = 90;

pub struct Server {
    ip: String,
    port: String,
    listener: TcpListener,
}

fn handle_client(mut stream: TcpStream) {
    // Creates a new receptor thread and transfer stream ownership
    // to the freshly created thread

    thread::spawn(move || {
        let mut buf: [u8; BUFSIZ] = [0; BUFSIZ];

        while let Ok(size) = stream.read(&mut buf) {

            if size == 0 {
                // Client closed connection.
                break;
            }

            // Transform buffer with received message to a String. If there
            // is an error, log the error and wait for another message.
            let message: String = {
                match str::from_utf8(&buf) {
                    Ok(converted) => String::from(converted),
                    Err(e) => {
                        error!("Error: {:?}", e);
                        continue;
                    }
                }
            };

            info!(
                "Received message from {:?}: {}. {}",
                stream.peer_addr().unwrap(),
                message,
                size
            );
        }

        info!("{:?} closed connection.", stream);
    });
}

impl Server {
    /// Create a new Server instance
    pub fn new(ip: String, port: String) -> Self {
        trace!("Creating TCP/IP server...");

        let address = format!("{}:{}", ip, port);
        let listener = TcpListener::bind(address).expect("Unable to find a listener.");

        trace!(
            "Listener created on port {}. Server can be targeted using address {}.",
            port,
            ip
        );
        Server {
            ip: ip,
            port: port,
            listener: listener,
        }
    }

    pub fn start(self) -> Result<(), std::io::Error> {
        // Server succesfuly opened
        info!(
            "Server opened on port {}. Waiting for incoming connections...",
            self.port
        );

        for stream in self.listener.incoming() {
            match stream {
                Ok(tcp_stream) => {
                    let client_addr = tcp_stream.peer_addr().unwrap();
                    info!(
                        "Client connected from address {:?} and port {:?}...",
                        client_addr.ip(),
                        client_addr.port()
                    );

                    handle_client(tcp_stream);
                }
                Err(_e) => {
                    error!("Client connection failed.");
                }
            }
        }
        Ok(())
    }
}
