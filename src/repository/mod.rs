use chrono::prelude::*;
use model::*;
use std::collections::HashMap;

pub trait RecruitRepository {
    fn store(recruit: Recruit);
    fn find(recruit_id: RecruitId) -> Recruit;
    fn find_all() -> Vec<Recruit>;
}

pub struct MockRecruitRepository {}

impl RecruitRepository for MockRecruitRepository {
    fn store(_recruit: Recruit) {
        // do nothing.
    }

    fn find(recruit_id: RecruitId) -> Recruit {
        let mut hashmap = HashMap::new();
        hashmap.insert(TaskId(1), vec![UserId(10)]);
        hashmap.insert(TaskId(2), vec![UserId(20), UserId(30)]);
        hashmap.insert(TaskId(3), vec![]);
        let entries = Entries(hashmap);

        return Recruit {
            id: recruit_id,
            entries: entries,
            start_at: Local.from_local_datetime(&NaiveDate::from_ymd(2018, 2,  5).and_hms(0, 0, 0)).unwrap(),
            end_at:   Local.from_local_datetime(&NaiveDate::from_ymd(2018, 2, 12).and_hms(0, 0, 0)).unwrap(),
        }
    }

    fn find_all() -> Vec<Recruit> {
        return vec![]
    }
}