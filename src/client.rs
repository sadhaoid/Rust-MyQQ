use clap::Parser;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin, stdout};
use tokio::net::TcpStream;
use tokio::{self};

#[derive(Parser)]
struct SettingPath {
    #[arg(long)]
    path: Option<String>,
}

pub async fn start_client(addr: &str) -> Result<(), Box<dyn Error>> {
    let stream: TcpStream = TcpStream::connect(&addr).await?;

    let (reader, mut writer) = stream.into_split();

    println!("Please Login first");

    //读控制台，写入流
    let x = tokio::spawn(async move {
        let mut buf_reader = BufReader::new(stdin());

        //
        loop {
            //println!("loop 1 is running");
            let mut input = String::new();
            buf_reader.read_line(&mut input).await.unwrap();
            //println!("read_line function");

            writer.write_all(input.as_bytes()).await.unwrap();
            writer.flush().await.unwrap();
        }
    });

    //读服务端(读stream)，显示在控制台
    let y = tokio::spawn(async move {
        let mut buf_reader = BufReader::new(reader);
        let mut client_output = stdout();
        loop {
            //println!("loop 2 is running");
            let mut input = String::new();
            match buf_reader.read_line(&mut input).await {
                Ok(0) => {
                    client_output
                        .write_all("Server is closed!\n".as_bytes())
                        .await
                        .unwrap();
                    client_output.flush().await.unwrap();
                    return;
                }

                Ok(_) => {
                    client_output.write_all(input.as_bytes()).await.unwrap();
                    client_output.flush().await.unwrap();
                }
                Err(_e) => {
                    return;
                }
            };
        }
    });

    y.await.unwrap();
    x.abort();
    Ok(())
}
