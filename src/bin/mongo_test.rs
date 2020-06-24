use securethebox_server_rs::controllers::mongo;
use async_std;
use mongodb::bson::doc;

#[async_std::main]
async fn main() -> std::io::Result<()>{
    let mut c = mongo::MongoDatabase {
        host: "mongodb://localhost:27017".to_string(),
        app_name: "securethebox".to_string(),
        database_name: "securethebox".to_string(),
        collection_name: "contests".to_string(),
    };
    
    // c.list_collection_names().await;
    // c.create_collection_data(&"securethebox".to_string()).await;
    let json_docs = c.collection_get_all().await;

    let first_doc = &json_docs[0]["_id"]["$oid"];

    // println!("first_doc {:#}",first_doc);

    // let json_doc = c.collection_get_nodes_by_object_id(first_doc.to_string()).await;
    // println!("json_doc {:#}",json_doc);

    // c.collection_update_node_with_object_id(first_doc.to_string()).await;
    let data_doc = doc! { "author": "Snoopie" };
    c.collection_update_node_with_object_id(first_doc.to_string(), data_doc).await;
    c.collection_delete_node_with_object_id(first_doc.to_string()).await;
    Ok(())
}
