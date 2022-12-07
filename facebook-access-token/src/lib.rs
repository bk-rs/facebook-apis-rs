//! [Ref](https://developers.facebook.com/docs/facebook-login/guides/access-tokens)

use core::time::Duration;

//
//
//
// https://developers.facebook.com/docs/facebook-login/guides/access-tokens/get-long-lived
pub const LONG_LIVED_USER_ACCESS_TOKEN_LIFETIME: Duration = Duration::from_secs(3600 * 24 * 60);
pub const SHORT_LIVED_USER_ACCESS_TOKEN_LIFETIME_MIN: Duration = Duration::from_secs(3600);
pub const SHORT_LIVED_USER_ACCESS_TOKEN_LIFETIME_MAX: Duration = Duration::from_secs(3600 * 2);

wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LongLivedUserAccessToken(String);
}

wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ShortLivedUserAccessToken(String);
}

wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UserAccessToken(String);
}

impl From<LongLivedUserAccessToken> for UserAccessToken {
    fn from(t: LongLivedUserAccessToken) -> Self {
        Self(t.into_inner())
    }
}

impl From<&LongLivedUserAccessToken> for UserAccessToken {
    fn from(t: &LongLivedUserAccessToken) -> Self {
        Self(t.inner().into())
    }
}

impl From<ShortLivedUserAccessToken> for UserAccessToken {
    fn from(t: ShortLivedUserAccessToken) -> Self {
        Self(t.into_inner())
    }
}

impl From<&ShortLivedUserAccessToken> for UserAccessToken {
    fn from(t: &ShortLivedUserAccessToken) -> Self {
        Self(t.inner().into())
    }
}

//
//
//
wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AppAccessToken(String);
}

impl AppAccessToken {
    pub fn with_app_secret(app_id: u64, app_secret: impl AsRef<str>) -> Self {
        Self(format!("{}|{}", app_id, app_secret.as_ref()))
    }

    pub fn app_id_and_app_secret(&self) -> Option<(u64, &str)> {
        let mut split = self.0.split('|');
        if let Some(app_id) = split.next().and_then(|x| x.parse::<u64>().ok()) {
            if let Some(app_secret) =
                split
                    .next()
                    .and_then(|x| if x.is_empty() { None } else { Some(x) })
            {
                if split.next().is_none() {
                    return Some((app_id, app_secret));
                }
            }
        }
        None
    }
}

//
//
//
/*
Get from https://graph.facebook.com/v15.0/me?fields=id%2Cname%2Caccounts%7Bid%2Cname%2Caccess_token%7D&access_token=YOUR_SHORT_LIVED_OR_LONG_LIVED_USER_ACCESS_TOKEN
PageAccessToken expires == UserAccessToken expires
*/
wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PageAccessToken(String);
}

//
//
//
wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ClientAccessToken(String);
}

impl ClientAccessToken {
    pub fn new(app_id: u64, client_token: impl AsRef<str>) -> Self {
        Self(format!("{}|{}", app_id, client_token.as_ref()))
    }
}

//
//
//
/*
https://developers.facebook.com/docs/facebook-login/guides/access-tokens/get-session-info
does not grant access to user data
calling the debug_token endpoint to verify that it is valid
*/
//
wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UserSessionInfoAccessToken(String);
}

//
wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PageSessionInfoAccessToken(String);
}

//
//
//
wrapping_macro::wrapping! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AccessTokenExpiresIn(usize);
}
impl core::fmt::Display for AccessTokenExpiresIn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_access_token() {
        assert_eq!(
            AppAccessToken::with_app_secret(1, "x").app_id_and_app_secret(),
            Some((1, "x"))
        );
        assert_eq!(
            AppAccessToken::from("1|y").app_id_and_app_secret(),
            Some((1, "y"))
        );
        assert!(AppAccessToken::from("1").app_id_and_app_secret().is_none());
        assert!(AppAccessToken::from("1|").app_id_and_app_secret().is_none());
        assert!(AppAccessToken::from("|x").app_id_and_app_secret().is_none());
    }
}
