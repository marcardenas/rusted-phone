mod server;

use server::Server;
use simple_logger::SimpleLogger;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "1357";

fn main() {
    SimpleLogger::new().init().unwrap();
    log_panics::init();

    let server: Server = Server::new(String::from(SERVER_IP), String::from(SERVER_PORT));

    server.start().unwrap();
}
