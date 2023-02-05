use serde::{Deserialize, Serialize};

/// Structure describing a decode request from a client
#[derive(Serialize, Deserialize, Debug)]
pub struct DecodeRequest {
    pub id: String,
    pub openlr_code: String,  // base64 encoded OpenLR code
    pub params_key: String,   // parameter set to be used during decode
    pub url: String,          // URL of MapServer
    pub credentials: String,  // Credentials authorizing use of MapServer
    pub log_level: String     // Logging level: trace | debug | info | warn | error | fatal
}