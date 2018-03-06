use chrono::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct RoomId(pub i32);
#[derive(Clone, Debug)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct UserId(pub i32);
#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: Option<String>,
    pub room: RoomId,
    pub is_admin: bool
}



#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct TaskId(pub i32);
#[derive(Clone, Debug)]
pub struct TaskContribution(pub i32);
#[derive(Clone, Debug)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub assignees: Vec<UserId>,
    pub contribution: TaskContribution,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
}



#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct RecruitId(pub i32);
#[derive(Clone, Debug)]
pub struct Entries(pub HashMap<TaskId, Vec<UserId>>);
#[derive(Clone, Debug)]
pub struct Recruit {
    pub id: RecruitId,
    pub title: String,
    pub description: String,
    pub entries: Entries,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>
}
impl Recruit {
    pub fn add_entry(&mut self, task_id: TaskId, user_id: UserId) {
        self.entries.0
            .entry(task_id)
            .and_modify(|user_ids| user_ids.push(user_id.clone()));
    }
}

