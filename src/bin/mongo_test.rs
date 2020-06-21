use securethebox_server_rs::controllers::mongo;
use async_std;

#[async_std::main]
async fn main() -> std::io::Result<()>{
    let mut c = mongo::MongoDatabase {
       app_name: "stb".to_string(),
       database_name: "stb".to_string(),
       collection_name: "books".to_string(),
    };
    // let con = c.mongo_connection().await;
    c.list_collection_names().await;
    c.create_collection_data(&"securethebox".to_string()).await;
    c.get_collection_data(&"securethebox".to_string()).await;

    Ok(())
}
