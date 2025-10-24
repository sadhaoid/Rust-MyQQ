use crate::handle_client::login_function;
use crate::server::USER_MAP;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;

pub async fn login_loop(
    mut buf_reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    mut writer: OwnedWriteHalf,
    mut current_id: String,
) -> (BufReader<tokio::net::tcp::OwnedReadHalf>, String) {
    loop {
        let mut line = String::new();
        let read = buf_reader.read_line(&mut line).await;

        match read {
            Ok(0) => {
                println!("client is closed!");
                return (buf_reader, current_id);
            }

            Ok(_) => {
                let line = line.trim().to_uppercase();
                let line_split: Vec<&str> = line.split_whitespace().collect();

                let login_result = login_function(line_split.clone()).await;

                let mut user_map = USER_MAP.lock().await;

                if line_split.len() == 2 && login_result == line_split[1].to_string() {
                    current_id = login_result.clone();
                    writer
                        .write_all("Login Successful\n".as_bytes())
                        .await
                        .unwrap();
                    user_map.insert(login_result.clone(), writer);
                    println!("user_map: {:?}", user_map);
                    return (buf_reader, current_id);
                } else {
                    writer.write_all(login_result.as_bytes()).await.unwrap();
                    writer.flush().await.unwrap();
                    continue;
                }
            }
            Err(_e) => {
                return (buf_reader, current_id);
            }
        };
    }
}
