#[macro_use] extern crate rocket;
use std::{collections::HashMap};

use rocket::serde::{Serialize};
use rocket::serde::json::Json;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Event {
    athletes: Vec<Athlete>
}


#[derive(Serialize)]
#[derive(Clone)]
#[serde(crate = "rocket::serde")]
struct Athlete {
    name: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<event_name>/athletes")]
fn athletes(event_name: &str) -> Result<Json<Vec<Athlete>>, &str> {
    let mut events = HashMap::new();
    events.insert("ØL 2022", Event {
        athletes: vec![Athlete {name: "Jakob".to_owned()}]
    });

    let event = events.get(event_name);
    match event {
        Some(event) => Ok(Json(event.athletes.to_vec())),
        None => Err("404"),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![athletes])
}