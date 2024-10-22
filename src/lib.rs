#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
pub mod models;
pub mod repository;
pub mod routes;
pub mod services;

use actix_web::{
    get,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Responder,
};
use repository::mongodb_repository::MongoDB;
use services::{
    depth_history_service, earnings_history_service, rune_pool_history_service,
    swaps_history_service,
};

#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Namasteee bosssss....")
}

pub async fn init_db() -> Result<Data<MongoDB>, Error> {
    let db = MongoDB::init().await.unwrap();
    Ok(Data::new(db))
}

pub async fn init_server(db_data: Data<MongoDB>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello_world)
            .service(web::scope("/depth-history").configure(depth_history_service::init))
            .service(web::scope("/earnings-history").configure(earnings_history_service::init))
            .service(web::scope("/swaps-history").configure(swaps_history_service::init))
            .service(web::scope("/rune-pool").configure(rune_pool_history_service::init))
    })
    .bind(("localhost", 3000))?
    .run()
    .await
}

pub async fn run() -> std::io::Result<()> {
    let db_data = init_db().await;

    let db_data = match db_data {
        Ok(data) => {
            println!("Successfully connected to database.");
            data
        }
        Err(_) => {
            println!("Failed to connect to the database.");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database connection failed",
            ));
        }
    };

    init_server(db_data).await
}
