use std::io::WriterPanicked;
use std::sync::LazyLock;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

use crate::check::check_friend_list;
use crate::global::FRIEND_MAP;
use crate::server::USER_MAP;
use crate::users::USERSLIST;

//pub static CURRENT_ID: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
//pub static CURRENT_FRIEND_ID: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

/**
 *
 */
pub async fn login_function(line: String) -> String {
    let line = line.trim().to_uppercase();
    let line_split: Vec<&str> = line.split_whitespace().collect();
    if line_split.len() == 2 && line_split.contains(&"LOGIN") && USERSLIST.contains(&line_split[1])
    {
        line_split[1].to_string()
    } else {
        "Please Input Correct Login Info!\n".to_string()
    }
}

// change 的对象有没有上线
// 做一个好友列表
pub async fn change_function(line: String, current_id: String) -> String {
    let line = line.trim().to_uppercase();
    let line_split: Vec<&str> = line.split_whitespace().collect();
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
        //*CURRENT_FRIEND_ID.lock().await = line_split[1].to_string();
        //"Ur Friends Can Chat_client\n".to_string()
        //println!("second if done");
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
    // println!("clinet: {:?}", line);
    // println!("{:?}", current_friend_id);
    let mut map = USER_MAP.lock().await;
    //println!("{:?}", current_friend_id);
    //需要判定change的人是否是好友,
    //还要判断好友是否在线,
    //客户端退出的时候, 用户也得从map里面清除掉
    let writer = map.get_mut(&current_friend_id).unwrap();
    //println!("{:?}", writer);

    writer
        .write_all(format!("From {} MSG:{}\n", current_id, &line).as_bytes())
        .await
        .unwrap();
    writer.flush().await.unwrap();
}

// get_writer
// write_for_id(id, text)
