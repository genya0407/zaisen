#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate zaisen;
extern crate chrono;

use rocket_contrib::{Template, Json};
use chrono::prelude::*;
use zaisen::*;
use zaisen::repository::*;

/*
use rocket::request::FromFormValue;
use rocket::http::RawStr;

struct FormDateTime(pub DateTime<Local>);

impl<'v> FromFormValue<'v> for FormDateTime {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<FormDateTime, &'v RawStr> {
        NaiveDateTime::parse_from_str(form_value, "%Y-%m-%dT%H:%M:%S")
            .map(|naive_datetime| FormDateTime(Local.from_local_datetime(&naive_datetime).unwrap()))
            .map_err(|_| form_value)
    }
}
*/

#[derive(Deserialize, Clone)]
struct TaskParams {
    pub contribution: i32,
    pub title: String,
    pub description: String,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
}

#[derive(Deserialize)]
struct RecruitTaskParams {
    pub title: String,
    pub description: String,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
    pub task_paramses: Vec<TaskParams>,
}

#[post("/recruits", data="<rt_params>")]
fn create_recruit(rt_params: Json<RecruitTaskParams>) -> &'static str {
    use usecase::create_recruit::*;

    let dto = CreateRecruitAndTaskDTO {
        title: rt_params.title.clone(),
        description: rt_params.description.clone(),
        start_at: rt_params.start_at.clone(),
        end_at: rt_params.end_at.clone(),
        task_dtos: rt_params.task_paramses.clone().into_iter().map(|task_params| {
            CreateTaskDTO {
                contribution: task_params.contribution,
                title: task_params.title,
                description: task_params.description,
                start_at: task_params.start_at,
                end_at: task_params.end_at,
            }
        }).collect(),
    };

    let recruit_repository = OnMemoryRecruitRepository::new();
    let task_repository = OnMemoryTaskRepository::new();
    let mut uc = CreateRecruitAndTask {
        recruit_repository: recruit_repository,
        task_repository: task_repository,
    };
    uc.run(dto);

    "hoge"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![create_recruit])
        .attach(Template::fairing())
        .launch();
}