// @generated automatically by Diesel CLI.

diesel::table! {
    project_records (id) {
        id -> Int4,
        project_id -> Int4,
        record_id -> Int4,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        keys -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    records (id) {
        id -> Int4,
        user_id -> Int4,
        record -> Jsonb,
    }
}

diesel::joinable!(project_records -> projects (project_id));
diesel::joinable!(project_records -> records (record_id));

diesel::allow_tables_to_appear_in_same_query!(project_records, projects, records,);
