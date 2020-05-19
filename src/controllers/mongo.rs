use mongodb::{options::ClientOptions, Client};

#[derive(Clone)]
pub struct MainContext {}

pub async fn mongo_client() -> Client {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    client_options.app_name = Some("SecureTheBox".to_string());

    let client = Client::with_options(client_options).unwrap();
    return client;
}
