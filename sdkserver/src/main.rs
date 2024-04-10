use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use logging::init_tracing;
use tracing::Level;

use crate::services::{auth, region};

mod logging;
mod services;

const PORT: u16 = 21000;

#[tokio::main]
async fn main() -> Result<()> {
    println!("    _   __            _       ____                           __ \n   / | / /___ __   __(_)___ _/  _/___ ___  ____  ____ ______/ /_\n  /  |/ / __ `/ | / / / __ `// // __ `__ \\/ __ \\/ __ `/ ___/ __/\n / /|  / /_/ /| |/ / / /_/ // // / / / / / /_/ / /_/ / /__/ /_  \n/_/ |_/\\__,_/ |___/_/\\__,_/___/_/ /_/ /_/ .___/\\__,_/\\___/\\__/  \n                                       /_/                   ");
    println!("Server made by ReversedRooms.\nDiscord: discord.gg/reversedrooms");

    init_tracing();

    let span = tracing::span!(Level::DEBUG, "main");
    let _ = span.enter();

    let router = Router::new()
        .route(
            region::QUERY_REGION_LIST_ENDPOINT,
            get(region::query_region_list),
        )
        .route(
            region::QUERY_CUR_REGION_ENDPOINT,
            get(region::query_cur_region),
        )
        .route(auth::RISKY_API_CHECK_ENDPOINT, post(auth::risky_api_check))
        .route(
            auth::LOGIN_WITH_PASSWORD_ENDPOINT,
            post(auth::login_with_password),
        )
        .route(
            auth::GRANTER_LOGIN_VERIFICATION_ENDPOINT,
            post(auth::granter_login_verification),
        );

    let addr = format!("0.0.0.0:{PORT}");
    let server = axum_server::bind(addr.parse()?);

    tracing::info!("sdkserver is listening at {addr}");
    server.serve(router.into_make_service()).await?;

    Ok(())
}
