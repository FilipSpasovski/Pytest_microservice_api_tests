use actix_web::dev::Server;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{
    body::ResponseBody, dev, dev::Body, error, http, web, App, HttpResponse, HttpServer, Result,
};
use chrono::Utc;
// use serde::Deserialize;
use serde_json::json;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::common::{AppInfo, ErrorInfo};
use crate::routes::{predict_severity, status};

const MAX_PAYLOAD_SIZE: usize = 4096;

fn render<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let body = json!(ErrorInfo {
        title: "Request error".to_string(),
        details: res.status().canonical_reason().unwrap().to_string(),
        status: res.status(),
    });
    let res = res.map_body(|_, _| ResponseBody::Body(Body::from(body)).into_body());
    Ok(ErrorHandlerResponse::Response(res))
}

fn handler_json_error(err: error::JsonPayloadError, _req: &web::HttpRequest) -> actix_web::Error {
    let resp = match &err {
        error::JsonPayloadError::Overflow => HttpResponse::PayloadTooLarge().json(ErrorInfo {
            title: "Request error".to_string(),
            details: format!("payload overflow, max size is {} bytes", MAX_PAYLOAD_SIZE),
            status: http::StatusCode::PAYLOAD_TOO_LARGE,
        }),
        error::JsonPayloadError::ContentType => HttpResponse::BadRequest().json(ErrorInfo {
            title: "Request error".to_string(),
            details: "wrong Content-Type, only accept application/json".to_string(),
            status: http::StatusCode::BAD_REQUEST,
        }),

        error::JsonPayloadError::Deserialize(json_error) => {
            HttpResponse::BadRequest().json(ErrorInfo {
                title: "Request error".to_string(),
                details: format!("{}", json_error),
                status: http::StatusCode::BAD_REQUEST,
            })
        }
        error::JsonPayloadError::Payload(payload_error) => {
            HttpResponse::BadRequest().json(ErrorInfo {
                title: "Request error".to_string(),
                details: format!("{}", payload_error),
                status: http::StatusCode::BAD_REQUEST,
            })
        }
    };
    error::InternalError::from_response(err, resp).into()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            //.wrap(ErrorHandlers::new().handler(http::StatusCode::BAD_REQUEST, render))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::FORBIDDEN, render))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, render))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::UNPROCESSABLE_ENTITY, render))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render))
            .app_data(
                // Json extractor configuration for this resource.
                web::JsonConfig::default()
                    .limit(MAX_PAYLOAD_SIZE) // Limit request payload size
                    .content_type(|mime| mime.type_() == mime::JSON)
                    .error_handler(handler_json_error),
            )
            .route("/status", web::get().to(status))
            .route("/predictions/severity", web::post().to(predict_severity))
            .data(AppInfo {
                start_at: Utc::now(),
            })
    })
    .listen(listener)?
    .run();

    Ok(server)
}
