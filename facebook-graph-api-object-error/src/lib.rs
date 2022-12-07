//! https://developers.facebook.com/docs/graph-api/guides/error-handling
//! https://developers.facebook.com/docs/instagram-api/reference/error-codes

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};

//
const CODE_STATUS_CODE_AND_BODY: i32 = -2_147_483_001;

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub message: String,
    pub r#type: Option<ErrorType>,
    pub code: i32,
    pub error_subcode: Option<i32>,
    pub error_user_title: Option<String>,
    pub error_user_msg: Option<String>,
    pub fbtrace_id: Option<String>,
    /*
    is_transient https://developers.facebook.com/docs/instagram-api/reference/error-codes
    error_data
    */
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}

impl Error {
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }

    pub fn new_with_status_code_and_body(status_code: u16, body: &str) -> Self {
        let mut extra = Map::new();
        extra.insert("status_code".to_string(), Value::from(status_code));
        extra.insert("body".to_string(), Value::from(body));

        Self {
            message: format!("status_code:{status_code} body:{body}"),
            r#type: None,
            code: CODE_STATUS_CODE_AND_BODY,
            error_subcode: None,
            error_user_title: None,
            error_user_msg: None,
            fbtrace_id: None,
            _extra: Some(extra),
        }
    }

    pub fn as_status_code_and_body(&self) -> Option<(u16, &str)> {
        if self.code != CODE_STATUS_CODE_AND_BODY {
            return None;
        }

        if let Some(extra) = self.extra() {
            if let Some(status_code) = extra.get("status_code").and_then(|x| x.as_i64()) {
                if let Some(body) = extra.get("body").and_then(|x| x.as_str()) {
                    return Some((status_code as u16, body));
                }
            }
        }

        None
    }
}

impl Error {
    pub fn is_error_validating_access_token(&self) -> bool {
        self.message
            .to_lowercase()
            .contains("error validating access token")
    }

    pub fn is_access_token_session_has_been_invalidated(&self) -> bool {
        self.message
            .to_lowercase()
            .contains("session has been invalidated")
    }

    pub fn is_access_token_session_has_expired(&self) -> bool {
        self.message.to_lowercase().contains("session has expired")
    }

    pub fn is_access_token_session_key_is_malformed(&self) -> bool {
        self.message
            .to_lowercase()
            .contains("session key is malformed")
            || (self.message.to_lowercase().contains("session key ")
                && self.message.to_lowercase().contains(" is malformed"))
    }
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone)]
pub enum ErrorType {
    OAuthException,
    GraphMethodException,
    #[serde(other)]
    Other(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

//
//
//
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum KnownErrorCase {
    ApiTooManyCalls,
    ApiUserTooManyCalls,
    AccessTokenExpiredOrRevokedOrInvalid,
    PermissionNotGrantedOrRemoved,
    RetryLater,
}

impl KnownErrorCase {
    pub fn is_api_too_many_calls(&self) -> bool {
        matches!(self, Self::ApiTooManyCalls)
    }

    pub fn is_api_user_too_many_calls(&self) -> bool {
        matches!(self, Self::ApiUserTooManyCalls)
    }

    pub fn is_access_token_expired_or_revoked_or_invalid(&self) -> bool {
        matches!(self, Self::AccessTokenExpiredOrRevokedOrInvalid)
    }

    pub fn is_permission_not_granted_or_removed(&self) -> bool {
        matches!(self, Self::PermissionNotGrantedOrRemoved)
    }

    pub fn is_retry_later(&self) -> bool {
        matches!(self, Self::RetryLater)
    }
}

impl core::fmt::Display for KnownErrorCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for KnownErrorCase {}

impl Error {
    pub fn to_known_error_case(&self) -> Option<KnownErrorCase> {
        if let Some(error_subcode) = self.error_subcode {
            #[allow(clippy::single_match)]
            match error_subcode {
                463 | 467 => return Some(KnownErrorCase::AccessTokenExpiredOrRevokedOrInvalid),
                _ => {}
            }
        }

        match self.code {
            102 => {
                if self.error_subcode.is_none() {
                    return Some(KnownErrorCase::AccessTokenExpiredOrRevokedOrInvalid);
                }
            }
            2 => return Some(KnownErrorCase::RetryLater),
            4 => return Some(KnownErrorCase::ApiTooManyCalls),
            17 => return Some(KnownErrorCase::ApiUserTooManyCalls),
            10 => return Some(KnownErrorCase::PermissionNotGrantedOrRemoved),
            190 => return Some(KnownErrorCase::AccessTokenExpiredOrRevokedOrInvalid),
            200..=299 => return Some(KnownErrorCase::PermissionNotGrantedOrRemoved),
            _ => {}
        }

        None
    }
}

impl From<&Error> for Option<KnownErrorCase> {
    fn from(error: &Error) -> Self {
        error.to_known_error_case()
    }
}

impl From<Error> for Option<KnownErrorCase> {
    fn from(error: Error) -> Self {
        error.to_known_error_case()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug)]
    struct ResponseBodyErrJson {
        error: Error,
    }

    #[test]
    fn test_de_error() {
        //
        let content = include_str!(
            "../tests/response_body_json_files/err__access_token_session_has_been_invalidated.json"
        );
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => {
                // println!("{:?}", err_json);
                assert!(matches!(
                    err_json.error.to_known_error_case().unwrap(),
                    KnownErrorCase::AccessTokenExpiredOrRevokedOrInvalid
                ));
                assert!(err_json.error.is_error_validating_access_token());
                assert!(err_json
                    .error
                    .is_access_token_session_has_been_invalidated());
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../tests/response_body_json_files/err__access_token_session_has_expired.json"
        );
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => {
                // println!("{:?}", err_json);
                assert!(matches!(
                    err_json.error.to_known_error_case().unwrap(),
                    KnownErrorCase::AccessTokenExpiredOrRevokedOrInvalid
                ));
                assert!(err_json.error.is_error_validating_access_token());
                assert!(err_json.error.is_access_token_session_has_expired());
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../tests/response_body_json_files/err__access_token_session_key_is_malformed.json"
        );
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => {
                // println!("{:?}", err_json);
                assert!(matches!(
                    err_json.error.to_known_error_case().unwrap(),
                    KnownErrorCase::AccessTokenExpiredOrRevokedOrInvalid
                ));
                assert!(err_json.error.is_error_validating_access_token());
                assert!(err_json.error.is_access_token_session_key_is_malformed());
            }
            Err(err) => panic!("{}", err),
        }

        /*
        When https://graph.instagram.com/refresh_access_token?grant_type=ig_refresh_token&access_token={short-lived-access-token}
        */
        let content = include_str!(
            "../tests/response_body_json_files/err__access_token_session_key_x_is_malformed.json"
        );
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => {
                // println!("{:?}", err_json);
                assert!(matches!(
                    err_json.error.to_known_error_case().unwrap(),
                    KnownErrorCase::AccessTokenExpiredOrRevokedOrInvalid
                ));
                assert!(err_json.error.is_error_validating_access_token());
                assert!(err_json.error.is_access_token_session_key_is_malformed());
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content =
            include_str!("../tests/response_body_json_files/err__access_token_unknown_1.json");
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => {
                // println!("{:?}", err_json);
                assert!(matches!(
                    err_json.error.to_known_error_case().unwrap(),
                    KnownErrorCase::AccessTokenExpiredOrRevokedOrInvalid
                ));
                assert!(!err_json.error.is_error_validating_access_token());
            }
            Err(err) => panic!("{}", err),
        }
    }
}
