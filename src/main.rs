use api_rust::{hello_controller_details, web, App, AppState, HttpServer};
// https://actix.rs/docs/application#configure
#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
