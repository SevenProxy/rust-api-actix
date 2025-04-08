use actix_web::HttpRequest;
use crate::{dto::PropsRequests, error::ServerError};

pub struct Requests {
  pub req: HttpRequest,
}

impl Requests {
  pub fn getRequests(self) -> Result<PropsRequests, ServerError> {
    Ok(PropsRequests {
      header: self.req.headers().clone(),
    })
  }
}
