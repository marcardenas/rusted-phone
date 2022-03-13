use crate::conf::{SERVER_IP, SERVER_PORT};
use structopt::StructOpt;

/// Client/server communication application with broadcast and echo support.
#[derive(StructOpt, Debug)]
#[structopt(name = "Rusted Phone")]
pub struct Args {
    /// Initialize application as server.
    #[structopt(short, long)]
    pub server: bool,
    /// If enabled, server will echo all messages sent to it. Valid only
    /// when server is enabled.
    #[structopt(short, long)]
    pub echo: bool,
    /// If enabled, server will broadcast any received message to all its
    /// connected clients. Valid only when server is enabled.
    #[structopt(short, long)]
    pub broadcast: bool,
    /// IP address to connect to server/host server.
    #[structopt(short, long, default_value=SERVER_IP)]
    pub ip: String,
    /// Port to connect server/host server.
    #[structopt(short, long, default_value=SERVER_PORT)]
    pub port: u16
}