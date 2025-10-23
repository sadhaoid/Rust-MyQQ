use crate::global::FRIEND_MAP;
use crate::handle_client::{change_function, login_function, send_function};
use crate::users::USERSLIST;
use std::collections::HashMap;
use std::io::{Result, WriterPanicked};
use std::os::unix::thread;
use std::string;
use std::sync::{Arc, LazyLock};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

pub type Hash = HashMap<String, OwnedWriteHalf>;
pub static USER_MAP: LazyLock<Mutex<HashMap<String, OwnedWriteHalf>>> =
    LazyLock::new(|| Mutex::new(Hash::new()));

pub async fn start_server() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    println!("Server is Listening the Port");

    loop {
        //初始数据的处理
        let (mut stream, addr) = listener.accept().await?;
        let (mut reader, mut writer) = stream.into_split();
        let mut cl = addr.ip().to_string() + ":" + &addr.port().to_string();
        //let x = USER_MAP.clone();

        //读取客户端输入并且写入流的内容
        tokio::spawn(async move {
            let mut buf_reader = BufReader::new(reader);
            let mut current_id = String::new();
            let mut current_friend_id = String::new();
            //first - loop{login_function}
            //second - loop{change_function}
            //third = loop{send function}

            //login
            loop {
                let mut line = String::new();
                let read = buf_reader.read_line(&mut line).await;
                //let mut a1 = x.lock().await;
                //let line = line.trim().to_uppercase();
                let line_split: Vec<&str> = line.split_whitespace().collect();

                let login_result = login_function(line.clone()).await;
                // println!("{}", login_result);

                match read {
                    Ok(0) => {
                        println!("client is closed!");
                        return;
                    }

                    Ok(_) => {
                        if line_split.len() == 2 && login_result == line_split[1].to_string() {
                            current_id = login_result.clone();
                            writer
                                .write_all("Login Successful\n".as_bytes())
                                .await
                                .unwrap();
                            USER_MAP.lock().await.insert(login_result.clone(), writer);
                            println!("user_map: {:?}", USER_MAP.lock().await);
                            // println!("{}", login_result.clone());
                            break;
                        } else {
                            writer.write_all(login_result.as_bytes()).await.unwrap();
                            writer.flush().await.unwrap();
                            continue;
                        }
                    }
                    Err(e) => {
                        return;
                    }
                };
            }

            // println!("{}", current_id);

            //change
            loop {
                let mut line = String::new();
                let read = buf_reader.read_line(&mut line).await;
                //println!("current id{}", login_result.clone());
                //let mut a1 = x.lock().await;
                let line = line.to_uppercase();
                let line_split: Vec<&str> = line.split_whitespace().collect();

                //second - loop{change_function}
                // println!("current_id: {}", login_result.clone());
                let change_result = change_function(line.clone(), current_id.clone()).await;

                match read {
                    Ok(0) => {
                        println!("client is closed!");
                        USER_MAP.lock().await.remove(&current_id);
                        println!("{:?}", USER_MAP.lock().await);
                        return;
                    }

                    Ok(_) => {
                        let mut map = USER_MAP.lock().await;
                        let a = map.get_mut(&current_id).unwrap();
                        if line_split.len() == 2 && change_result == line_split[1].to_string() {
                            current_friend_id = change_result;
                            a.write_all("Ur Friends Can Chat_client\n".as_bytes())
                                .await
                                .unwrap();
                            break;
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
                    Err(e) => {
                        USER_MAP.lock().await.remove(&current_id);
                        return;
                    }
                };
            }

            // //third = loop{send function}
            loop {
                let mut line = String::new();
                let read = buf_reader.read_line(&mut line).await;

                let line_upper = line.to_uppercase();
                let line_split: Vec<&str> = line_upper.split_whitespace().collect();

                //println!("{:?}", line);
                match read {
                    Ok(0) => {
                        println!("client is closed!");
                        USER_MAP.lock().await.remove(&current_id);
                        return;
                    }

                    Ok(_) => {
                        let a: Vec<String> = USER_MAP.lock().await.keys().cloned().collect();
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
                                let mut map = USER_MAP.lock().await;
                                let mut writer = map.get_mut(&current_id).unwrap();
                                current_friend_id = line_split[1].to_string();
                                writer.write_all("选择好友成功\n".as_bytes()).await.unwrap();
                            } else {
                                let mut map = USER_MAP.lock().await;
                                let mut writer = map.get_mut(&current_id).unwrap();
                                //给自己发送一个不是好友
                                writer
                                    .write_all("你们还不是好友或好友没有上线\n".as_bytes())
                                    .await
                                    .unwrap();
                            }
                            continue;
                        }
                        // 给对方发消息
                        // println!("{:?}", line);
                        // println!("{:?}", current_friend_id);
                        send_function(line, current_friend_id.clone(), current_id.clone()).await;
                    }
                    Err(e) => {
                        USER_MAP.lock().await.remove(&current_id);
                        return;
                    }
                };
            }
        });
    }

    Ok(())
}
