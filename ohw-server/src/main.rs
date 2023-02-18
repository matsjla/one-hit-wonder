#![feature(decl_macro)]

use lambda_http::Error;
use rocket::http::RawStr;
use rocket_lamb::RocketExt;
#[macro_use] extern crate rocket;

#[get("/note/<id>")]
fn note_by_id(id: &RawStr) -> String {
    format!("Hello note {}", id.as_str())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    rocket::ignite()
        .mount("/api", routes![note_by_id])
        .lambda()
        .launch();
}
