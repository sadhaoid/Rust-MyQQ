use crate::handle_client::change_function;
use crate::server::USER_MAP;
use crate::server::cleanup_user;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub async fn change_loop(
    mut buf_reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    current_id: String,
    mut current_friend_id: String,
) -> (BufReader<tokio::net::tcp::OwnedReadHalf>, String, String) {
    loop {
        let mut line = String::new();
        let read = buf_reader.read_line(&mut line).await;

        match read {
            Ok(0) => {
                cleanup_user(&current_id).await;
                return (buf_reader, current_id, current_friend_id);
            }

            Ok(_) => {
                let line = line.trim().to_uppercase();
                let line_split: Vec<&str> = line.split_whitespace().collect();

                let change_result = change_function(line_split.clone(), current_id.clone()).await;

                let mut user_map = USER_MAP.lock().await;

                let a = user_map.get_mut(&current_id).unwrap();
                if line_split.len() == 2 && change_result == line_split[1].to_string() {
                    current_friend_id = change_result;
                    a.write_all("Ur Friends Can Chat_client\n".as_bytes())
                        .await
                        .unwrap();
                    return (buf_reader, current_id, current_friend_id);
                } else if line_split.len() == 1 && line_split == ["CHECK"] {
                    a.write_all(format!("{}{}", change_result, "\n").as_bytes())
                        .await
                        .unwrap();
                    continue;
                } else {
                    a.write_all(change_result.as_bytes()).await.unwrap();
                    a.flush().await.unwrap();
                    continue;
                }
            }
            Err(_e) => {
                cleanup_user(&current_id).await;
                return (buf_reader, current_id, current_friend_id);
            }
        };
    }
}
