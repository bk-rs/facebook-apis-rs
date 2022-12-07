/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p facebook-access-token-api-demo --bin fb_a_t_a_page_access_token_simple -- 'YOUR_APP_ID' 'YOUR_APP_SECRET' 'YOUR_PAGE_ACCESS_TOKEN'
*/

use std::{env, error};

use facebook_access_token_api::{
    endpoints::{
        debug_page_access_token, debug_page_session_info_access_token_via_app_access_token,
        debug_page_session_info_access_token_via_page_access_token,
        gen_page_session_info_access_token, DebugTokenEndpoint, EndpointRet,
    },
    facebook_access_token::AppAccessToken,
};
use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let app_id = env::args().nth(1).unwrap().parse::<u64>().unwrap();
    let app_secret = env::args().nth(2).unwrap();
    let page_access_token = env::args().nth(3).unwrap();

    //
    let client = IsahcClient::new()?;

    //
    let page_access_token_debug_result = debug_page_access_token(&client, &page_access_token)
        .await?
        .map_err(|(status_code, err_json)| format!("{status_code} {err_json:?}"))?;
    println!("page_access_token debug_result:{page_access_token_debug_result:?}");

    //
    let (page_session_info_access_token, page_session_info_access_token_expires_in) =
        gen_page_session_info_access_token(&client, app_id, &page_access_token)
            .await?
            .map_err(|(status_code, err_json)| format!("{status_code} {err_json:?}"))?;

    println!(
        "page_session_info_access_token value:{page_session_info_access_token} expires_in:{page_session_info_access_token_expires_in:?}"
    );

    //
    {
        let ep = DebugTokenEndpoint::new(
            page_session_info_access_token.inner(),
            page_session_info_access_token.inner(),
            None,
        );
        let ret = client.respond_endpoint(&ep).await?;
        match ret {
            EndpointRet::Other((status_code, Ok(err_json))) => {
                println!("{status_code} {err_json:?}");
                if status_code.as_u16() != 400
                    || !err_json.error.message.to_lowercase().contains(
                        "Invalid OAuth access token - Debug only access token"
                            .to_lowercase()
                            .as_str(),
                    )
                {
                    eprintln!(
                        "debug_token page_session_info_access_token {status_code} {err_json:?}"
                    );
                }
            }
            ret => panic!("{ret:?}"),
        }
    }

    //
    let page_session_info_access_token_debug_result =
        debug_page_session_info_access_token_via_page_access_token(
            &client,
            page_session_info_access_token.inner(),
            &page_access_token,
        )
        .await?
        .map_err(|(status_code, err_json)| format!("{status_code} {err_json:?}"))?;
    println!(
        "page_session_info_access_token debug_result:{page_session_info_access_token_debug_result:?}"
    );

    //
    let page_session_info_access_token_debug_result =
        debug_page_session_info_access_token_via_app_access_token(
            &client,
            page_session_info_access_token.inner(),
            AppAccessToken::with_app_secret(app_id, &app_secret),
        )
        .await?
        .map_err(|(status_code, err_json)| format!("{status_code} {err_json:?}"))?;
    println!(
        "page_session_info_access_token debug_result:{page_session_info_access_token_debug_result:?}"
    );

    Ok(())
}
