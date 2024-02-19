use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

use clap::Parser;

#[derive(Parser)]
 struct Options{
    host: String,
    port: u16 ,
}
const BUFFER_SIZE: usize = 2048;

#[tokio::main]
async fn main()-> anyhow::Result<()>{
    let options = Options::parse();
    let serveur = TcpListener::bind(format!("{}:{}",options.host,options.port)).await?;
    loop {
        let (mut stream, _) = serveur.accept().await?;
        tokio::spawn(async move {
            let (mut reader, mut writer)= stream.split();
            let mut buf = [0;BUFFER_SIZE];
            let size = reader.read(&mut buf).await?;
            let received_message = std::str::from_utf8(&buf [..size])?;
            writer.write_all(received_message.as_bytes()).await?;
            writer.flush().await?;

            Ok::<(), anyhow::Error>(())
        });
    }
}