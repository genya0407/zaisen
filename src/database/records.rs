use chrono::prelude::*;

#[derive(Queryable)]
pub struct Recruit {
    pub id: i32,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}
#[derive(Insertable)]
pub struct NewRecruit {
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub recruit_id: i32,
    pub contribution: i32,
    pub title: String,
    pub description: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}
#[derive(Insertable)]
pub struct NewTask {
    pub recruit_id: i32,
    pub contribution: i32,
    pub title: String,
    pub description: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Assignment {
    pub id: i32,
    pub task_id: i32,
    pub user_id: i32,
}
#[derive(Insertable)]
pub struct NewAssignment {
    pub task_id: i32,
    pub user_id: i32,
}

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub task_id: i32,
    pub user_id: i32,
}
#[derive(Insertable)]
pub struct NewEntry {
    pub task_id: i32,
    pub user_id: i32,
}