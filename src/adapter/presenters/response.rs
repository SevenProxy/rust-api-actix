use actix_web::{web::Json, HttpResponse, Responder};
use crate::dto::JsonResponse;

pub struct ResponsePresenters {
  pub response: HttpResponse,
}

impl ResponsePresenters {
  pub fn ok(json: JsonResponse) -> Self {
    Self {
      response: HttpResponse::Ok().json(json)
    }
  }

  pub fn bad_request(msg: String) -> Self {
    Self {
      response: HttpResponse::BadRequest().json(JsonResponse {
        value: false,
        message: Some(msg),
        data: None,
      }),
    }
  }

  pub fn internal_error(msg: String) -> Self {
    Self {
      response: HttpResponse::InternalServerError().json(JsonResponse {
        value: false,
        message: Some(msg),
        data: None,
      })
    }
  }

  pub fn into_inner(self) -> HttpResponse {
    self.response
  }

}


impl Responder for ResponsePresenters {
  type Body = actix_web::body::BoxBody;

  fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
    self.response
  }
}
