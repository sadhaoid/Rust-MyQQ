use crate::change::change_loop;
use crate::login::login_loop;
use crate::send::send_loop;
use std::collections::HashMap;
use std::io::Result;
use std::sync::LazyLock;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

pub type Hash = HashMap<String, OwnedWriteHalf>;
pub static USER_MAP: LazyLock<Mutex<HashMap<String, OwnedWriteHalf>>> =
    LazyLock::new(|| Mutex::new(Hash::new()));

pub async fn start_server() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server is Listening the Port");

    loop {
        //初始数据的处理
        let (stream, _addr) = listener.accept().await?;
        let (reader, mut writer) = stream.into_split();

        //读取客户端输入并且写入流的内容
        tokio::spawn(async move {
            let mut buf_reader: BufReader<tokio::net::tcp::OwnedReadHalf> = BufReader::new(reader);
            let mut current_id = String::new();
            let mut current_friend_id = String::new();

            //login
            let (buf_reader, current_id) = login_loop(buf_reader, writer, current_id).await;

            //change
            let (buf_reader, current_id, current_friend_id) =
                change_loop(buf_reader, current_id, current_friend_id).await;

            //send
            send_loop(buf_reader, current_id, current_friend_id).await;
        });
    }

    Ok(())
}

pub async fn cleanup_user(user_id: &str) {
    USER_MAP.lock().await.remove(user_id);
    println!("User {} disconnected", user_id);
}
