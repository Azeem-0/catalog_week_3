#![recursion_limit = "256"]

pub mod models;
pub mod repository;
pub mod services;
pub mod utils;

use utoipa::OpenApi;

use utoipa_swagger_ui::SwaggerUi;

use actix_web::{
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use repository::mongodb_repository::MongoDB;
use services::{
    depth_history_service, earnings_history_service, rune_pool_history_service,
    swaps_history_service,
};
use utils::{api_doc::ApiDoc, scheduler::run_cron_job};

pub async fn home_route() -> HttpResponse {
    HttpResponse::Ok().body("Hello! Welcome to our API")
}

pub async fn init_db() -> Result<Data<MongoDB>, Error> {
    let db = MongoDB::init().await.unwrap();
    Ok(Data::new(db))
}

pub async fn init_server(db_data: Data<MongoDB>) -> std::io::Result<()> {
    actix_web::rt::spawn(run_cron_job(db_data.clone()));

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/", web::get().to(home_route))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(web::scope("/depth-history").configure(depth_history_service::init))
            .service(web::scope("/earnings-history").configure(earnings_history_service::init))
            .service(web::scope("/swaps-history").configure(swaps_history_service::init))
            .service(web::scope("/rune-pool-history").configure(rune_pool_history_service::init))
    })
    .bind(("0.0.0.0", 3000))?
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
