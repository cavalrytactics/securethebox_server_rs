use securethebox_server_rs::controllers::terraform;

// NOTE
// ALL tests run in PARALLEL (built-in Rust)

#[test]
fn test_set_google_service() {
    assert_eq!(
        terraform::set_google_service("kubernetes".to_string()),
        true
    );
}

#[test]
fn test_set_terraform_path() {
    assert_eq!(
        terraform::set_terraform_path("./src/securethebox_terrform".to_string()),
        true
    );
}

#[test]
fn test_set_google_cloud_keyfile() {
    assert_eq!(
        terraform::set_google_cloud_keyfile("kubernetes".to_string()),
        true
    );
}

#[test]
fn test_remove_terraform_cache() {
    assert_eq!(terraform::remove_terraform_cache(), true);
}

#[test]
fn test_set_terraform_workspace_name() {
    assert_eq!(
        terraform::set_terraform_workspace_name("test".to_string()),
        true
    );
}

#[test]
fn test_set_kubernetes_cluster_name() {
    assert_eq!(
        terraform::set_kubernetes_cluster_name("cluster-name".to_string()),
        true
    );
}

#[test]
fn test_create_terraform_workspace() {
    assert_eq!(
        terraform::create_terraform_workspace("test".to_string()),
        true
    );
}

#[test]
fn test_select_terraform_workspace() {
    assert_eq!(
        terraform::select_terraform_workspace("test".to_string()),
        true
    );
}

#[test]
fn test_delete_terraform_workspace() {
    assert_eq!(
        terraform::delete_terraform_workspace("test".to_string()),
        true
    );
}

#[test]
fn test_init() {
    assert_eq!(
        terraform::init(
            "kubernetes".to_string(),
            "./src/securethebox_terraform".to_string()
        ),
        true
    );
}

#[test]
fn test_plan() {
    assert_eq!(
        terraform::plan(
            "kubernetes".to_string(),
            "./src/securethebox_terraform".to_string(),
            "test".to_string(),
            "cluster-name".to_string()
        ),
        true
    );
}

#[test]
#[ignore]
fn test_apply() {
    assert_eq!(
        terraform::apply(
            "kubernetes".to_string(),
            "./src/securethebox_terraform".to_string(),
            "test".to_string(),
            "cluster-name".to_string()
        ),
        true
    );
}

#[test]
#[ignore]
fn test_destroy() {
    assert_eq!(
        terraform::destroy(
            "kubernetes".to_string(),
            "./src/securethebox_terraform".to_string(),
            "test".to_string(),
            "cluster-name".to_string()
        ),
        true
    );
}
