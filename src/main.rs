use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use futures::future;
mod views;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn two() -> impl Responder {
    "Two reached - not!\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Running 2 Servers in parallel
    let server1 = HttpServer::new(move || {
        println!("worker here");
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .bind("127.0.0.1:8080")?
    .workers(3) // If not set -> using the amount of threads
    .run();

    let server2 = HttpServer::new(move || {
        println!("worker here");
        App::new().service(web::resource("/one").route(web::get().to(|| two())))
    })
    .bind("127.0.0.1:8081")?
    .workers(3) // If not set -> using the amount of threads
    .run();

    let server3 = HttpServer::new(move || {
        let app = App::new().configure(views::views_factory);
        return app
    })
        .bind("127.0.0.1:8000")?
        .workers(3) // If not set -> using the amount of threads
        .run();

    future::try_join3(server1, server2, server3).await?;
    Ok(())
}
