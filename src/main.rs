mod app;
mod api;
mod domain;

#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};


//noinspection RsMainFunctionNotFound
#[launch]
fn rocket() -> Rocket<Build> {
    api::init()
}