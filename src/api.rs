use crate::global::FRIEND_MAP;
use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::{delete, post},
};
use serde::{Deserialize, Serialize};
use std::collections::*;
use std::fs::write;
use std::path::Path;
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize)]
struct AddDeleteFriendsJson {
    user_id: u64,
    friend_id: u64,
}

async fn add_friends(
    Json(payload): Json<AddDeleteFriendsJson>,
) -> Result<Json<String>, StatusCode> {
    let path = Path::new("friends.json");
    let mut friends_map = FRIEND_MAP.lock().await;

    if payload.user_id == payload.friend_id {
        Ok(Json("Cannot add yourself!".to_string()))
    } else {
        insert_friendship(&mut friends_map, payload.user_id, payload.friend_id).await;
        insert_friendship(&mut friends_map, payload.friend_id, payload.user_id).await;

        let contents: Vec<u8> = serde_json::to_vec(&*friends_map).unwrap();
        let _ = write(path, contents);
        Ok(Json("Add Successful".to_string()))
    }
}

async fn delete_friends(
    Json(payload): Json<AddDeleteFriendsJson>,
) -> Result<Json<String>, StatusCode> {
    let path = Path::new("friends.json");
    let mut friends_map = FRIEND_MAP.lock().await;
    delete_friendship(&mut friends_map, payload.user_id, payload.friend_id).await;
    delete_friendship(&mut friends_map, payload.friend_id, payload.user_id).await;

    let contents: Vec<u8> = serde_json::to_vec(&*friends_map).unwrap();
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

async fn insert_friendship(
    friends_map: &mut HashMap<u64, HashSet<u64>>,
    user_id: u64,
    friend_id: u64,
) {
    friends_map
        .entry(user_id)
        .or_insert_with(HashSet::new)
        .insert(friend_id);
}

async fn delete_friendship(
    friends_map: &mut HashMap<u64, HashSet<u64>>,
    user_id: u64,
    friend_id: u64,
) {
    friends_map
        .get_mut(&user_id)
        .map(|set| set.remove(&friend_id));
}
