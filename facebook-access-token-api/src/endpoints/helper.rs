use facebook_access_token::{
    AccessTokenExpiresIn, AppAccessToken, LongLivedUserAccessToken, PageAccessToken,
    PageSessionInfoAccessToken, ShortLivedUserAccessToken, UserAccessToken,
    UserSessionInfoAccessToken,
};
use facebook_graph_api_object_error::Error;
use http_api_client::{Client, ClientRespondEndpointError};
use http_api_client_endpoint::http::StatusCode;

use crate::{
    endpoints::{AccessTokenEndpoint, DebugTokenEndpoint, EndpointError, EndpointRet},
    objects::{DebugTokenResult, ResponseBodyErrJson},
};

//
// https://developers.facebook.com/docs/facebook-login/guides/access-tokens/get-long-lived#get-a-long-lived-user-access-token
// The short_lived_user_access_token still valid after exchanged.
//
pub async fn get_long_lived_user_access_token<C: Client + Send + Sync>(
    client: &C,
    app_id: u64,
    app_secret: impl AsRef<str>,
    short_lived_user_access_token: impl Into<ShortLivedUserAccessToken>,
) -> Result<
    Result<
        (LongLivedUserAccessToken, Option<AccessTokenExpiresIn>),
        (StatusCode, ResponseBodyErrJson),
    >,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let ep = AccessTokenEndpoint::new(
        "fb_exchange_token",
        app_id,
        Some(app_secret.as_ref().into()),
        Some(short_lived_user_access_token.into().into_inner().into()),
        None,
    );

    let ret = client.respond_endpoint(&ep).await?;

    match ret {
        EndpointRet::Ok(ok_json) => Ok(Ok((
            ok_json.access_token.into(),
            ok_json.expires_in.map(Into::into),
        ))),
        EndpointRet::Other((status_code, Ok(err_json))) => Ok(Err((status_code, err_json))),
        EndpointRet::Other((status_code, Err(body))) => Ok(Err((
            status_code,
            ResponseBodyErrJson {
                error: Error::new_with_status_code_and_body(
                    status_code.as_u16(),
                    String::from_utf8_lossy(&body).as_ref(),
                ),
            },
        ))),
    }
}

//
// https://developers.facebook.com/docs/facebook-login/guides/access-tokens#generating-an-app-access-token
//
pub async fn gen_app_access_token<C: Client + Send + Sync>(
    client: &C,
    app_id: u64,
    app_secret: impl AsRef<str>,
) -> Result<
    Result<AppAccessToken, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let ep = AccessTokenEndpoint::new(
        "client_credentials",
        app_id,
        Some(app_secret.as_ref().into()),
        None,
        None,
    );

    let ret = client.respond_endpoint(&ep).await?;

    match ret {
        EndpointRet::Ok(ok_json) => Ok(Ok(ok_json.access_token.into())),
        EndpointRet::Other((status_code, Ok(err_json))) => Ok(Err((status_code, err_json))),
        EndpointRet::Other((status_code, Err(body))) => Ok(Err((
            status_code,
            ResponseBodyErrJson {
                error: Error::new_with_status_code_and_body(
                    status_code.as_u16(),
                    String::from_utf8_lossy(&body).as_ref(),
                ),
            },
        ))),
    }
}

//
// https://developers.facebook.com/docs/facebook-login/guides/access-tokens/get-session-info#generate-session-info-token
//
async fn gen_x_session_info_access_token_inner<C: Client + Send + Sync>(
    client: &C,
    app_id: u64,
    x_session_info_access_token: &str,
) -> Result<
    Result<(String, Option<AccessTokenExpiresIn>), (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let ep = AccessTokenEndpoint::new(
        "fb_attenuate_token",
        app_id,
        None,
        Some(x_session_info_access_token.into()),
        None,
    );

    let ret = client.respond_endpoint(&ep).await?;

    match ret {
        EndpointRet::Ok(ok_json) => Ok(Ok((
            ok_json.access_token.to_owned(),
            ok_json.expires_in.map(Into::into),
        ))),
        EndpointRet::Other((status_code, Ok(err_json))) => Ok(Err((status_code, err_json))),
        EndpointRet::Other((status_code, Err(body))) => Ok(Err((
            status_code,
            ResponseBodyErrJson {
                error: Error::new_with_status_code_and_body(
                    status_code.as_u16(),
                    String::from_utf8_lossy(&body).as_ref(),
                ),
            },
        ))),
    }
}

//
pub async fn gen_user_session_info_access_token<C: Client + Send + Sync>(
    client: &C,
    app_id: u64,
    long_lived_user_access_token: impl Into<LongLivedUserAccessToken>,
) -> Result<
    Result<
        (UserSessionInfoAccessToken, Option<AccessTokenExpiresIn>),
        (StatusCode, ResponseBodyErrJson),
    >,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    match gen_x_session_info_access_token_inner(
        client,
        app_id,
        long_lived_user_access_token.into().inner(),
    )
    .await
    {
        Ok(Ok((value, expires_in))) => Ok(Ok((value.into(), expires_in))),
        Ok(Err(x)) => Ok(Err(x)),
        Err(err) => Err(err),
    }
}

