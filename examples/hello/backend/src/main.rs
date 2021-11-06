#[macro_use]
extern crate rocket;

use api;
use pages;

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", pages::routes())
    .mount("/api", api::routes())
}
