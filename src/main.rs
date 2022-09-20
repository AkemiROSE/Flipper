mod codec;
mod screen;
mod service;
mod utils;
mod message;
mod protocol;
mod transport;

use clap::{App, Arg};
use anyhow::{Result, anyhow};

use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use service::Service;
use transport::Transport;
struct Server {
    listener: TcpListener,
    service: Service,
}

impl Server {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr).await?,
            service: Service::new()?,
        })
    } 

    pub async fn run(&mut self) -> Result<()> {
        match self.listener.accept().await {
            Ok((mut tcp_stream, addr,)) => {
                println!("recv tcp connecting from {}", addr);
                let transport = Transport::new(tcp_stream);
                self.stream_handle(transport).await?;
            },
            Err(_) => {}
        }
        Ok(())
    }

    async fn stream_handle(&mut self, transport: Transport) -> Result<()>{
        self.service.video_service_start(transport).await?;
        Ok(())
    }
}
#[tokio::main]
async fn main() -> Result<()>{

    let cli = App::new("Flipper")
    .version("1.0")
    .author("Author: RyuAlize <https://github.com/RyuAlize>")
    .about("A toy remote desk.")
    .arg(
        Arg::with_name("socket")    
        .default_value("127.0.0.1:8989")   
            .takes_value(true)
            .help(
                r#"The server socket adress"#,
            )
    );
    let args = cli.get_matches();
    let socket = args.value_of("socket").unwrap_or_default();
    let mut server = Server::bind(socket).await?;
    server.run().await?;
    Ok(())
}
