use actix_web::http::header::HeaderMap;
use serde::{Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct JsonResponse {
  pub value: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<Value>,
}


pub struct PropsRequests {
  pub header: HeaderMap,
}
