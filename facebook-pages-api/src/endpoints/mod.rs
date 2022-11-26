//
pub mod search;

pub use search::SearchEndpoint;

//
pub mod common;

pub use common::{EndpointError, EndpointRet};

//
pub const URL_BASE: &str = "https://graph.facebook.com";
pub const VERSION: &str = "v15.0";
