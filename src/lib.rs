mod adapter;
mod dto;
mod error;

pub use actix_web::{HttpServer, App, web};
pub use adapter::gateways::hello_controller_details;
pub struct AppState {
  pub app_name: String,
}
