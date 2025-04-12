use std::{io, net::IpAddr};
use actix_web::{http::{header::HeaderMap, Method}, HttpRequest};

use crate::{error::ServerError, log::recive_request};

pub struct RequestsPresenters {
  inner: HttpRequest,
}

impl RequestsPresenters {
  pub fn new(inner: HttpRequest) -> Self {
    let result = Self { inner };
    recive_request(result.get_ip());
    result
  }

  pub fn get_ip(&self) -> Result<IpAddr, ServerError>{
    match self.inner.peer_addr() {
      Some(addr) => Ok(addr.ip()),
      None => Err(ServerError::Io(io::Error::new(
        io::ErrorKind::Other,
        "Não foi possível obter o IP do cliente",
      ))),
    }
  }

  pub fn headers(&self) -> &HeaderMap {
    self.inner.headers()
  }

  pub fn method(&self) -> &Method {
    self.inner.method()
  }

  pub fn path(&self) -> &str {
    self.inner.path()
  }

  //pub fn try_new(req: HttpRequest) -> Result<Self, String> {
  //  if req.headers().contains_key("x-auth") {
  //      Ok(Self { req })
  //  } else {
  //      Err("Cabeçalho de autenticação ausente".into())
  //  }
  //}
}
