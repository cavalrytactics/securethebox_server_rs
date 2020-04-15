use securethebox_server_rs::controllers::travis;

#[test]
fn test_set_file_name() {
    // travis::set_file_name()
    assert_eq!(travis::set_file_name("test"), true);
    // let mut c = travis:: {
    //     current_directory: std::env::temp_dir(),
    //     file_name: "".to_string()
    // };
    // let _ = c.set_file_name("");
    // let _ = c.set_current_directory();
    // println!("File Name: {}", c.file_name);
    // println!("Current Dir: {:?}", c.current_directory);
    // assert!(c.set_file_name("test"), )
}

#[test]
fn test_set_current_directory() {
    assert_eq!(travis::set_current_directory(), true);
}