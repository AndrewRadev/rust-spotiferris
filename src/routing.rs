use std::env;
use gotham::handler::assets::FileOptions;
use gotham::router::Router;
use gotham::router::builder::*;
use crate::handlers;

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get_or_head("/").to(handlers::songs::index);

        route.associate("/songs", |assoc| {
            assoc.get_or_head().to(handlers::songs::index);
            assoc.post().to(handlers::songs::create);
        });

        route.get("/songs/new").to(handlers::songs::new);

        route.associate("/songs/:id", |assoc| {
            assoc.get().
                with_path_extractor::<handlers::songs::SongPath>().
                to(handlers::songs::show);
            assoc.put().
                with_path_extractor::<handlers::songs::SongPath>().
                to(handlers::songs::update);
            assoc.patch().
                with_path_extractor::<handlers::songs::SongPath>().
                to(handlers::songs::update);
            assoc.delete().
                with_path_extractor::<handlers::songs::SongPath>().
                to(handlers::songs::delete);
        });

        route.scope("/api", |route| {
            // Won't be used for the moment, but will be interesting later
            route.get("/songs").to(handlers::api::songs::index);
        });

        let mut assets_path = env::current_dir().unwrap();
        assets_path.push("public");
        route.get("*").to_dir(
            FileOptions::new(&assets_path)
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build(),
        );
    })
}
