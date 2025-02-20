use crate::inference::inference;
use actix_web::{web, Error, HttpRequest, HttpResponse};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str;

#[derive(Debug, Deserialize)]
pub struct PredictSeverityInput {
    pub vehicle_sit: f32,
    pub weather: f32,
    pub sex: f32,
    pub year: f32,
    pub birth_year: f32,
    pub security_used: f32,
    pub hour: f32,
    pub luminosity: f32,
    pub department: f32,
    pub in_agglomeration: f32,
    pub collision_type: f32,
    pub road_type: f32,
    pub pathways_width: f32,
    pub vehicle_type: f32,
    pub obstacle_type: f32,
    pub shock_location: f32,
    pub maneuver_type: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MLOutput {
    severity: u32,
}

#[tracing::instrument(
    skip(req_body),
    fields(
        weather = ?req_body.weather,
        sex = ?req_body.sex,
        year = ?req_body.year,
        birth_year = ?req_body.birth_year,
        security_used = ?req_body.security_used,
        hour =?req_body.hour,
        luminosity = ?req_body.luminosity,
        department = ?req_body.department,
        in_agglomeration = ?req_body.in_agglomeration,
        collision_type = ?req_body.collision_type,
        road_type = ?req_body.road_type,
        pathways_width = ?req_body.pathways_width,
        vehicle_type = ?req_body.vehicle_type,
        obstacle_type = ?req_body.obstacle_type,
        shock_location = ?req_body.shock_location,
        maneuver_type= ?req_body.maneuver_type,
    )
)]
pub async fn predict_severity(
    req_body: web::Json<PredictSeverityInput>,
    _: HttpRequest,
) -> Result<HttpResponse, Error> {
    let input = [
        req_body.vehicle_sit,
        req_body.weather,
        req_body.sex,
        req_body.year,
        req_body.birth_year,
        req_body.security_used,
        req_body.hour,
        req_body.luminosity,
        req_body.department,
        req_body.in_agglomeration,
        req_body.collision_type,
        req_body.road_type,
        req_body.pathways_width,
        req_body.vehicle_type,
        req_body.obstacle_type,
        req_body.shock_location,
        req_body.maneuver_type,
    ];
    let prediction = MLOutput {
        severity: inference(&input),
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(json!(prediction)))
}
