-- Your SQL goes here
create table if not exists bashrc (
    id integer primary key,
    name text not null unique,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp
);

create table if not exists bashrc_entry (
    id integer primary key,
    bashrc_id integer not null,
    file_settings blob,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp,
    foreign key (bashrc_id) references bashrc (id)
);
