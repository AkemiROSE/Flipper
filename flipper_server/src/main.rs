mod service;

use std::net::TcpStream;

use anyhow::{anyhow, Result};
use clap::{App, Arg};

use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::mpsc::{channel, Receiver},
};
use tracing::{debug, error, info, instrument};


use service::Service;

pub async fn start_server<A: ToSocketAddrs>(addr: A, shutdown_rx: Receiver<()>) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    
    match listener.accept().await {
        Ok((tcp_stream, addr)) => {
            println!("recv tcp connecting from {}", addr);
            let mut service = Service::new(tcp_stream,shutdown_rx)?;
            match service.video_service_start().await {
                Ok(_) =>{},
                Err(_) =>{return Err(anyhow!("connect interrupted"));}
            }
        }
        Err(e) => {return Err(anyhow!("couldn't get client: {:?}", e));}
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
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
    let (shutdown_tx, shutdown_rx) = channel(1);
    let sig = tokio::signal::ctrl_c();

    tokio::select! {
        res = start_server(socket, shutdown_rx) => {
            if let Err(err) = res {
                error!(cause = %err, "failed to accept");
            }
        } 
        _ = sig => {
            shutdown_tx.send(()).await;
            // The shutdown signal has been received.
            info!("shutting down");
        }
    };
    
  
}
