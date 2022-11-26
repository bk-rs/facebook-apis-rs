//! [Ref](https://developers.facebook.com/docs/facebook-login/guides/access-tokens/get-long-lived#get-a-long-lived-user-access-token)

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
    objects::ResponseBodyErrJson,
};

//
#[derive(Debug, Clone)]
pub struct AccessTokenEndpoint {
    pub grant_type: Box<str>,
    pub app_id: u64,
    pub app_secret: Option<Box<str>>,
    pub fb_exchange_token: Option<Box<str>>,
    //
    pub version: Option<Box<str>>,
}

impl AccessTokenEndpoint {
    pub fn new(
        grant_type: impl AsRef<str>,
        app_id: u64,
        app_secret: impl Into<Option<Box<str>>>,
        fb_exchange_token: impl Into<Option<Box<str>>>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            grant_type: grant_type.as_ref().into(),
            app_id,
            app_secret: app_secret.into(),
            fb_exchange_token: fb_exchange_token.into(),
            version: version.into(),
        }
    }
}

impl Endpoint for AccessTokenEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<AccessTokenResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/oauth/access_token",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("grant_type", &self.grant_type);

        url.query_pairs_mut()
            .append_pair("client_id", &self.app_id.to_string());

        if let Some(client_secret) = &self.app_secret {
            url.query_pairs_mut()
                .append_pair("client_secret", client_secret);
        }

        if let Some(fb_exchange_token) = &self.fb_exchange_token {
            url.query_pairs_mut()
                .append_pair("fb_exchange_token", fb_exchange_token);
        }

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
                Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
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
pub struct AccessTokenResponseBodyOkJson {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<usize>,
}
