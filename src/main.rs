use actix_web::{App, HttpResponse, HttpServer, dev::Server, middleware, web};
use std::net::TcpListener;

use awesome_blog::start;

pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
    let srv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(HttpResponse::Ok))
    })
    .listen(listener)?
    .run();

    Ok(srv)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:8080")?;
    start_blog(listener)?.await?;
    Ok(())
}
