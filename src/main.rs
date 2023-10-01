pub mod json_serialization;
mod processes;
mod state;
mod to_do;
mod views;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use futures::future;
use sentry;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn two() -> impl Responder {
    "Two reached - not!\n"
}

fn main() -> std::io::Result<()> {
    // Sentry init
    let sentry_dsn: String = dotenv::var("SENTRY_DSN").unwrap();
    let _guard = sentry::init((
        sentry_dsn.as_str(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
    std::env::set_var("RUST_BACKTRACE", "1");

    // Running 3 Servers in parallel
    let server1 = HttpServer::new(move || {
        println!("worker here");
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .bind("127.0.0.1:9093")?
    .workers(3) // If not set -> using the amount of threads
    .run();

    let server2 = HttpServer::new(move || {
        println!("worker here");
        App::new().service(web::resource("/one").route(web::get().to(|| two())))
    })
    .bind("127.0.0.1:9094")?
    .workers(3) // If not set -> using the amount of threads
    .run();

    let server3 = HttpServer::new(move || {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:9095")?
    .workers(3) // If not set -> using the amount of threads
    .run();

    // We have to create the actix runtime manually due to sentry needing to be initialized first
    actix_web::rt::System::new()
        .block_on(async { future::try_join3(server1, server2, server3).await })?;
    Ok(())
}
