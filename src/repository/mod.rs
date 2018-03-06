use chrono::prelude::*;
use model::*;
use std::collections::HashMap;



pub trait RecruitRepository {
    fn store(&mut self, recruit: Recruit);
    fn find(&self, recruit_id: RecruitId) -> Option<Recruit>;
}

pub struct OnMemoryRecruitRepository {
    pub recruits: HashMap<RecruitId, Recruit>,
    pub max_id: i32,
}

impl OnMemoryRecruitRepository {
    pub fn new() -> Self {
        Self { recruits: HashMap::new(), max_id: 0 }
    }
}

impl RecruitRepository for OnMemoryRecruitRepository {
    fn store(&mut self, mut recruit: Recruit) {
        if recruit.id.0 == 0 {
            recruit.id = RecruitId(self.max_id + 1);
            self.max_id += 1;
        }

        self.recruits.insert(recruit.id.clone(), recruit);
    }

    fn find(&self, recruit_id: RecruitId) -> Option<Recruit> {
        self.recruits.get(&recruit_id).map(|recruit| recruit.clone())
    }
}


pub trait TaskRepository {
    fn store(&mut self, task: Task) -> Task;
    fn find(&self, task_id: TaskId) -> Option<Task>;
}

pub struct OnMemoryTaskRepository {
    pub tasks: HashMap<TaskId, Task>,
    pub max_id: i32,
}

impl OnMemoryTaskRepository {
    pub fn new() -> Self {
        Self { tasks: HashMap::new(), max_id: 0 }
    }    
}

impl TaskRepository for OnMemoryTaskRepository {
    fn store(&mut self, mut task: Task) -> Task {
        if task.id.0 == 0 {
            task.id = TaskId(self.max_id + 1);
            self.max_id += 1;
        }

        let task_return = task.clone();
        self.tasks.insert(task.id.clone(), task);

        return task_return
    }

    fn find(&self, task_id: TaskId) -> Option<Task> {
        self.tasks.get(&task_id).map(|task| task.clone())
    }
}

