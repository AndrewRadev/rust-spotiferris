use actix_files::Files;
use actix_session::CookieSession;
use actix_web::middleware::Logger;
use actix_web::{web, guard, App, HttpServer, HttpResponse};
use env_logger::{self, Env};

const SESSION_SECRET: &[u8; 32] = b"1236833f3cc3f6a1415ea89c6cd0acfa";

use spotiferris::handlers;

macro_rules! build_app {
    () => {
        App::new().
            wrap(Logger::default()).
            wrap(CookieSession::private(SESSION_SECRET).secure(true)).
            service(Files::new("/public", "public")).
            route("/", web::get().to(handlers::songs::index)).
            service(
                web::resource("/songs").
                route(web::get().to(handlers::songs::index)).
                route(web::post().to(handlers::songs::create))
            ).
            route("/songs/new", web::get().to(handlers::songs::new)).
            service(
                web::resource("/songs/{id}").
                route(web::get().to(handlers::songs::show)).
                route(web::put().to(handlers::songs::update)).
                route(web::delete().to(handlers::songs::delete))
            ).
            default_service(
                web::resource("").
                route(web::get().to(handlers::render_404)).
                route(
                    web::route().
                    guard(guard::Not(guard::Get())).
                    to(HttpResponse::MethodNotAllowed),
                ),
            )
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(|| build_app!());

    server.bind(addr)?.run().await
}
