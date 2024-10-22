use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use repository::mongodb_repository::MongoDB;

pub mod models;
pub mod repository;
pub mod routes;
pub mod services;

#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Namasteee bosssss....")
}

pub async fn init_db() -> Data<MongoDB> {
    let db = MongoDB::init().await.unwrap();
    Data::new(db)
}

pub async fn init_server(db_data: Data<MongoDB>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().app_data(db_data.clone()).service(hello_world)
        // .service(web::scope("/note").configure(notes_service::init))
        // .service(web::scope("/todos").configure(todo_service::init))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

pub async fn run() -> std::io::Result<()> {
    let db_data = init_db().await;

    init_server(db_data).await
}
