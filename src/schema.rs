// @generated automatically by Diesel CLI.

diesel::table! {
    shell_commands (id) {
        id -> Integer,
        name -> Text,
        shell_command -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
