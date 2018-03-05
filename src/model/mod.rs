use chrono::prelude::*;
use std::collections::HashMap;

pub struct RoomId {
    pub value: i32,
}

pub struct Room {
    pub id: RoomId,
    pub name: String,
}

pub struct UserId {
    pub value: i32,
}



pub struct User {
    pub id: UserId,
    pub name: String,
    pub room: RoomId,
    pub is_admin: bool
}



pub struct TaskId {
    pub value: i32,
}

pub struct TaskContribution {
    pub value: i32,
}

pub struct Task {
    pub id: TaskId,
    pub assignee: Option<UserId>,
    pub contribution: TaskContribution,
    pub title: String,
    pub body: String,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
}



pub struct RecruitTable {
    pub entries: Entries,
    pub recruit_start_at: DateTime<Local>,
    pub recruit_end_at: DateTime<Local>
}

pub struct Entries(HashMap<TaskId, Vec<UserId>>);

