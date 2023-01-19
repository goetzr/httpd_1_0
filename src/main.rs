use std::net::{TcpListener, TcpStream, Ipv4Addr, SocketAddrV4};

use clap::Parser;

mod net;
mod message;

#[derive(Parser)]
#[command(about)]
struct CliArgs {
    /// The HTTP server port.
    #[arg(verbatim_doc_comment, default_value_t = 8080)]
    port: u16,
}

fn main() {
    let args = CliArgs::parse();
    let server_sock = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, args.port))?;

    loop {
        let client = 
    }

    println!("Hello, world!");
}


