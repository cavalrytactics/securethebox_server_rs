use securethebox_server_rs::controllers::mongo;

// NOTE
// ALL tests run in PARALLEL (built-in Rust)
//
#[test]
fn test_set_host() {
    assert_eq!(
        mongo::set_host(
            "mongodb://localhost:27017".to_string()),
        true
    );
}

#[test]
fn test_set_app_name() {
    assert_eq!(
        mongo::set_app_name(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string()),
        true
    );
}

#[test]
fn test_set_database_name() {
    assert_eq!(
        mongo::set_database_name(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string()),
        true
    );
}

#[test]
fn test_set_collection_name() {
    assert_eq!(
        mongo::set_collection_name(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string()),
        true
    );
}

#[async_std::test]
async fn test_mongo_client() {
    assert_eq!(
        mongo::mongo_client(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string()).await,
        true
    );
}

#[async_std::test]
async fn test_mongo_database() {

    assert_eq!(
        mongo::mongo_database(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string()).await,
        true
    );
}

#[async_std::test]
async fn test_mongo_collection() {
    assert_eq!(
        mongo::mongo_collection(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string()).await,
        true
    );
}

#[async_std::test]
async fn test_collection_create() {
    assert_eq!(
        mongo::collection_create(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string()).await,
        true
    );
}

#[async_std::test]
async fn test_collection_list_names() {
    assert_eq!(
        mongo::collection_list_names(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string()).await,
        true
    );
}

#[async_std::test]
async fn test_collection_insert_one() {
    let doc = mongodb::bson::doc!{"test_key":"test_value"};
    assert_eq!(
        mongo::collection_insert_one(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string(),
            doc).await,
        true
    );
}

#[async_std::test]
async fn test_collection_insert_many() {
    let docs = vec![mongodb::bson::doc!{"test_key":"test_value"}];
    assert_eq!(
        mongo::collection_insert_many(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string(),
            docs).await,
        true
    );
}

#[async_std::test]
async fn test_collection_get_all() {
    assert_eq!(
        mongo::collection_get_all(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string()).await,
        true
    );
}

#[async_std::test]
async fn test_collection_get_node_by_object_id() {
    let mut c = mongo::MongoDatabase {
        host: "mongodb://localhost:27017".to_string(),
        app_name: "securethebox".to_string(),
        database_name: "securethebox".to_string(),
        collection_name: "contests".to_string(),
    };
    let json_docs = c.collection_get_all().await;
    let object_id = &json_docs[0]["_id"]["$oid"];
    assert_eq!(
        mongo::collection_get_node_by_object_id(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string(),
            object_id.to_string()).await,
        true
    );
}

#[async_std::test]
async fn test_collection_get_node_by_key_value() {
    let doc = mongodb::bson::doc!{"test_key":"test_value"};
    assert_eq!(
        mongo::collection_get_nodes_by_key_value(
            "mongodb://localhost:27017".to_string(),
            "securethebox".to_string(),
            "securethebox".to_string(),
            "contests".to_string(),
            doc).await,
        true
    );
}

