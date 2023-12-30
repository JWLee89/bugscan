// @generated automatically by Diesel CLI.

diesel::table! {
    bashrc (id) {
        id -> Integer,
        name -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    bashrc_entry (id) {
        id -> Integer,
        bashrc_id -> Integer,
        file_settings -> Nullable<Binary>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(bashrc_entry -> bashrc (bashrc_id));

diesel::allow_tables_to_appear_in_same_query!(
    bashrc,
    bashrc_entry,
);
