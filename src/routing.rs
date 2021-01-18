use actix_files::Files;
use actix_web::web;

use crate::handlers;

pub fn configuration() -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(|cfg: &mut web::ServiceConfig| {
        cfg.
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
                route(web::post().to(handlers::songs::update))
            ).
            service(
                web::resource("/songs/{id}/edit").
                route(web::get().to(handlers::songs::edit))
            ).
            service(
                web::resource("/songs/{id}/delete").
                route(web::post().to(handlers::songs::delete))
            );
    })
}
