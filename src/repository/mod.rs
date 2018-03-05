use chrono::prelude::*;
use model::*;
use std::collections::HashMap;

pub trait RecruitRepository {
    fn store(&self, recruit: Recruit);
    fn find(&self, recruit_id: RecruitId) -> Option<Recruit>;
    fn closest_recruit(&self) -> Option<Recruit>;
}

pub struct MockRecruitRepository {}

impl MockRecruitRepository {
    fn sample_recruit(&self) -> Recruit {
        let mut hashmap = HashMap::new();
        hashmap.insert(TaskId(1), vec![UserId(10)]);
        hashmap.insert(TaskId(2), vec![UserId(20), UserId(30)]);
        hashmap.insert(TaskId(3), vec![]);
        let entries = Entries(hashmap);

        return Recruit {
            id: RecruitId(1),
            entries: entries,
            start_at: Local.from_local_datetime(&NaiveDate::from_ymd(2018, 2,  5).and_hms(0, 0, 0)).unwrap(),
            end_at:   Local.from_local_datetime(&NaiveDate::from_ymd(2018, 2, 12).and_hms(0, 0, 0)).unwrap(),
        }
    }
}

impl RecruitRepository for MockRecruitRepository {
    fn find(&self, recruit_id: RecruitId) -> Option<Recruit> {
        let mut recruit = self.sample_recruit();
        recruit.id = recruit_id;
        return Some(recruit)
    }

    fn store(&self, _recruit: Recruit) {
        // do nothing.
    }

    fn closest_recruit(&self) -> Option<Recruit> {
        return Some(self.sample_recruit())
    }
}