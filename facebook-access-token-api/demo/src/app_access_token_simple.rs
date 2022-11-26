/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p facebook-access-token-api-demo --bin fb_a_t_a_app_access_token_simple -- 'YOUR_APP_ID' 'YOUR_APP_SECRET'
*/

use std::{env, error};

use facebook_access_token_api::{
    endpoints::{debug_app_access_token, gen_app_access_token},
    facebook_access_token::AppAccessToken,
};
use futures_lite::future::block_on;
use http_api_isahc_client::IsahcClient;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let app_id = env::args().nth(1).unwrap().parse::<u64>().unwrap();
    let app_secret = env::args().nth(2).unwrap();

    //
    let client = IsahcClient::new()?;

    //
    let app_access_token = gen_app_access_token(&client, app_id, &app_secret)
        .await?
        .map_err(|(status_code, err_json)| format!("{} {:?}", status_code, err_json))?;

    println!("app_access_token value:{}", app_access_token);

    assert_eq!(app_access_token.app_id_and_app_secret().unwrap().0, app_id);

    //
    let app_access_token_debug_result = debug_app_access_token(&client, app_access_token)
        .await?
        .map_err(|(status_code, err_json)| format!("{} {:?}", status_code, err_json))?;
    println!(
        "app_access_token debug_result:{:?}",
        app_access_token_debug_result
    );

    //
    let app_access_token_debug_result = debug_app_access_token(
        &client,
        AppAccessToken::with_app_secret(app_id, &app_secret),
    )
    .await?
    .map_err(|(status_code, err_json)| format!("{} {:?}", status_code, err_json))?;
    println!(
        "app_access_token with_app_secret debug_result:{:?}",
        app_access_token_debug_result
    );

    Ok(())
}
