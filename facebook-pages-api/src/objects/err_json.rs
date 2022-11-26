use facebook_graph_api_object_error::Error;
use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrJson {
    pub error: Error,
}
