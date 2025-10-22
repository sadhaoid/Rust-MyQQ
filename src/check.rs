use crate::global::FRIEND_MAP;
use crate::server::USER_MAP;
use crate::users::USERSLIST;

pub async fn check_friend_list(current_id: String) -> Vec<u64> {
    let trnasfer_unpacked_friend_map = FRIEND_MAP.lock().await;

    let unpacked_friend_map = trnasfer_unpacked_friend_map
        .get(&current_id.parse::<u64>().unwrap())
        .unwrap();
    let user_map: Vec<String> = USER_MAP.lock().await.keys().cloned().collect();
    let mut logined_friend: Vec<u64> = Vec::new();

    for logined_id in user_map {
        let transferd_logined_id = logined_id.parse::<u64>().unwrap();
        if unpacked_friend_map.contains(&transferd_logined_id) {
            logined_friend.push(transferd_logined_id);
        }
    }
    logined_friend
}
