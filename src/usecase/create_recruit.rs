use model::*;
use repository::*;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CreateTaskDTO {
    pub contribution: i32,
    pub title: String,
    pub description: String,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
}

#[derive(Clone)]
pub struct CreateRecruitAndTaskDTO {
    pub title: String,
    pub description: String,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
    pub task_dtos: Vec<CreateTaskDTO>,
}

pub struct CreateRecruitAndTask<RR, TR> where RR: RecruitRepository, TR: TaskRepository {
    pub recruit_repository: RR,
    pub task_repository: TR,
}

impl<RR, TR> CreateRecruitAndTask<RR, TR> where RR: RecruitRepository, TR: TaskRepository {
    pub fn run(&mut self, dto: CreateRecruitAndTaskDTO) {
        let mut entries = HashMap::new();
        for task in self.task_dto2model(dto.clone().task_dtos) {
            let task = self.task_repository.store(task);
            entries.insert(task.id, vec![]);
        }

        let recruit = self.dto2model(dto, Entries(entries));
        println!("{:?}", recruit);
        self.recruit_repository.store(recruit);
    }

    fn task_dto2model(&self, dtos: Vec<CreateTaskDTO>) -> Vec<Task> {
        dtos.into_iter().map(|task_dto| {
            Task {
                id: TaskId(0),
                title: task_dto.title,
                description: task_dto.description,
                assignees: vec![],
                contribution: TaskContribution(task_dto.contribution),
                start_at: task_dto.start_at,
                end_at: task_dto.end_at,
            }
        }).collect()
    }

    fn dto2model(&self, dto: CreateRecruitAndTaskDTO, entries: Entries) -> Recruit {
        Recruit {
            id: RecruitId(0),
            title: dto.title,
            description: dto.description,
            entries: entries,
            start_at: dto.start_at,
            end_at: dto.end_at,
        }
    }
}