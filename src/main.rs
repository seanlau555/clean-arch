mod models;
mod services;
use crate::services::db::Database;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = Database::init().await;
    let db_data: Data<Database> = Data::new(db);

    HttpServer::new(move || App::new().app_data(db_data.clone()).service(hello))
        .bind(("localhost", 5001))?
        .run()
        .await
}
