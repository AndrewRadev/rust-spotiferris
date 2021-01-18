use actix_web::App;
use actix_web::HttpServer;
use actix_web::HttpResponse;
use actix_web::{guard, web, middleware::Logger};
use actix_session::CookieSession;
use env_logger::{self, Env};

use spotiferris::handlers;
use spotiferris::routing;

const SESSION_SECRET: &[u8; 32] = b"1236833f3cc3f6a1415ea89c6cd0acfa";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(|| {
        App::new().
            wrap(Logger::default()).
            wrap(CookieSession::private(SESSION_SECRET).secure(true)).
            configure(routing::configuration()).
            default_service(
                web::resource("").
                route(web::get().to(handlers::render_404)).
                route(
                    web::route().
                    guard(guard::Not(guard::Get())).
                    to(HttpResponse::MethodNotAllowed),
                ),
            )
    });

    server.bind(addr)?.run().await
}
