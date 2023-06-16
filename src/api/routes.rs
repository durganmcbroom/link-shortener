/*! `/routes` */

use crate::app::App;
use rocket::State;

#[get("/")]
pub async fn hello(app: &State<App>) -> String {
   format!("Hello, world!")
}

#[put("/shorten")]
pub async fn shorten(app: &State<App>) -> Result<(), ()> {
   app.shorten()
}