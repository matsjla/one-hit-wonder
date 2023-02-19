#![feature(proc_macro_hygiene)]
#![feature(async_fn_in_trait)]
#![feature(decl_macro)]

use lambda_http::Error;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::uuid::Uuid;
use rocket_lamb::RocketExt;
use serde::Serialize;

pub mod note;
pub mod postgres;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct CreateNoteDto {
    content: String,
    confidential: bool,
}

#[get("/<id>")]
fn api_get_note_by_id(id: Uuid) -> String {
    format!("Hello note {}", id.into_inner())
}

#[post("/", format = "json", data = "<message>")]
fn api_create_note(message: Json<CreateNoteDto>) -> JsonValue {
    let id = uuid::Uuid::new_v4();
    json!({
        "content": message.0.content,
        "id": id.to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    rocket::ignite()
        .mount("/api/notes", routes![api_get_note_by_id, api_create_note])
        .lambda()
        .launch()
}
