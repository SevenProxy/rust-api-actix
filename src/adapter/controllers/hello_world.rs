use actix_web::HttpResponse;
use crate::{adapter::presenters::{Requests, Response}, dto::JsonResponse};

pub async fn hello(req: Requests) -> HttpResponse {
  let message_response = JsonResponse {
    value: true,
    message: Some(String::from("hello worlds")),
    data: None,
  };
  let response = Response {
    json: Some(Ok(message_response)),
  };
  response.respond_to_json(req)
}
