mod counter;
mod database;
mod helpers;
pub mod json_serialization;
mod jwt;
mod models;
mod schema;
mod views;

#[macro_use]
extern crate diesel;

use crate::helpers::env::get_float_from_env;
use actix_cors::Cors;
use actix_service::Service;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use dotenv::dotenv;
use futures::future;
use std::env;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn two() -> impl Responder {
    "Two reached - not!\n"
}

fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    // Sentry init
    create_sentry();

    env::set_var("RUST_BACKTRACE", "1");

    // Running 3 Servers in parallel
    let server1 = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .bind("127.0.0.1:9093")?
    .workers(1) // If not set -> using the amount of threads
    .run();

    let server2 = HttpServer::new(move || {
        App::new().service(web::resource("/one").route(web::get().to(two)))
    })
    .bind("127.0.0.1:9094")?
    .workers(1) // If not set -> using the amount of threads
    .run();

    // Counter for requests
    let site_counter = counter::Counter { count: 0 };
    match site_counter.save() {
        Ok(_) => {}
        Err(error) => {
            sentry::capture_error(&error);
        }
    };

    let server3 = HttpServer::new(move || {
        // Handling CORS issues
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        // Returning the app
        App::new()
            .wrap_fn(|req, srv| {
                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count += 1;
                println!("Http requests counting: {}", &site_counter);
                match site_counter.save() {
                    Ok(_) => {}
                    Err(error) => {
                        sentry::capture_error(&error);
                    }
                };

                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors)
    })
    .bind("127.0.0.1:9095")?
    .workers(3) // If not set -> using the amount of threads
    .run();

    // We have to create the actix runtime manually due to sentry needing to be initialized first
    actix_web::rt::System::new()
        .block_on(async { future::try_join3(server1, server2, server3).await })?;
    Ok(())
}

fn create_sentry() {
    let sentry_dsn: String = env::var("SENTRY_DSN").unwrap();
    let sample_rate = get_float_from_env("SENTRY_SAMPLE_RATE".to_string());

    let _guard = sentry::init((
        sentry_dsn.as_str(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            // Placeholder for now - not yet enabled
            traces_sample_rate: sample_rate,
            ..Default::default()
        },
    ));
}
