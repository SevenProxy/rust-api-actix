use actix_web::{get, HttpRequest, HttpResponse};

use crate::adapter::{controllers::hello, presenters::{Requests, Response}};

#[get("/hello")]
pub async fn hello_controller_details(req: HttpRequest) -> HttpResponse {
  let req = Requests {
    req,
  };
  hello(req).await
}
