mod screen;
mod service;
mod utils;
mod message;
mod protocol;

use clap::{App, Arg};
use anyhow::{Result, anyhow};

use std::net::{TcpListener, SocketAddr, ToSocketAddrs, TcpStream};

use service::Service;
struct Server {
    listener: TcpListener,
    service: Service,
}

impl Server {
    fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            service: Service::new()?,
        })
    } 

    pub fn run(&mut self) {
        match self.listener.accept() {
            Ok((mut tcp_stream, addr,)) => {
                println!("recv tcp connecting from {}", addr);
                self.stream_handle(&mut tcp_stream);
            },
            Err(_) => {}
        }
    }

    fn stream_handle(&mut self, tcp_stream: &mut TcpStream) -> Result<()>{
        self.service.video_service_start(&mut tcp_stream.try_clone()?)?;
        Ok(())
    }
}
fn main() -> Result<()>{

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
    let mut server = Server::bind(socket)?;
    server.run();
    Ok(())
}
