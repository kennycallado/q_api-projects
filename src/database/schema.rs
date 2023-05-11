// @generated automatically by Diesel CLI.

diesel::table! {
    project_values (id) {
        id -> Int4,
        project_id -> Int4,
        values_id -> Int4,
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
    values (id) {
        id -> Int4,
        user_id -> Int4,
        value -> Jsonb,
    }
}

diesel::joinable!(project_values -> projects (project_id));
diesel::joinable!(project_values -> values (values_id));

diesel::allow_tables_to_appear_in_same_query!(
    project_values,
    projects,
    values,
);
