

pub mod database {
    use rusqlite::Connection;

    #[derive(Debug, Clone, Copy)]
    pub struct Database<'a> {
        database_path: &'a str,
    }
    
    impl<'a> Database<'a> {
        /// Creates a new [`Database`].
        pub fn new(database_path: &'a str) -> Self {
            let db = Database {
                database_path,
            };
            db.create_table().unwrap();
            db
        }
        fn create_table(&self) -> rusqlite::Result<()> {   
            // Todo: move this to a separate function
            // Make main function small
    
            let conn = Connection::open("bashrc_manager.db")?;

            conn.execute(
                "create table if not exists bashrc (
                    id integer primary key,
                    name text not null unique,
                    created_at datetime default current_timestamp,
                    updated_at datetime default current_timestamp
                )"
                ,()
            )?;

            conn.execute(
                "create table if not exists bashrc_entry (
                    id integer primary key,
                    bashrc_id integer not null,
                    file_settings blob,
                    created_at datetime default current_timestamp,
                    updated_at datetime default current_timestamp,
                    foreign key (bashrc_id) references bashrc (id)
                )"
                ,()
            );
            Ok(())
        }
    }
}
