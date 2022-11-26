use facebook_graph_api_object_error::Error;
use serde::{Deserialize, Serialize};

//
/*
When the access_token has expired,
debug_token endpoint error same as access_token endpoint grant_type=fb_exchange_token error.
*/
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrJson {
    pub error: Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_err_json() {
        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__400__debug_only_access_token.json"
        );
        match serde_json::from_str::<ErrJson>(content) {
            Ok(err_json) => {
                // println!("{:?}", err_json);
                assert_eq!(
                    err_json.error.message,
                    "Invalid OAuth access token - Debug only access token"
                );
            }
            Err(err) => panic!("{}", err),
        }
    }
}
