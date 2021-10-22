#[macro_use]
extern crate rocket;

use api;
use pages;

// Routes is generated automatically (via macro if possible, otherwise with build.rs or cli tool),
// this struct will have one method per route, the api routes will be prefixed with `api_`, the page
// routes not, also will have a method that returns all routes (used in register_routes).
// This struct will be useful also to use strong typed routes.

// It will generate at compile time (maybe can be moved to build.rs or a cli tool) a base mod called
// `i18n` that will have a hierarchy of mods and methods that will be used for translations, also will
// warn about unused/missing translations in some or all yml files.
// i18n_init!(['en']);

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
  rocket::build()
    .mount("/", pages::routes())
    .mount("/api", api::routes())
}
