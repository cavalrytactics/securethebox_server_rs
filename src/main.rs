mod controllers;

fn main() {
    controllers::travis::set_file_name("test");
    controllers::travis::set_current_directory();
}
