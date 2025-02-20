use crate::common::AppInfo;
use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationMilliSeconds};
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum HealthCheckItemStatus {
    Up,
    Down,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct StatusItem {
    status: HealthCheckItemStatus,
    name: String,
    #[serde_as(as = "DurationMilliSeconds")]
    response_time_ms: Duration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthCheckStatus {
    status: HealthCheckItemStatus,
    version: Option<String>,
    start_time: Option<DateTime<Utc>>,
}

#[tracing::instrument(
    name = "Check ML endpoint status",
    skip(app_info),
    fields(
        start_at = %app_info.start_at,
    )
)]
pub async fn status(app_info: web::Data<AppInfo>) -> Result<HttpResponse, Error> {
    let version = env!("CARGO_PKG_VERSION");

    let status = HealthCheckStatus {
        status: HealthCheckItemStatus::Up,
        version: Some(String::from(version)),
        start_time: Some(app_info.start_at),
    };

    Ok(HttpResponse::Ok().json(status))
}
