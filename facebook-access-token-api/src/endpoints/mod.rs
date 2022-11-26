//
pub mod access_token;
pub mod debug_token;

pub use access_token::AccessTokenEndpoint;
pub use debug_token::DebugTokenEndpoint;

//
pub mod common;

pub use common::{EndpointError, EndpointRet};

pub mod helper;

pub use helper::*;

//
pub const URL_BASE: &str = "https://graph.facebook.com";
pub const VERSION: &str = "v15.0";
