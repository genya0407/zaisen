use repository::*;
use model::*;

pub struct CreateEntry {
    pub recruit_repository: Box<RecruitRepository>,
}

impl CreateEntry {
    pub fn run(&self, recruit_id: i32, task_id: i32, user_id: i32) {
        let mut recruit = self.recruit_repository.find(RecruitId(recruit_id));
        recruit.add_entry(TaskId(task_id), UserId(user_id));
        self.recruit_repository.store(recruit);
    }
}