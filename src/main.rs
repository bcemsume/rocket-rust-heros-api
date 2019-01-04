#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};
mod hero;

#[post("/",data ="<hero>")]
fn create(hero: Json<hero::Hero>) -> Json<hero::Hero>{
    hero
}

#[get("/")]
fn read()-> Json<hero::Hero>{
    let hr = hero::Hero { age:15, hometown:String::from("mahmut"), id:0, identity:String::from("mahmut"), name:String::from("mahmut") }; 
    Json(hr)
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero:Json<hero::Hero>)-> Json<hero::Hero>{
    hero
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<JsonValue>{
    Json(json!({"status":"ok"}))
}

fn main() {
    rocket::ignite().mount("/hero", routes![create,update,delete])
    .mount("/heroes", routes![read]).launch();
}
