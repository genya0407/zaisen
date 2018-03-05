table! {
    entries (id) {
        id -> Int4,
        recruit_task_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    recruit_tasks (id) {
        id -> Int4,
        recruit_id -> Int4,
        task_id -> Int4,
    }
}

table! {
    recruits (id) {
        id -> Int4,
        start_at -> Timestamp,
        end_at -> Timestamp,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        assignee -> Nullable<Int4>,
        contribution -> Int4,
        title -> Text,
        description -> Text,
        start_at -> Timestamp,
        end_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        room_id -> Nullable<Int4>,
        name -> Text,
        email -> Nullable<Text>,
        is_admin -> Bool,
    }
}

joinable!(entries -> recruit_tasks (recruit_task_id));
joinable!(entries -> users (user_id));
joinable!(recruit_tasks -> recruits (recruit_id));
joinable!(recruit_tasks -> tasks (task_id));
joinable!(tasks -> users (assignee));
joinable!(users -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    entries,
    recruit_tasks,
    recruits,
    rooms,
    tasks,
    users,
);
