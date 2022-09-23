use std::net::TcpStream;

use anyhow::{anyhow, Result};
use clap::{App, Arg};

use tokio::net::{TcpListener, ToSocketAddrs};

mod service;

use service::Service;

pub async fn start_server<A: ToSocketAddrs>(addr: A) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    
    loop {
        match listener.accept().await {
            Ok((tcp_stream, addr)) => {
                println!("recv tcp connecting from {}", addr);
                let mut service = Service::new(tcp_stream)?;
                match service.video_service_start().await {
                    Ok(_) =>{},
                    Err(_) =>{println!("err");}
                }
            }
            Err(_) => {}
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = App::new("Flipper")
        .version("1.0")
        .author("Author: RyuAlize <https://github.com/RyuAlize>")
        .about("A toy remote desk.")
        .arg(
            Arg::with_name("socket")
                .default_value("127.0.0.1:8989")
                .takes_value(true)
                .help(r#"The server socket adress"#),
        );
    let args = cli.get_matches();
    let socket = args.value_of("socket").unwrap_or_default();
    start_server(socket).await?;
    Ok(())
}
