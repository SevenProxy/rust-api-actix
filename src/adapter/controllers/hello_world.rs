use actix_web::HttpResponse;
use crate::{adapter::presenters::{RequestsPresenters, ResponsePresenters}, dto::JsonResponse};

pub async fn hello(_req: RequestsPresenters) -> ResponsePresenters {
  let message_response = JsonResponse {
    value: true,
    message: Some(String::from("hello worlds")),
    data: None,
  };
  ResponsePresenters::ok(message_response)
}
