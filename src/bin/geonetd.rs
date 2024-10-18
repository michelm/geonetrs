//! GEONET server daemon

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use core::net::SocketAddr;
use clap::Parser;
use std::io;
use log::{debug, warn}; // info, error, trace

/// Command line arguments for the geonetd server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Device for sending/receiving Ethernet frames
    #[arg(short, long, default_value_t = String::from("eth0"))]
    dev: String,

    /// Listen address for clients
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    listen: String,

    /// Listen port for clients
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Returns the geonet version
    #[arg(short, long)]
    geonet: bool,
}

async fn client_connected(_sock: TcpStream, addr: SocketAddr) {
    debug!("client connected({})", addr);
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.geonet {
        println!("geonet v{}", geonetrs::version());
        return Ok(());
    }

    let listen = format!("{}:{}", args.listen, args.port);
    let listener = TcpListener::bind(listen).await?;

    loop {
        match listener.accept().await {
            Ok((sock, addr)) => {
                client_connected(sock, addr).await;
            },
            Err(e) => warn!("client connect failed: {:?}", e),
        } 
    }
}
