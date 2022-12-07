/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p facebook-pages-api-demo --bin fb_pages_api_search -- 'Facebook' 'YOUR_ACCESS_TOKEN'
*/

use std::{env, error};

use facebook_pages_api::endpoints::{EndpointRet, SearchEndpoint};
use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let q = env::args().nth(1).unwrap();
    let access_token = env::args().nth(2).unwrap();

    //
    let client = IsahcClient::new()?;

    //
    let ep = SearchEndpoint::new(q, access_token, None);
    let ret = client.respond_endpoint(&ep).await?;
    match ret {
        EndpointRet::Ok(ok_json) => {
            println!("{ok_json:?}");
        }
        EndpointRet::Other((status_code, Ok(err_json))) => {
            println!("{status_code} {err_json:?}");
        }
        EndpointRet::Other((status_code, Err(body))) => {
            println!("{} {:?}", status_code, String::from_utf8_lossy(&body));
        }
    }

    Ok(())
}
