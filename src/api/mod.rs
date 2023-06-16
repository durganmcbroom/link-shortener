use rocket::{Build, Rocket};
use crate::app::App;

mod routes;
use routes::hello;

pub fn init() -> Rocket<Build> {
    rocket::build()
        .manage(App::default())
        .mount("/", routes![hello])
}