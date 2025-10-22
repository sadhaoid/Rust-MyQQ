mod api;
mod check;
mod client;
mod global;
mod handle_client;
mod server;
mod users;
use crate::global::FRIEND_MAP;

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env::args;
use std::path::Path;
use std::*;
use tokio::io::AsyncWriteExt;

use tokio::net::TcpListener;

use tokio::time::{Duration, sleep};

type Hash = HashMap<String, Vec<u64>>;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command_vec: Vec<String> = args().collect();
    //server & API
    if command_vec.contains(&"server".to_string()) {
        global::read_friends().await;
        tokio::spawn(async move {
            api::use_api().await.unwrap();
        });

        server::start_server().await?;
    } else {
        client::start_client().await.unwrap();
    }
    //tokio::signal::ctrl_c().await; //todo
    //1, login 10001
    //2, change 10002
    //3, send hello

    Ok(())
}
