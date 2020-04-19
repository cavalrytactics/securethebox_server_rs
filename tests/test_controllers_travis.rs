use securethebox_server_rs::controllers::travis;

#[test]
fn test_set_file_name() {
    assert_eq!(travis::set_file_name("test"), true);
}

#[test]
fn test_set_current_directory() {
    assert_eq!(travis::set_current_directory(), true);
}

#[test]
fn test_tar_secrets_directory() {
    assert_eq!(travis::tar_secrets_directory(), true);
}

#[test]
fn test_encrypt_tar_secrets() {
    assert_eq!(travis::encrypt_tar_secrets(), true);
}

#[test]
fn test_add_openssl_cmd() {
    assert_eq!(travis::add_openssl_cmd(), true);
}