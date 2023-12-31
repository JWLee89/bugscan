-- Your SQL goes here
create table if not exists shell_commands (
    id integer primary key not null,
    name text not null unique,
    shell_command text not null,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp
);