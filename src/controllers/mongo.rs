use mongodb::options::{ClientOptions, FindOptions};
use mongodb::{Client, Database, Collection};
use mongodb::bson::{doc, oid::ObjectId, Bson, Document};
use futures::stream::StreamExt;
use serde_json::value::Value;

#[derive(Clone, Debug)]
pub struct MongoDatabase {
    pub host: String,
    pub app_name: String,
    pub database_name: String,
    pub collection_name: String,
}

impl MongoDatabase {

    pub fn set_host(&mut self, host: &String) {
        self.host = host.to_string()
    }

    pub fn set_app_name(&mut self, app_name: &String) {
        self.app_name = app_name.to_string()
    }

    pub fn set_database_name(&mut self, database_name: &String) {
        self.database_name = database_name.to_string()
    }

    pub fn set_collection_name(&mut self, collection_name: &String) {
        self.collection_name = collection_name.to_string()
    }

    /// create mongodb client
    ///
    /// # Example
    ///
    /// ```rust
    /// use securethebox_server_rs::controllers::mongo;
    /// use async_std;
    ///
    /// #[async_std::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let c = mongo::MongoDatabase {
    ///         host: "mongodb://localhost:27017".to_string(),
    ///         app_name: "securethebox".to_string(),
    ///         database_name: "securethebox".to_string(),
    ///         collection_name: "contests".to_string(),
    ///     }
    ///
    ///     let client = c.mongo_client().await;
    ///
    ///     Ok(())
    /// ```
    ///
    pub async fn mongo_client(&mut self) -> Client {
        let mut client_options = ClientOptions::parse(&self.host.as_str())
            .await
            .unwrap();

        client_options.app_name = Some(self.app_name.to_string());

        let client = Client::with_options(client_options).unwrap();

        //return
        client
    }

    pub async fn mongo_database(&mut self) -> Database {
        let client = self.mongo_client().await;
        let database = client.database(&self.database_name);

        // return
        database
    }

    pub async fn mongo_collection(&mut self) -> Collection {

        let database = self.mongo_database().await;
        let collection = database.collection(&self.collection_name);

        // return
        collection
    }

    pub async fn collection_list_names(&mut self) -> Vec<String> {

        self.mongo_database()
            .await
            .list_collection_names(None).await.unwrap()
    }

    pub async fn collection_create(&mut self) -> bool {
        let _ = self.mongo_database()
            .await
            .create_collection(&self.collection_name, None)
            .await;
        true
    }

