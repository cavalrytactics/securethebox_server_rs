use mongodb::{options::ClientOptions, Client};
use std::io::prelude::*;
use std::io;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};

#[derive(Clone, Debug)]
pub struct MongoDatabase {
    pub app_name: String,
    pub database_name: String,
    pub collection_name: String,
}

impl MongoDatabase {
    pub fn new(self){

    }

    pub fn set_app_name(&mut self, app_name: &String) {
        self.app_name = app_name.to_string()
    }

    pub fn set_database_name(&mut self, database_name: &String) {
        self.app_name = database_name.to_string()
    }

    pub fn set_collection_name(&mut self, collection_name: &String) {
        self.app_name = collection_name.to_string()
    }

    pub async fn mongo_connection(&mut self) -> Client {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();
        client_options.app_name = Some("SecureTheBox".to_string());
        let client = Client::with_options(client_options).unwrap();
        client
    }


    pub async fn list_collection_names(&mut self) -> Vec<String> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();

        client_options.app_name = Some("SecureTheBox".to_string());

        let client = Client::with_options(client_options).unwrap();
        let database_connection = client.database("securethebox");
        database_connection.list_collection_names(None).await.unwrap()
    }

    pub async fn create_collection(&mut self, connection_name: &String) -> bool {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();

        client_options.app_name = Some("SecureTheBox".to_string());

        let client = Client::with_options(client_options).unwrap();
        let database_connection = client.database("securethebox");
        let _ = database_connection.create_collection(&connection_name.to_string(), None).await;
        true
        // database_connection.list_collection_names(None).await.unwrap()
    }

    pub async fn create_collection_data(&mut self, collection_name: &String) -> bool {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();

        client_options.app_name = Some("SecureTheBox".to_string());

        let client = Client::with_options(client_options).unwrap();
        let database_connection = client.database("securethebox");
        let database_collection = database_connection.collection(collection_name);
        let docs = vec![
            doc! { "title": "1984", "author": "George Orwell" },
            doc! { "title": "Animal Farm", "author": "George Orwell" },
            doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
        ];
        let _ = database_collection.insert_many(docs, None).await;
        true
    }
    pub async fn get_collection_data(&mut self, collection_name: &String) -> Result<(),mongodb::error::Error>{
        let filter = doc! { "author": "George Orwell" };
        let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();

        client_options.app_name = Some("SecureTheBox".to_string());

        let client = Client::with_options(client_options).unwrap();
        let db = client.database("securethebox");
        let collection = db.collection("contests");
        let mut cursor = collection.find(filter, find_options).await.unwrap();
        // println!("{:#?}", cursor.unwrap());

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    if let Some(title) = document.get("title").and_then(Bson::as_str) {
                        println!("title: {}", title);
                    }  else {
                        println!("no title found");
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }
}


