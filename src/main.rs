use api_rust::{hello_controller_details, start_server, web, App, AppState, HttpServer};
use tracing_subscriber::fmt;
const PORT: u16 = 3000;
// https://actix.rs/docs/application#configure

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  fmt()
    .with_max_level(tracing::Level::INFO)
    .with_target(false)
    .pretty()
    .init();
  start_server();
  HttpServer::new(|| {
    App::new()
      .app_data(web::Data::new(AppState {
        app_name: String::from("Rust API"),
      }))
      .service(
        web::scope("/api")
          .service(hello_controller_details)
      )
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