    /// insert one document into collection
    ///
    /// # Example
    ///
    /// ```rust
    /// use securethebox_server_rs::controllers::mongo;
    /// use mongodb::bson::doc;
    /// use async_std;
    ///
    /// #[async_std::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let c = mongo::MongoDatabase {
    ///         host: "mongodb://localhost:27017".to_string(),
    ///         app_name: "securethebox".to_string(),
    ///         database_name: "securethebox".to_string(),
    ///         collection_name: "contests".to_string(),
    ///     }
    ///
    ///     let doc = doc! { "title": "1984", "author": "George Orwell" }
    ///
    ///     let _ = self.collection_insert_one(doc)
    ///         .await;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn collection_insert_one(&mut self, doc: Document) -> bool {
        let _ = self.mongo_collection()
            .await
            .insert_one(doc, None)
            .await;
        true
    }

    /// insert a vector of documents into collection
    ///
    /// # Example
    ///
    /// ```rust
    /// use securethebox_server_rs::controllers::mongo;
    /// use mongodb::bson::doc;
    /// use async_std;
    ///
    /// #[async_std::main]
    /// async fn main() -> std::io::Result<()> {
    ///
    ///     let c = mongo::MongoDatabase {
    ///         host: "mongodb://localhost:27017".to_string(),
    ///         app_name: "securethebox".to_string(),
    ///         database_name: "securethebox".to_string(),
    ///         collection_name: "contests".to_string(),
    ///     }
    ///
    ///     let docs = vec![
    ///         doc! { "title": "1984", "author": "George Orwell" },
    ///         doc! { "title": "Animal Farm", "author": "George Orwell" },
    ///         doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ///     ];
    ///
    ///     let _ = self.collection_insert_many(docs)
    ///         .await
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn collection_insert_many(&mut self, docs: Vec<Document>) -> bool {
        let _ = self.mongo_collection()
            .await
            .insert_many(docs, None)
            .await;
        true
    }

    /// get all documents in collection as a JSON
    ///
    /// # Example
    ///
    /// ```rust
    /// use securethebox_server_rs::controllers::mongo;
    /// use async_std;
    ///
    /// #[async_std::main]
    /// async fn main() -> std::io::Result<()>{
    ///
    ///     let mut c = mongo::MongoDatabase {
    ///         host: "mongodb://localhost:27017".to_string(),
    ///         app_name: "securethebox".to_string(),
    ///         database_name: "securethebox".to_string(),
    ///         collection_name: "contests".to_string(),
    ///     };
    ///
    ///     let json_docs = c.collection_get_all().await;
    ///     println!("{}",json_docs[0]["key"]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn collection_get_all(&mut self) -> Value {
        let collection = self.mongo_collection().await;
        // set filter to None to get all values in collection
        let mut cursor = collection.find(None, None).await.unwrap();

        // create a vector to store documents
        let mut vec_docs = Vec::new();

        // iterate results
        while let Some(result) = cursor.next().await {
            match result {
                // if valid document
                Ok(document) => {
                    // push value to vector
                    vec_docs.push(document)
                }
                Err(e) => {
                    println!("Error adding document {}", e);
                }
            }
        }

        // convert vector to json
        let json_docs = serde_json::json!(vec_docs);

        // return
        json_docs
    }


    /// get all data that have a same { "key" : "value" }
    ///
    /// # Example
    ///
    /// ```rust
    /// use securethebox_server_rs::controllers::mongo;
    /// use async_std;
    /// use mongodb::bson::doc;
    ///
    /// #[async_std::main]
    /// async fn main() -> std::io::Result<()>{
    ///
    ///     let mut c = mongo::MongoDatabase {
    ///         host: "mongodb://localhost:27017".to_string(),
    ///         app_name: "securethebox".to_string(),
    ///         database_name: "securethebox".to_string(),
    ///         collection_name: "contests".to_string(),
    ///     };
    ///
    ///     let json_doc = c.collection_get_nodes_by_key_value(doc! { "author": "George Orwell"} ).await;
    ///     println!("{}",json_doc);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn collection_get_nodes_by_key_value(&mut self, filter_doc: Document) -> Vec<Value> {
        // filter by { "key": "value" }
        let filter = filter_doc;

        // add filter options: sort, skip, limit, batch_size
        let find_options = FindOptions::builder().sort(None).build();

        let collection = self.mongo_collection().await;

        // define cursor and add filter and options
        let mut cursor = collection.find(filter, find_options).await.unwrap();

        // create a new string
        let mut found_docs = Vec::new();
        // let mut found_doc = String::new();

        // iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    found_docs.push(
                        serde_json::from_str(
                            serde_json::to_string(&document)
                            .unwrap()
                            .as_str())
                        .unwrap());
                    // found_doc = serde_json::to_string(&document).unwrap();
                }
                Err(e) => {
                    println!("Error adding document {}", e);
                }
            }
        }

        // convert vector to json
        // let json_doc = serde_json::from_str(found_docs.as_str()).unwrap();

        // return
        // json_doc
        found_docs
    }

    /// get data from object_id
    ///
    /// # Example
    ///
    /// ```rust
    /// use securethebox_server_rs::controllers::mongo;
    /// use async_std;
    /// use mongodb::bson::doc;
    ///
    /// #[async_std::main]
    /// async fn main() -> std::io::Result<()>{
    ///
    ///     let mut c = mongo::MongoDatabase {
    ///         host: "mongodb://localhost:27017".to_string(),
    ///         app_name: "securethebox".to_string(),
    ///         database_name: "securethebox".to_string(),
    ///         collection_name: "contests".to_string(),
    ///     };
    ///
    ///     let object_id = "5b44ab692254d2749c47c209".to_string();
    ///     let json_doc = c.collection_get_node_by_object_id(&object_id).await;
    ///     println!("{}",json_doc);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn collection_get_node_by_object_id(&mut self, object_id: String) -> Value {
        let collection = self.mongo_collection().await;

        // convert String to object_id and remove quotes
        let oid_res = ObjectId::with_string(&object_id.replace("\"",""));

        // define cursor and add filter and options
        let cursor = collection.find_one(doc! { "_id": Bson::ObjectId(oid_res.unwrap()) }, None).await;

        // convert cursor response to json
        let json_doc = serde_json::json!(cursor.unwrap());

        // return
        json_doc
    }

    /// update node with object_id
    ///
    /// # Example
    ///
    /// ```rust
    /// use securethebox_server_rs::controllers::mongo;
    /// use async_std;
    /// use mongodb::bson::doc;
    ///
    /// #[async_std::main]
    /// async fn main() -> std::io::Result<()>{
    ///
    ///     let mut c = mongo::MongoDatabase {
    ///         host: "mongodb://localhost:27017".to_string(),
    ///         app_name: "securethebox".to_string(),
    ///         database_name: "securethebox".to_string(),
    ///         collection_name: "contests".to_string(),
    ///     };
    ///
    ///     let object_id = "5eed851800f7cac600e268fe".to_string();
    ///     let data_doc = doc! { "author": "Bob" };
    ///     let json_doc = c.collection_update_node_with_object_id(object_id, data_doc).await;
    ///     println!("{}",json_doc);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn collection_update_node_with_object_id(&mut self, object_id: String, data_doc: Document) -> bool {
        let collection = self.mongo_collection().await;

        // convert String to object_id and remove quotes
        let oid_res = ObjectId::with_string(&object_id.replace("\"",""));

        // define cursor and add filter and options
        let _ = collection.update_one(doc! { "_id": Bson::ObjectId(oid_res.unwrap()) }, data_doc, None).await;

        // return
        true
    }

    /// delete node with object_id
    ///
    /// # Example
    ///
    /// ```rust
    /// use securethebox_server_rs::controllers::mongo;
    /// use async_std;
    /// use mongodb::bson::doc;
    ///
    /// #[async_std::main]
    /// async fn main() -> std::io::Result<()>{
    ///
    ///     let mut c = mongo::MongoDatabase {
    ///         host: "mongodb://localhost:27017".to_string(),
    ///         app_name: "securethebox".to_string(),
    ///         database_name: "securethebox".to_string(),
    ///         collection_name: "contests".to_string(),
    ///     };
    ///
    ///     let object_id = "5eed851800f7cac600e268fe".to_string();
    ///     let data_doc = doc! { "author": "Bob" };
    ///     let json_doc = c.collection_update_node_with_object_id(object_id, data_doc).await;
    ///     println!("{}",json_doc);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn collection_delete_node_with_object_id(&mut self, object_id: String) -> bool {
        let collection = self.mongo_collection().await;

        // convert String to object_id and remove quotes
        let oid_res = ObjectId::with_string(&object_id.replace("\"",""));

        // define cursor and add filter and options
        let _ = collection.delete_one(doc! { "_id": Bson::ObjectId(oid_res.unwrap()) }, None).await;

        // return
        true
    }
}

