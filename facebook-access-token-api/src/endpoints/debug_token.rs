//! [Ref](https://developers.facebook.com/docs/graph-api/reference/v15.0/debug_token)
//! [Ref](https://developers.facebook.com/docs/facebook-login/guides/%20access-tokens/debugging)

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    endpoints::{
        common::{EndpointError, EndpointRet},
        URL_BASE, VERSION,
    },
    objects::{DebugTokenResult, ResponseBodyErrJson},
};

//
#[derive(Debug, Clone)]
pub struct DebugTokenEndpoint {
    pub input_token: Box<str>,
    pub access_token: Box<str>,
    //
    pub version: Option<Box<str>>,
}

impl DebugTokenEndpoint {
    pub fn new(
        input_token: impl AsRef<str>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            input_token: input_token.as_ref().into(),
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }
}

impl Endpoint for DebugTokenEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<DebugTokenResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/debug_token",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("input_token", &self.input_token)
            .append_pair("access_token", &self.access_token);

        let request = Request::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .header(USER_AGENT, "facebook-access-token-api")
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let status = response.status();
        match status {
            StatusCode::OK => Ok(EndpointRet::Ok(
                serde_json::from_slice(response.body())
                    .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
            )),
            status => match serde_json::from_slice::<ResponseBodyErrJson>(response.body()) {
                Ok(ok_json) => Ok(EndpointRet::Other((status, Ok(ok_json)))),
                Err(_) => Ok(EndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            },
        }
    }
}

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DebugTokenResponseBodyOkJson {
    pub data: DebugTokenResult,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::debug_token::DebugTokenResultTypeExtra;

    #[test]
    fn test_de_response_body_ok_json() {
        //
        //
        //
        //
        let content =
            include_str!("../../tests/response_body_json_files/debug_token__app_access_token.json");
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.data.error.is_none(),);
                match ok_json.data.type_extra.unwrap() {
                    DebugTokenResultTypeExtra::App(info) => {
                        assert_eq!(info.app_id, 257422819769992);
                        assert_eq!(info.application, "oauth2-rs-f-b-web-app-demo");
                    }
                    DebugTokenResultTypeExtra::User(x) => panic!("{:?}", x),
                    DebugTokenResultTypeExtra::Page(x) => panic!("{:?}", x),
                }
            }
            Err(err) => panic!("{}", err),
        }

        //
        //
        //
        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__user_access_token_1.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.data.error.is_none(),);
                match ok_json.data.type_extra.unwrap() {
                    DebugTokenResultTypeExtra::App(x) => panic!("{:?}", x),
                    DebugTokenResultTypeExtra::User(info) => {
                        assert_eq!(info.app_id, 257422819769992);
                        assert_eq!(info.user_id, 123);
                        assert!(info.metadata.is_some());
                    }
                    DebugTokenResultTypeExtra::Page(x) => panic!("{:?}", x),
                }
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__user_access_token_2.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.data.error.is_none(),);
                match ok_json.data.type_extra.unwrap() {
                    DebugTokenResultTypeExtra::App(x) => panic!("{:?}", x),
                    DebugTokenResultTypeExtra::User(info) => {
                        assert_eq!(info.app_id, 257422819769992);
                        assert_eq!(info.user_id, 123);
                        assert!(info.metadata.is_none());
                    }
                    DebugTokenResultTypeExtra::Page(x) => panic!("{:?}", x),
                }
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__user_access_token_3.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.data.error.unwrap().message.contains("has expired"));
                match ok_json.data.type_extra.unwrap() {
                    DebugTokenResultTypeExtra::App(x) => panic!("{:?}", x),
                    DebugTokenResultTypeExtra::User(info) => {
                        assert_eq!(info.app_id, 257422819769992);
                        assert_eq!(info.user_id, 123);
                    }
                    DebugTokenResultTypeExtra::Page(x) => panic!("{:?}", x),
                }
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__user_access_token_4.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json
                    .data
                    .error
                    .unwrap()
                    .message
                    .contains("has been invalidated"));
                match ok_json.data.type_extra.unwrap() {
                    DebugTokenResultTypeExtra::App(x) => panic!("{:?}", x),
                    DebugTokenResultTypeExtra::User(info) => {
                        assert_eq!(info.app_id, 257422819769992);
                        assert_eq!(info.user_id, 123);
                        assert!(info.granular_scopes.is_some());
                    }
                    DebugTokenResultTypeExtra::Page(x) => panic!("{:?}", x),
                }
            }
            Err(err) => panic!("{}", err),
        }

        //
        //
        //
        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__user_access_token_example_1.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(_ok_json) => {
                // println!("{:?}", ok_json);
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__user_access_token_example_2.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(_ok_json) => {
                // println!("{:?}", ok_json);
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content =
            include_str!("../../tests/response_body_json_files/debug_token__200__example_1.json");
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(_ok_json) => {
                // println!("{:?}", ok_json);
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content =
            include_str!("../../tests/response_body_json_files/debug_token__200__example_2.json");
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.data.is_valid);
            }
            Err(err) => panic!("{}", err),
        }

        //
        //
        //
        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__page_access_token.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.data.error.is_none(),);
                match ok_json.data.type_extra.unwrap() {
                    DebugTokenResultTypeExtra::App(x) => panic!("{:?}", x),
                    DebugTokenResultTypeExtra::User(x) => panic!("{:?}", x),
                    DebugTokenResultTypeExtra::Page(info) => {
                        assert_eq!(info.app_id, 257422819769992);
                        assert_eq!(info.user_id, 123);
                        assert_eq!(info.profile_id, 103455271248220);
                        assert!(info.granular_scopes.is_some());
                        assert_eq!(
                            info.granular_scopes
                                .unwrap()
                                .first()
                                .cloned()
                                .unwrap()
                                .target_ids
                                .unwrap()
                                .first()
                                .cloned()
                                .unwrap(),
                            103455271248220
                        );
                    }
                }
            }
            Err(err) => panic!("{}", err),
        }

        //
        //
        //
        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__200__access_token_could_not_be_decrypted.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.data.error.is_some(),);
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../../tests/response_body_json_files/debug_token__200__cannot_get_application_info.json"
        );
        match serde_json::from_str::<DebugTokenResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.data.error.is_some(),);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
