mod conf;
mod server;
mod argparse;

use argparse::Args;
use server::Server;
use structopt::StructOpt;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    log_panics::init();

    let Args {
        server,
        echo,
        broadcast,
        ip,
        port
    } = Args::from_args();

    if server {
        let server: Server = Server::new(ip, port, echo);
        server.start().unwrap();
    }
}
