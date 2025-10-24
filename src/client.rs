use std::error::Error;
use std::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, stdin};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};
use tokio::{self, stream};

use crate::handle_client::{change_function, login_function, send_function};

pub async fn start_client() -> Result<(), Box<dyn Error>> {
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8080").await?;

    let (mut reader, mut writer) = stream.into_split();

    println!("Please Login first");

    //读控制台，写入流
    let x = tokio::spawn(async move {
        let mut buf_reader = BufReader::new(stdin());
        //
        loop {
            //println!("loop 1 is running");
            let mut input = String::new();
            buf_reader.read_line(&mut input).await;
            //println!("read_line function");

            writer.write_all(input.as_bytes()).await;
            writer.flush().await;
        }
    });

    //读服务端(读stream)，显示在控制台
    let y = tokio::spawn(async move {
        let mut buf_reader = BufReader::new(reader);
        loop {
            //println!("loop 2 is running");
            let mut input = String::new();
            match buf_reader.read_line(&mut input).await {
                Ok(0) => {
                    println!("Server is closed!");
                    return;
                }

                Ok(_) => {
                    println!("{}", input);
                }
                Err(e) => {
                    return;
                }
            };
        }
    });

    y.await.unwrap();
    x.abort();
    //println!("Server is closed!\n");
    Ok(())
}
