use actix_web::App;
use actix_web::HttpServer;
use actix_web::{web, middleware::Logger};
use actix_web::cookie::Key;
use actix_session::{
    SessionMiddleware,
    storage::CookieSessionStore,
    config::CookieContentSecurity,
};
use env_logger::{self, Env};
use sqlx::PgPool;
use dotenv::dotenv;

use spotiferris::handlers;
use spotiferris::routing;

const SESSION_SECRET: &[u8; 64] = b"1236833f3cc3f6a1415ea89c6cd0acfa1236833f3cc3f6a1415ea89c6cd0acfa";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&database_url).
        await.
        unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    let db = web::Data::new(db);

    let server = HttpServer::new(move || {
        let session_middleware =
            SessionMiddleware::builder(
                CookieSessionStore::default(),
                Key::from(SESSION_SECRET),
            ).
            cookie_name(String::from("hello-web")).
            cookie_secure(true).
            cookie_content_security(CookieContentSecurity::Private).
            build();

        App::new().
            wrap(Logger::default()).
            wrap(session_middleware).
            configure(routing::configuration()).
            app_data(db.clone()).
            default_service(web::get().to(handlers::render_404))
    });

    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);
    server.bind(addr)?.run().await
}