pub fn set_host(arg_host: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    if c.host == arg_host{
        true
    } else {
        false
    }
}

pub fn set_app_name(arg_host: String, arg_app_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    if c.app_name == arg_app_name {
        true
    } else {
        false
    }
}

pub fn set_database_name(arg_host: String, arg_app_name: String, arg_database_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    if c.database_name == arg_database_name {
        true
    } else {
        false
    }
}

pub fn set_collection_name(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    if c.collection_name == arg_collection_name {
        true
    } else {
        false
    }
}

pub async fn mongo_client(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let client = c.mongo_client().await;
    let databases = client.list_databases(None, None).await;
    if databases.unwrap().len() > 0 {
        true
    } else {
        false
    }
}

pub async fn mongo_database(arg_host: String,arg_app_name: String, arg_database_name: String,arg_collection_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let database = c.mongo_database().await;
    let collections = database.list_collection_names(None).await;
    if collections.unwrap().len() > 0 {
        true
    } else {
        false
    }
}

pub async fn mongo_collection(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let collection = c.mongo_collection().await;
    let documents = collection.count_documents(None, None).await;
    if documents.unwrap() > 0 {
        true
    } else {
        false
    }
}


pub async fn collection_create(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let collection = c.collection_create().await;
    if collection == true {
        true
    } else {
        false
    }
}


pub async fn collection_list_names(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let collection_list = c.collection_list_names().await;
    if collection_list.len() > 0 {
        true
    } else {
        false
    }
}

pub async fn collection_insert_one(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String, arg_document: Document) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let insert_one = c.collection_insert_one(arg_document).await;
    if insert_one == true {
        true
    } else {
        false
    }
}

pub async fn collection_insert_many(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String, arg_documents: Vec<Document>) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let insert_many = c.collection_insert_many(arg_documents).await;
    if insert_many == true {
        true
    } else {
        false
    }
}

pub async fn collection_get_all(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let documents = c.collection_get_all().await;
    if documents.as_array().unwrap().len() > 0 {
        true
    } else {
        false
    }
}

pub async fn collection_get_node_by_object_id(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String,arg_object_id: String) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let document = c.collection_get_node_by_object_id(arg_object_id).await;
    if document["_id"] != serde_json::Value::Null {
        true
    } else {
        false
    }
}

pub async fn collection_get_nodes_by_key_value(arg_host: String, arg_app_name: String, arg_database_name: String,arg_collection_name: String,arg_document: Document) -> bool {
    let mut c = MongoDatabase {
        host: String::new(),
        app_name: String::new(),
        database_name: String::new(),
        collection_name: String::new(),
    };
    c.set_host(&arg_host);
    c.set_app_name(&arg_app_name);
    c.set_database_name(&arg_database_name);
    c.set_collection_name(&arg_collection_name);
    let documents = c.collection_get_nodes_by_key_value(arg_document.clone()).await;
    if documents.len() > 0 {
        true
    } else {
        false
    }
}

