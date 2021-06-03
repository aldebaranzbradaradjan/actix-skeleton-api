use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

use dotenv::dotenv;
use std::env;

mod db;
mod errors;
mod handlers;
mod mails;
mod middlewares;
mod templates;

use crate::db as database;
use crate::handlers as handler;
use crate::middlewares::session::Level;

use actix::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // ASCII art banner always looks cool
    // https://www.patorjk.com/software/taag/#p=display&h=0&v=0&f=Bloody&t=Skeleton
    println!(
        "
          ██████  ██ ▄█▀▓█████  ██▓    ▓█████ ▄▄▄█████▓ ▒█████   ███▄    █ 
        ▒██    ▒  ██▄█▒ ▓█   ▀ ▓██▒    ▓█   ▀ ▓  ██▒ ▓▒▒██▒  ██▒ ██ ▀█   █ 
        ░ ▓██▄   ▓███▄░ ▒███   ▒██░    ▒███   ▒ ▓██░ ▒░▒██░  ██▒▓██  ▀█ ██▒
          ▒   ██▒▓██ █▄ ▒▓█  ▄ ▒██░    ▒▓█  ▄ ░ ▓██▓ ░ ▒██   ██░▓██▒  ▐▌██▒
        ▒██████▒▒▒██▒ █▄░▒████▒░██████▒░▒████▒  ▒██▒ ░ ░ ████▓▒░▒██░   ▓██░
        ▒ ▒▓▒ ▒ ░▒ ▒▒ ▓▒░░ ▒░ ░░ ▒░▓  ░░░ ▒░ ░  ▒ ░░   ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
       ░ ░▒  ░ ░░ ░▒ ▒░ ░ ░  ░░ ░ ▒  ░ ░ ░  ░    ░      ░ ▒ ▒░ ░ ░░   ░ ▒░
        ░  ░  ░  ░ ░░ ░    ░     ░ ░      ░     ░      ░ ░ ░ ▒     ░   ░ ░ 
              ░  ░  ░      ░  ░    ░  ░   ░  ░             ░ ░           ░ 
                                                                     
                                                                 
        VERSION : DEV 0.0.1     
        Your server is up and running at http://127.0.0.1:8080\n
    "
    );

    let pool = database::init_pool().expect("Failed to create pool");
    let postman = mails::Postman.start();

    HttpServer::new(move || {
        App::new()
            // add the pool to app state
            .data(pool.clone())
            .data(postman.clone())
            // PURE API
            .service(
                web::scope("/api/v1")
                    // lock down routes with User Middleware
                    .wrap(middlewares::session::BrancaSession(Level::User))
                    // AUTH routes
                    .route("/login", web::post().to(handler::user::login))
                    .route("/logout", web::get().to(handler::user::logout))
                    // USER routes
                    .service(
                        web::scope("/user")
                            .route("", web::get().to(handler::user::get))
                            .route("/update", web::put().to(handler::user::update))
                            .route("/delete", web::delete().to(handler::user::delete))
                            .route("/register", web::post().to(handler::user::register))
                            .route(
                                "/forgot_password",
                                web::post().to(handler::user::forgot_password),
                            )
                            .route(
                                "/reset_password",
                                web::post().to(handler::user::reset_password),
                            )
                            .route(
                                "/change_password",
                                web::post().to(handler::user::change_password),
                            ),
                    ),
            )
            // DASHBOARD
            .service(
                web::scope("/dashboard/")
                    // lock down routes with Admin Middleware
                    .wrap(middlewares::session::BrancaSession(Level::Admin))
                    .route("login", web::get().to(handler::dashboard::dashboard_login))
                    .service(
                        Files::new(
                            "",
                            env::var("DASHBOARD_PATH").expect("DASHBOARD_PATH must be set"),
                        )
                        .index_file("index.html"),
                    ),
            )
            // PUBLICS FILES
            .service(Files::new(
                "/",
                env::var("PUBLIC_PATH").expect("PUBLIC_PATH must be set"),
            ))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
