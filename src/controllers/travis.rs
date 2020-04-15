use std::path::PathBuf;
use std::env;

pub struct Travis {
    current_directory: PathBuf,
    file_name: String
}

impl Travis {
    fn set_file_name(&mut self, file_name: &str){
        self.file_name = file_name.to_string();
    }
    fn set_current_directory(&mut self){
        self.current_directory = env::current_dir().unwrap();
    }
}

pub fn set_file_name(arg_file_name: &str) -> bool {
    let mut c = Travis {
        current_directory: env::temp_dir(),
        file_name: String::new()
    };
    c.set_file_name(arg_file_name);
    if c.file_name == arg_file_name {
        println!("File Name: {}", c.file_name);
        true
    } else {
        false
    }
}

pub fn set_current_directory() -> bool {
    let mut c = Travis {
        current_directory: env::temp_dir(),
        file_name: String::new() 
    };
    c.set_current_directory();
    if c.current_directory != env::temp_dir() {
        println!("Current Directory: {:?}", c.current_directory);
        true
    } else {
        false
    }
}