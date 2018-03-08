use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use repository::*;
use model;
use std::collections::HashMap;
use chrono::prelude::*;

pub mod schema;
pub mod records;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    return connection
}

pub struct PsqlRecruitRepository<'a> {
    connection: &'a PgConnection
}

impl<'a> PsqlRecruitRepository<'a> {
    fn fetch_recruit_record(&self, recruit_id: i32) -> Option<records::Recruit> {
        use self::schema::recruits::dsl::*;

        recruits.find(recruit_id)
            .load::<records::Recruit>(self.connection)
            .unwrap()
            .pop()
    }

    fn fetch_entries(&self, recruit_id: i32) -> model::Entries {
        use self::schema::{tasks, entries};

        let task_user_ids = tasks::table
                            .left_join(entries::table)
                            .select((tasks::id, entries::user_id.nullable()))
                            .filter(tasks::recruit_id.eq(recruit_id))
                            .load::<(i32, Option<i32>)>(self.connection)
                            .unwrap();

        let mut entry_content: HashMap<model::TaskId, Vec<model::UserId>> = HashMap::new();
        for (task_id, user_id_opt) in task_user_ids {
            entry_content
                .entry(model::TaskId(task_id))
                .and_modify(|user_ids| {
                    if let Some(user_id) = user_id_opt {
                        user_ids.push(model::UserId(user_id));
                    }
                })
                .or_insert(
                    if let Some(user_id) = user_id_opt {
                        vec![model::UserId(user_id)]
                    } else {
                        vec![]
                    }
                );
        }

        return model::Entries(entry_content)
    }

    fn insert(&self, recruit: model::Recruit) {
        let recruit_record = records::NewRecruit {
            start_at: recruit.start_at.naive_utc(),
            end_at: recruit.end_at.naive_utc(),
        };

        let mut entry_records = vec![];
        for (task_id, user_ids) in recruit.entries {
            for user_id in user_ids {
                let entry_record = records::NewEntry {
                    task_id: task_id.0,
                    user_id: user_id.0
                };
                entry_records.push(entry_record);
            }
        }
    }
}

impl<'a> RecruitRepository for PsqlRecruitRepository<'a> {
    fn store(&self, recruit: model::Recruit) {
        if recruit.id.0 == 0 {
            self.insert(recruit);
        } else {
            self.update(recruit)
        }
    }

    fn find(&self, recruit_id: model::RecruitId) -> Option<model::Recruit> {
        let recruit_opt = self.fetch_recruit_record(recruit_id.0);
        recruit_opt.map(|recruit| {
            let entries = self.fetch_entries(recruit_id.0);
            model::Recruit {
                id: model::RecruitId(recruit.id),
                entries: entries,
                start_at: Local.from_utc_datetime(&recruit.start_at),
                end_at: Local.from_utc_datetime(&recruit.end_at),
            }
        })
    }
}