//
pub async fn gen_page_session_info_access_token<C: Client + Send + Sync>(
    client: &C,
    app_id: u64,
    page_access_token: impl Into<PageAccessToken>,
) -> Result<
    Result<
        (PageSessionInfoAccessToken, Option<AccessTokenExpiresIn>),
        (StatusCode, ResponseBodyErrJson),
    >,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    match gen_x_session_info_access_token_inner(client, app_id, page_access_token.into().inner())
        .await
    {
        Ok(Ok((value, expires_in))) => Ok(Ok((value.into(), expires_in))),
        Ok(Err(x)) => Ok(Err(x)),
        Err(err) => Err(err),
    }
}

//
// https://developers.facebook.com/docs/facebook-login/guides/%20access-tokens/debugging
//
async fn debug_x_access_token_inner<C: Client + Send + Sync>(
    client: &C,
    input_token: &str,
    access_token: &str,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let ep = DebugTokenEndpoint::new(input_token, access_token, None);

    let ret = client.respond_endpoint(&ep).await?;

    match ret {
        EndpointRet::Ok(ok_json) => Ok(Ok(ok_json.data)),
        EndpointRet::Other((status_code, Ok(err_json))) => Ok(Err((status_code, err_json))),
        EndpointRet::Other((status_code, Err(body))) => Ok(Err((
            status_code,
            ResponseBodyErrJson {
                error: Error::new_with_status_code_and_body(
                    status_code.as_u16(),
                    String::from_utf8_lossy(&body).as_ref(),
                ),
            },
        ))),
    }
}

//
pub async fn debug_user_access_token<C: Client + Send + Sync>(
    client: &C,
    short_lived_or_long_lived_user_access_token: impl Into<UserAccessToken>,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let token = short_lived_or_long_lived_user_access_token.into();
    debug_x_access_token_inner(client, token.inner(), token.inner()).await
}

/*
Sometimes, debug_user_access_token without app_access_token will
400
{
    "error": {
        "message": "(#100) You must provide an app access token, or a user access token that is an owner or developer of the app",
        "type": "OAuthException",
        "code": 100,
        "fbtrace_id": "ARIDoDhBfOqF7CZBTrzBCdP"
    }
}
*/
//
pub async fn debug_user_access_token_via_app_access_token<C: Client + Send + Sync>(
    client: &C,
    short_lived_or_long_lived_user_access_token: impl Into<UserAccessToken>,
    app_access_token: impl Into<AppAccessToken>,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let input_token = short_lived_or_long_lived_user_access_token.into();
    let access_token = app_access_token.into();
    debug_x_access_token_inner(client, input_token.inner(), access_token.inner()).await
}

//
pub async fn debug_app_access_token<C: Client + Send + Sync>(
    client: &C,
    app_access_token: impl Into<AppAccessToken>,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let token = app_access_token.into();
    debug_x_access_token_inner(client, token.inner(), token.inner()).await
}

//
pub async fn debug_page_access_token<C: Client + Send + Sync>(
    client: &C,
    page_access_token: impl Into<PageAccessToken>,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let token = page_access_token.into();
    debug_x_access_token_inner(client, token.inner(), token.inner()).await
}

//
pub async fn debug_user_session_info_access_token_via_app_access_token<C: Client + Send + Sync>(
    client: &C,
    user_session_info_access_token: impl Into<UserSessionInfoAccessToken>,
    app_access_token: impl Into<AppAccessToken>,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let input_token = user_session_info_access_token.into();
    let access_token = app_access_token.into();
    debug_x_access_token_inner(client, input_token.inner(), access_token.inner()).await
}

//
pub async fn debug_user_session_info_access_token_via_long_lived_user_access_token<
    C: Client + Send + Sync,
>(
    client: &C,
    user_session_info_access_token: impl Into<UserSessionInfoAccessToken>,
    long_lived_user_access_token: impl Into<LongLivedUserAccessToken>,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let input_token = user_session_info_access_token.into();
    let access_token = long_lived_user_access_token.into();
    debug_x_access_token_inner(client, input_token.inner(), access_token.inner()).await
}

//
pub async fn debug_page_session_info_access_token_via_app_access_token<C: Client + Send + Sync>(
    client: &C,
    page_session_info_access_token: impl Into<PageSessionInfoAccessToken>,
    app_access_token: impl Into<AppAccessToken>,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let input_token = page_session_info_access_token.into();
    let access_token = app_access_token.into();
    debug_x_access_token_inner(client, input_token.inner(), access_token.inner()).await
}

//
pub async fn debug_page_session_info_access_token_via_page_access_token<C: Client + Send + Sync>(
    client: &C,
    page_session_info_access_token: impl Into<PageSessionInfoAccessToken>,
    page_access_token: impl Into<PageAccessToken>,
) -> Result<
    Result<DebugTokenResult, (StatusCode, ResponseBodyErrJson)>,
    ClientRespondEndpointError<C::RespondError, EndpointError, EndpointError>,
> {
    let input_token = page_session_info_access_token.into();
    let access_token = page_access_token.into();
    debug_x_access_token_inner(client, input_token.inner(), access_token.inner()).await
}
