table! {
    assignees (id) {
        id -> Int4,
        task_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    entries (id) {
        id -> Int4,
        task_id -> Int4,
        user_id -> Int4,
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
        recruit_id -> Int4,
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

joinable!(assignees -> tasks (task_id));
joinable!(assignees -> users (user_id));
joinable!(entries -> tasks (task_id));
joinable!(entries -> users (user_id));
joinable!(tasks -> recruits (recruit_id));
joinable!(users -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    assignees,
    entries,
    recruits,
    rooms,
    tasks,
    users,
);
