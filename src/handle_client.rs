use std::io::WriterPanicked;
use std::sync::LazyLock;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

use crate::check::check_friend_list;
use crate::global::FRIEND_MAP;
use crate::server::USER_MAP;
use crate::users::USERSLIST;

pub async fn login_function(line_split: Vec<&str>) -> String {
    if line_split.len() == 2 && line_split.contains(&"LOGIN") && USERSLIST.contains(&line_split[1])
    {
        line_split[1].to_string()
    } else {
        "Please Input Correct Login Info!\n".to_string()
    }
}

// change 的对象有没有上线
// 做一个好友列表
pub async fn change_function(line_split: Vec<&str>, current_id: String) -> String {
    let a: Vec<String> = USER_MAP.lock().await.keys().cloned().collect();
    if line_split.len() == 2
        && line_split.contains(&"CHANGE")
        && USERSLIST.contains(&line_split[1])
        && FRIEND_MAP
            .lock()
            .await
            .get(&current_id.parse::<u64>().unwrap())
            .unwrap()
            .contains(&line_split[1].parse::<u64>().unwrap())
        && a.contains(&line_split[1].to_string())
    {
        line_split[1].to_string()
    } else if line_split.len() == 1 && line_split == ["CHECK"] {
        let mut result = String::new();
        let list = check_friend_list(current_id).await;
        for id in list {
            result = result + &id.to_string() + "\n";
        }
        if result == "" {
            "无好友登录".to_string()
        } else {
            result
        }
    } else if line_split.len() > 2 {
        //"不是朋友或是命令错误或者是好友没有登录, 请使用change命令或使用check命令查看已登录好友列表!\n".to_string()
        "命令的长度过长: 请使用change 好友ID即可\n".to_string()
    } else if line_split.len() == 1 {
        "命令的长度过短: 请使用change 好友ID即可\n".to_string()
    } else if line_split.len() == 2 && !line_split.contains(&"CHANGE") {
        "请使用change关键词选择好友\n".to_string()
    } else if line_split.len() == 2
        && line_split.contains(&"CHANGE")
        && !USERSLIST.contains(&line_split[1])
    {
        "该用户还没有注册\n".to_string()
    } else if line_split.len() == 2
        && line_split.contains(&"CHANGE")
        && USERSLIST.contains(&line_split[1])
        && !FRIEND_MAP
            .lock()
            .await
            .get(&current_id.parse::<u64>().unwrap())
            .unwrap()
            .contains(&line_split[1].parse::<u64>().unwrap())
    {
        "该用户还不是你的好友\n".to_string()
    } else if line_split.len() == 2
        && line_split.contains(&"CHANGE")
        && USERSLIST.contains(&line_split[1])
        && FRIEND_MAP
            .lock()
            .await
            .get(&current_id.parse::<u64>().unwrap())
            .unwrap()
            .contains(&line_split[1].parse::<u64>().unwrap())
        && !a.contains(&line_split[1].to_string())
    {
        "该用户还没有登录\n".to_string()
    } else {
        "这都有我没有想到的漏洞？？？\n".to_string()
    }
}

pub async fn send_function(line: String, mut current_friend_id: String, current_id: String) {
    println!("{:?},{},{}", line, current_id, current_friend_id);
    let mut map = USER_MAP.lock().await;
    println!("2: {:?},{},{}", line, current_id, current_friend_id);
    let writer = map.get_mut(&current_friend_id).unwrap();
    println!("3: {:?},{},{}", line, current_id, current_friend_id);

    writer
        .write_all(format!("From {} MSG:{}\n", current_id, &line).as_bytes())
        .await
        .unwrap();
    writer.flush().await.unwrap();
}

// get_writer
// write_for_id(id, text)
