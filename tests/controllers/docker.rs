use securethebox_server_rs::controllers::docker;

// NOTE
// ALL tests run in PARALLEL (built-in Rust)

#[test]
#[ignore]
fn test_build_image() {
    assert_eq!(docker::build_image(), true);
}
