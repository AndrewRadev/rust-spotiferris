# Spotiferris

A music management app that uses [Actix](https://actix.rs/) for a web server, [SQLx](https://github.com/launchbadge/sqlx) for the database, and [Askama](https://github.com/djc/askama) for templating.

Created as a learning experiment, not intended for any serious use.

## Installation

Install sqlx:

``` .sh-session
$ cargo install sqlx-cli --no-default-features --features rustls,postgres
```

Set up the postgres databases:

``` .sh-session
$ cargo sqlx database setup
$ DATABASE_URL=$TEST_DATABASE_URL cargo sqlx database setup
```

Tests should now pass, but they need to run in one thread -- execute with `./run_test`. Launch development server with `./server` (port 7000), which uses [cargo-watch](https://github.com/watchexec/cargo-watch).
