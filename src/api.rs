use crate::global::FRIEND_MAP;
use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::{delete, get, post},
    serve::Listener,
};
use serde::{Deserialize, Serialize};
use std::fs::write;
use std::path::Path;
use std::{collections::*, env::consts};
use std::{fs::read_to_string, sync::LazyLock};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

type Hash = HashMap<String, HashSet<u64>>;

#[derive(Debug, Serialize, Deserialize)]
struct AddDeleteFriendsJson {
    user_id: u64,
    friend_id: u64,
}

async fn add_friends(
    Json(payload): Json<AddDeleteFriendsJson>,
) -> Result<Json<String>, StatusCode> {
    let path = Path::new("friends.json");
    // let mut payload_string = read_to_string("friends.json").unwrap();
    // let payload: Hash = serde_json::from_str(&payload_string).unwrap();
    //println!("{}", payload);
    let mut friends_map = FRIEND_MAP.lock().await;

    // println!("{:?}", b);
    if payload.user_id == payload.friend_id {
        Ok(Json("Cannot add yourself!".to_string()))
    } else {
        friends_map
            .entry(payload.user_id)
            .or_insert_with(HashSet::new)
            .insert(payload.friend_id);
        // println!("after op memo{:?}", friends_map);
        // println!("after op global{:?}", FRIEND_MAP);
        friends_map
            .entry(payload.friend_id)
            .or_insert_with(HashSet::new)
            .insert(payload.user_id);
        let contents: Vec<u8> = serde_json::to_vec(&*friends_map).unwrap(); // or_unwrap() // else_unwrap();
        let _ = write(path, contents); //todo 
        Ok(Json("Add Successful".to_string()))
    }

    // let friends_map_after = FRIEND_MAP.lock().await;
    // println!("Re-read from FRIEND_MAP: {:?}", friends_map_after);
}

async fn delete_friends(
    Json(payload): Json<AddDeleteFriendsJson>,
) -> Result<Json<String>, StatusCode> {
    let path = Path::new("friends.json");
    let mut friends_map = FRIEND_MAP.lock().await;
    // friends_map
    //     .get_mut(&payload.user_id) // 这里不用unwrap，看是否有其他方案api
    //     .map_or(Ok(Json("Delete Successful".to_string())), |friends| {
    //         if friends.remove(&payload.friend_id) {
    //             Ok(Json("Delete Successful".to_string()))
    //         } else {
    //             Ok(Json("Friend not in list".to_string()))
    //         }
    //     });

    friends_map
        .get_mut(&payload.user_id)
        .map(|set| set.remove(&payload.friend_id));

    // println!("after op memo{:?}", friends_map);
    // println!("after op global{:?}", FRIEND_MAP);
    // friends_map
    //     .get_mut(&payload.user_id)
    //     .unwrap()
    //     .remove(&payload.user_id);

    friends_map
        .get_mut(&payload.friend_id)
        .map(|set| set.remove(&payload.user_id));
    let contents: Vec<u8> = serde_json::to_vec(&*friends_map).unwrap(); // or_unwrap() // else_unwrap(); // todo
    let _ = write(path, contents);
    Ok(Json("Delete Successful".to_string()))
}

pub async fn use_api() -> Result<(), Box<dyn std::error::Error>> {
    let listener_api_port = TcpListener::bind("0.0.0.0:8041").await?;
    println!("API is running");
    let router = Router::new()
        .route("/friends", post(add_friends))
        .route("/friends", delete(delete_friends));
    axum::serve(listener_api_port, router).await?;
    Ok(())
}
