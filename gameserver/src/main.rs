use anyhow::Result;

mod data;
mod game;
mod logging;
mod net;
mod util;

use data::init_assets;
use logging::init_tracing;
use net::Gateway;

#[tokio::main]
async fn main() -> Result<()> {
    println!("    _   __            _       ____                           __ \n   / | / /___ __   __(_)___ _/  _/___ ___  ____  ____ ______/ /_\n  /  |/ / __ `/ | / / / __ `// // __ `__ \\/ __ \\/ __ `/ ___/ __/\n / /|  / /_/ /| |/ / / /_/ // // / / / / / /_/ / /_/ / /__/ /_  \n/_/ |_/\\__,_/ |___/_/\\__,_/___/_/ /_/ /_/ .___/\\__,_/\\___/\\__/  \n                                       /_/                   ");
    println!("Server made by ReversedRooms.\nDiscord: discord.gg/reversedrooms");

    init_tracing();
    init_assets();

    let mut gateway = Gateway::new("0.0.0.0", 22101).await?;
    Box::pin(gateway.listen()).await
}
