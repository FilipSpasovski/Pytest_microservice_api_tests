use actix_web::http::StatusCode;
use serde::{Serialize, Serializer};
use serde_with::{serde_as, SerializeAs};
// use thiserror::Error;
mod local_status_code {
    use actix_web::http::StatusCode;
    use serde::{self, Serializer};

    pub fn serialize<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(status_code.as_u16())
    }
}

struct LocalStatusCode;
impl SerializeAs<StatusCode> for LocalStatusCode {
    fn serialize_as<S>(value: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        local_status_code::serialize(value, serializer)
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct ErrorInfo {
    pub title: String,
    pub details: String,
    #[serde_as(as = "LocalStatusCode")]
    pub status: StatusCode,
}
