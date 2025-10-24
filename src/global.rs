use std::collections::*;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub type Hash = HashMap<u64, HashSet<u64>>;

pub static FRIEND_MAP: LazyLock<Mutex<Hash>> = LazyLock::new(|| Mutex::new(Hash::new()));

pub async fn read_friends() {
    if Path::new("friends.json").exists() {
        let mut friends_string = read_to_string("friends.json").unwrap();
        if friends_string.is_empty() {
            // println!("friends.json can't be empty, please delete it")
            friends_string = "{}".to_string();
        }
        let friend_store: Hash = serde_json::from_str(&friends_string).unwrap();

        let mut map = FRIEND_MAP.lock().await;
        *map = friend_store;
    }
    //println!("good: {:?}", FRIEND_MAP)
}
