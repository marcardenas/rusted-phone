use log::{error, info, trace};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

const BUFSIZ: usize = 90;

pub struct Server {
    ip: String,
    port: u16,
    echo: bool,
    listener: TcpListener,
}

impl Server {
    /// Create a new Server instance
    pub fn new(ip: String, port: u16, echo: bool) -> Self {
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
            echo: echo,
            listener: listener,
        }
    }

    pub fn start(&self) -> Result<(), std::io::Error> {
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

                    self.handle_client(tcp_stream);
                }
                Err(_e) => {
                    error!("Client connection failed.");
                }
            }
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
        // Creates a new receptor thread and transfer stream ownership
        // to the freshly created thread
    
        let echo: bool = self.echo;

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

                if echo {
                    info!("Echoing message to client: {:?}", message.trim_matches(char::from(0)));
                    match stream.write(&buf) {
                        Ok(_size) => {},
                        Err(e) => {error!("Error echoing message: {:?}", e);}
                    }
                }
            }
            info!("{:?} closed connection.", stream);
        });
    }
}
