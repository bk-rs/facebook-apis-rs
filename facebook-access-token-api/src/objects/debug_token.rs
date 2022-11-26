use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use facebook_graph_api_object_error::Error;
use facebook_permission::FacebookPermission;
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json::Value;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DebugTokenResult {
    pub is_valid: bool,
    pub scopes: Vec<String>,
    pub error: Option<Error>,
    #[serde(flatten)]
    pub type_extra: Option<DebugTokenResultTypeExtra>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DebugTokenResultTypeExtra {
    #[serde(rename = "APP")]
    App(DebugTokenResultAppTypeExtra),
    #[serde(rename = "USER")]
    User(DebugTokenResultUserTypeExtra),
    #[serde(rename = "PAGE")]
    Page(DebugTokenResultPageTypeExtra),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DebugTokenResultAppTypeExtra {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub app_id: u64,
    pub application: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DebugTokenResultUserTypeExtra {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub app_id: u64,
    pub application: String,
    //
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub user_id: u64,
    #[serde(default, with = "ts_seconds_option")]
    pub issued_at: Option<DateTime<Utc>>,
    #[serde(default, with = "ts_seconds")]
    pub expires_at: DateTime<Utc>,
    #[serde(default, with = "ts_seconds")]
    pub data_access_expires_at: DateTime<Utc>,
    //
    pub metadata: Option<Value>,
    pub granular_scopes: Option<Vec<DebugTokenResultUserTypeExtraGranularScope>>,
}

impl DebugTokenResultUserTypeExtra {
    pub fn expires(&self) -> DebugTokenResultExpires {
        if self.expires_at.timestamp() == 0 {
            DebugTokenResultExpires::Never
        } else {
            DebugTokenResultExpires::Date(self.expires_at)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DebugTokenResultPageTypeExtra {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub app_id: u64,
    pub application: String,
    //
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub user_id: u64,
    #[serde(default, with = "ts_seconds_option")]
    pub issued_at: Option<DateTime<Utc>>,
    #[serde(default, with = "ts_seconds")]
    pub expires_at: DateTime<Utc>,
    #[serde(default, with = "ts_seconds")]
    pub data_access_expires_at: DateTime<Utc>,
    //
    pub metadata: Option<Value>,
    pub granular_scopes: Option<Vec<DebugTokenResultUserTypeExtraGranularScope>>,
    //
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub profile_id: u64,
}

impl DebugTokenResultPageTypeExtra {
    pub fn expires(&self) -> DebugTokenResultExpires {
        if self.expires_at.timestamp() == 0 {
            DebugTokenResultExpires::Never
        } else {
            DebugTokenResultExpires::Date(self.expires_at)
        }
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DebugTokenResultUserTypeExtraGranularScope {
    pub scope: FacebookPermission,
    #[serde(default, deserialize_with = "deserialize_target_ids")]
    pub target_ids: Option<Vec<i64>>,
}

fn deserialize_target_ids<'de, D>(deserializer: D) -> Result<Option<Vec<i64>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum VecOrNull {
        Vec(Vec<String>),
        Null,
    }

    match VecOrNull::deserialize(deserializer)? {
        VecOrNull::Vec(v) => v
            .into_iter()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        VecOrNull::Null => Ok(None),
    }
}

//
#[derive(Debug, Clone)]
pub enum DebugTokenResultExpires {
    Never,
    Date(DateTime<Utc>),
}
