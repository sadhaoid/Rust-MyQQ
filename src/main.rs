mod api;
mod change;
mod check;
mod client;
mod global;
mod handle_client;
mod login;
mod send;
mod server;
mod users;
use crate::client::start_client;
use crate::global::read_friends;
use crate::server::start_server;
use std::env::args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command_vec: Vec<String> = args().collect();
    //server & API
    if command_vec.contains(&"server".to_string()) {
        read_friends().await;
        tokio::spawn(async move {
            api::use_api().await.unwrap();
        });

        start_server().await?;
    } else {
        start_client().await.unwrap();
    }

    Ok(())
}
