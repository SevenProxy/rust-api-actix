use actix_web::HttpResponse;

use crate::dto::JsonResponse;

use super::Requests;

pub struct Response {
  pub json: Option<Result<JsonResponse, String>>,
}

impl Response {
  pub fn respond_to_json(self, _req: Requests) -> HttpResponse {
    match self.json {
      Some(Ok(msg_res)) => HttpResponse::Ok().json(msg_res),
      Some(Err(e)) => HttpResponse::BadRequest().json(JsonResponse {
        value: false,
        message: Some(e),
        data: None,
      }),
      None => HttpResponse::InternalServerError().json(JsonResponse {
        value: false,
        message: Some("Nenhum conteúdo JSON configurado.".into()),
        data: None,
      }),
    }
  }
}
