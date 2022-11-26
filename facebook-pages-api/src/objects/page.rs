//! [Ref](https://developers.facebook.com/docs/pages/searching#fields)

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageForSearchEndpoint {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub is_eligible_for_branded_content: Option<bool>,
    pub is_unclaimed: Option<bool>,
    pub link: Box<str>,
    pub location: Option<PageLocationForSearchEndpoint>,
    pub name: Box<str>,
    pub verification_status: Option<Box<str>>,
}

impl PageForSearchEndpoint {
    pub fn fields() -> Box<str> {
        "id,name,location{city,country,latitude,longitude,state,street,zip},link,is_eligible_for_branded_content,is_unclaimed,verification_status".into()
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageLocationForSearchEndpoint {
    pub city: Option<Box<str>>,
    pub country: Option<Box<str>>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub state: Option<Box<str>>,
    pub street: Option<Box<str>>,
    // e.g. 150-0022
    pub zip: Option<Box<str>>,
}
