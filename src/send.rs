use crate::global::FRIEND_MAP;
use crate::handle_client::send_function;
use crate::server::USER_MAP;
use crate::server::cleanup_user;
use crate::users::USERSLIST;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub async fn send_loop(
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
                let line_upper = line.trim().to_uppercase();
                let line_split: Vec<&str> = line_upper.split_whitespace().collect();
                let mut user_map = USER_MAP.lock().await;
                let a: Vec<String> = user_map.keys().cloned().collect();
                if line_split.len() == 2 && line_split.contains(&"CHANGE") {
                    if USERSLIST.contains(&line_split[1])
                        && FRIEND_MAP
                            .lock()
                            .await
                            .get(&current_id.parse::<u64>().unwrap())
                            .unwrap()
                            .contains(&line_split[1].parse::<u64>().unwrap())
                        && a.contains(&line_split[1].to_string())
                    {
                        // 全变量修改
                        let writer = user_map.get_mut(&current_id).unwrap();
                        current_friend_id = line_split[1].to_string();
                        writer.write_all("选择好友成功\n".as_bytes()).await.unwrap();
                    } else {
                        let mut map = USER_MAP.lock().await;
                        let writer = map.get_mut(&current_id).unwrap();
                        //给自己发送一个不是好友
                        writer
                            .write_all("你们还不是好友或好友没有上线\n".as_bytes())
                            .await
                            .unwrap();
                    }
                    continue;
                }
                // 给对方发消息
                println!("{:?},{},{}", buf_reader, current_id, current_friend_id);
                send_function(line, current_friend_id.clone(), current_id.clone()).await;
                println!("{:?},{},{}", buf_reader, current_id, current_friend_id);
            }
            Err(_e) => {
                cleanup_user(&current_id).await;
                return (buf_reader, current_id, current_friend_id);
            }
        };
    }
}
