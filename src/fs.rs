pub mod io {
    use std::{fs::File, io::{Read, Write}};

    /// Returns the home directory of the current user
    pub fn get_home() -> String {
        return std::env::var("HOME").unwrap();
    }

    /// Returns the default location of the bashrc file
    pub fn get_default_bashrc_location() -> String {
        let home = get_home();
        format!("{}/.bashrc", home)
    }

    pub struct BashrcFile<'a> {
        file_path: &'a str, 
        contents: Option<String>,
    }

    impl<'a> BashrcFile<'a> {
        pub fn new(file_path: &'a str) -> Self {
            BashrcFile {
                file_path: file_path,
                contents: None,
            }
        }

        pub fn read(&self) -> String {
            let mut file = File::open(self.file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        }

        pub fn write(&self, file_path: &str) {
            let mut source = File::open(self.file_path).unwrap();
            let mut target = File::create(file_path).unwrap();
            std::io::copy(&mut source, &mut target).unwrap();
        }
    }
}
