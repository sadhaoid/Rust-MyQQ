mod api;
mod check;
mod client;
mod global;
mod handle_client;
mod server;
mod users;
use crate::client::start_client;
use crate::global::read_friends;
use crate::server::start_server;
use clap::Parser;
use config::Config;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    server: bool,

    #[arg(short, long, default_value = "config.toml")]
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    println!("{:?}", cli);
    if cli.server {
        read_friends().await;
        tokio::spawn(async move {
            api::use_api().await.unwrap();
        });

        start_server().await?;
    } else {
        let settings = Config::builder()
            .add_source(config::File::with_name(&cli.path))
            .build()
            .unwrap();

        start_client(&settings.get_string("path")?).await.unwrap();
    }

    Ok(())
}
