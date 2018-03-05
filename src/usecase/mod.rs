use chrono::prelude::*;
use repository::*;
use model::*;
use std::collections::HashMap;

pub struct CreateEntry<R: RecruitRepository> {
    pub recruit_repository: R,
}

impl<R> CreateEntry<R> where R: RecruitRepository {
    pub fn run(&self, recruit_id: i32, task_id: i32, user_id: i32) {
        let mut recruit = self.recruit_repository.find(RecruitId(recruit_id));
        recruit.add_entry(TaskId(task_id), UserId(user_id));
        self.recruit_repository.store(recruit);
    }
}

pub struct RecruitDTO {
    pub id: i32,
    pub entries: HashMap<i32, Vec<i32>>,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
}

pub struct ClosestRecruit<R: RecruitRepository> {
    pub recruit_repository: R,    
}

impl<R> ClosestRecruit<R> where R: RecruitRepository {
    pub fn run(&self) -> Option<RecruitDTO> {
        let recruit_option = self.recruit_repository.closest_recruit();
        return recruit_option.map(|r| self.recruit2dto(r))
    }

    fn recruit2dto(&self, recruit: Recruit) -> RecruitDTO {
        return RecruitDTO {
            id: recruit.id.0,
            entries: recruit.entries.0.into_iter()
                                    .map(|(task_id, user_ids)| {
                                        let ids = user_ids.into_iter().map(|user_id| user_id.0).collect();
                                        (task_id.0, ids)
                                    }).collect(),
            start_at: recruit.start_at,
            end_at: recruit.end_at,
        }        
    }
}