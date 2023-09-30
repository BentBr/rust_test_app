mod processes;
mod state;
mod to_do;
mod views;
pub mod json_serialization;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _guard = sentry::init(("", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

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

    future::try_join3(server1, server2, server3).await?;
    Ok(())
}
