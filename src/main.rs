<<<<<<< HEAD
#![allow(unused_imports)]

mod app;
=======
mod app;
mod config;
>>>>>>> 7e9a26c (Initial commit)
mod database;

#[cfg(test)]
mod test;

<<<<<<< HEAD
#[cfg(feature = "db_diesel")]
extern crate openssl;
#[cfg(feature = "db_diesel")]
#[macro_use]
extern crate diesel;
#[cfg(feature = "db_diesel")]
#[macro_use]
extern crate diesel_migrations;
#[cfg(feature = "db_diesel")]
extern crate rocket_sync_db_pools;

#[macro_use]
extern crate rocket;
=======
extern crate openssl;

#[allow(unused_imports)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;
>>>>>>> 7e9a26c (Initial commit)

fn main() {
    app::server::main();
}
