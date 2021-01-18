use actix_web::App;
use actix_web::HttpServer;
use actix_web::HttpResponse;
use actix_web::{guard, web, middleware::Logger};
use actix_session::CookieSession;
use env_logger::{self, Env};
use sqlx::PgPool;
use dotenv::dotenv;

use spotiferris::handlers;
use spotiferris::routing;

const SESSION_SECRET: &[u8; 32] = b"1236833f3cc3f6a1415ea89c6cd0acfa";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&database_url).
        await.
        unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    let db = web::Data::new(db);

    let server = HttpServer::new(move || {
        App::new().
            wrap(Logger::default()).
            wrap(CookieSession::private(SESSION_SECRET).secure(true)).
            configure(routing::configuration()).
            app_data(db.clone()).
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

    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);
    server.bind(addr)?.run().await
}
