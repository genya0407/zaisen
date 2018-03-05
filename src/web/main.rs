#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate zaisen;
extern crate chrono;

use rocket_contrib::{Template};
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Serialize)]
struct RecruitViewDTO {
    id: i32,
    entries: HashMap<i32, Vec<i32>>,
    start_at: DateTime<Local>,
    end_at: DateTime<Local>,
}

impl RecruitViewDTO {
    pub fn from(rdto: zaisen::usecase::RecruitDTO) -> Self {
        Self {
            id: rdto.id,
            entries: rdto.entries,
            start_at: rdto.start_at,
            end_at: rdto.end_at
        }
    }
}

#[get("/")]
fn index() -> Template {
    let rr = zaisen::repository::MockRecruitRepository {};
    let usecase = zaisen::usecase::ClosestRecruit { recruit_repository: rr };
    let recruit_dto = usecase.run().unwrap();
    Template::render("index", &RecruitViewDTO::from(recruit_dto))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}