// use actix_web::{App, HttpServer};
// mod routes;
// mod postgres;
// mod handlers;
mod errors;
mod handlers;
mod models;
mod schema;
// use actix_web::{web, App, HttpServer};
// use crate::routes::*;
// use crate::postgres::*;


#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};



pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");


    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("0.0.0.0:9000")?
    // .bind("127.0.0.1:8080")?
    .run()
    .await
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     std::env::set_var("RUST_LOG", "actix_web=debug");

//     // Start http server
//     HttpServer::new(move || {
//         App::new()
//             .route("/users", web::get().to(handlers::get_users))
//             .route("/users/{id}", web::get().to(handlers::get_user_by_id))
//             .route("/users", web::post().to(handlers::add_user))
//             .route("/users/{id}", web::delete().to(handlers::delete_user))
//     })
    
//     .bind("0.0.0.0:9000")?
//     // .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     establish_connection();
//     HttpServer::new(|| {
//         App::new()
//             .data(pool.clone())
//             .configure(init_router)
//             .route("/users", web::get().to(handlers::get_users))
//             .route("/users/{id}", web::get().to(handlers::get_user_by_id))
//             .route("/users", web::post().to(handlers::add_user))
//             .route("/users/{id}", web::delete().to(handlers::delete_user))
//     })
//     .bind("0.0.0.0:9000")?
//     .run()
//     .await
// }