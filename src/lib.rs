mod adapter;
mod dto;
mod error;
mod log;

pub use actix_web::{HttpServer, App, web};
pub use adapter::gateways::hello_controller_details;
pub use log::start_server;
pub struct AppState {
  pub app_name: String,
}
