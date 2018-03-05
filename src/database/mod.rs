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

/*
pub trait RecruitRepository {
    fn store(&self, recruit: Recruit);
    fn find(&self, recruit_id: RecruitId) -> Recruit;
    fn find_all(&self) -> Vec<Recruit>;
    fn closest_recruit(&self) -> Option<Recruit>;
}
*/

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
        use self::schema::{recruit_tasks, entries};

        let task_users = recruit_tasks::table
                            .left_join(entries::table)
                            .select((recruit_tasks::task_id, entries::user_id.nullable()))
                            .filter(recruit_tasks::task_id.eq(recruit_id))
                            .load::<(i32, Option<i32>)>(self.connection)
                            .unwrap();

        let mut entry_content: HashMap<model::TaskId, Vec<model::UserId>> = HashMap::new();
        for (task_id, user_id_opt) in task_users {
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
}

//impl<'a> RecruitRepository for PsqlRecruitRepository<'a> {
impl<'a> PsqlRecruitRepository<'a> {
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
