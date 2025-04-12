use actix_web::{get, HttpRequest};

use crate::adapter::{controllers::hello, presenters::{RequestsPresenters, ResponsePresenters}};

#[get("/hello")]
pub async fn hello_controller_details(req: HttpRequest) -> ResponsePresenters {
  let req = RequestsPresenters::new(req);
  hello(req).await
}
