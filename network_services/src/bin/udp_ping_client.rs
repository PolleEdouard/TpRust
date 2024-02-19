use std::net::UdpSocket;

use clap::Parser;

#[derive(Parser)]
 struct Options{
    host: String,
    port: u16 ,
}

const PING: &str = "PING";


fn main() -> anyhow::Result<()>{
    let options = Options::parse();
    let server = format!("{}:{}",options.host, options.port);
    let my_socket=UdpSocket::bind("127.0.0.1:0")?;
    my_socket.send_to(PING.as_bytes(),server)?;

    
    Ok(())
}