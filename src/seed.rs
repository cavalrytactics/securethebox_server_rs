mod db;
mod models;
mod schema;

use bson::doc;
use dotenv::dotenv;
use mongodb_base_service::BaseService;
use std::sync::Arc;

use crate::db::Clients;
use crate::models::{
    Application, Cluster, Configuration, Course, Credential, Dummy, Job, Problem, Rank, Report,
    Service, Solution, Submission, Team, University, User, Vulnerability,
};
use crate::schema::{create_schema, Schema};

fn main() {
    std::env::set_var("RUST_LOG", "info,actix_web=warn");
    env_logger::init();
    dotenv().ok();

    let db_clients = Arc::new(Clients {
        mongo: db::mongo::connect(),
    });

    // drop the existing data
    let vulnerabilities_service = db_clients
        .mongo
        .get_mongo_service("vulnerabilities")
        .unwrap();
    let _ = vulnerabilities_service.data_source().drop(None);

    let pets_service = db_clients.mongo.get_mongo_service("pets").unwrap();
    let _ = pets_service.data_source().drop(None);

    // seed data
    let vulnerabilities = vec![
        {
            doc! {"scope": "application", "type": "injection" }
        },
        {
            doc! {"scope": "application", "type": "broken_authentication" }
        },
        {
            doc! { "scope": "infrastructure", "type": "misconfiguration"}
        },
    ];

    // add the owners
    let results: Vec<Vulnerability> = vulnerabilities_service
        .insert_many(vulnerabilities, None)
        .unwrap();
    let ids: Vec<String> = results.iter().map(|x| x.node.id.to_string()).collect();
    let pets = vec![
        {
            doc! { "name": "Fido", "pet_type": "Dog", "age": 10, "gender": "Male", "owner": &ids[0] }
        },
        {
            doc! { "name": "Cleo", "pet_type": "Cat", "age": 12, "gender": "Female", "owner": &ids[1] }
        },
        {
            doc! { "name": "Oreo", "pet_type": "Cat", "age": 2, "gender": "Female", "owner": &ids[2] }
        },
        {
            doc! { "name": "Milo", "pet_type": "Dog", "age": 10, "gender": "Male", "owner": &ids[3] }
        },
        {
            doc! { "name": "Squirt", "pet_type": "Fish", "age": 2, "gender": "Female", "owner": &ids[4] }
        },
        {
            doc! { "name": "Lurch", "pet_type": "Hamster", "age": 1, "gender": "Male", "owner": &ids[0] }
        },
        {
            doc! { "name": "Fonz", "pet_type": "Turtle", "age": 10, "gender": "Male", "owner": &ids[1] }
        },
        {
            doc! { "name": "Lucy", "pet_type": "Turtle", "age": 10, "gender": "Female", "owner": &ids[1] }
        },
    ];

    let _pets_results: Vec<Pet> = pets_service.insert_many(pets, None).unwrap();
    println!("Data inserted");

    // putting this here to prevent dead code check issues
    let _schema: Arc<Schema> = std::sync::Arc::new(create_schema());
}
