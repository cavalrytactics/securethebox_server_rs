use bson::doc;
use chrono::{DateTime, Utc};
use log::warn;
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::configurations::Configuration;
use crate::models::dummies::Dummy;
use crate::models::problems::Problem;
use crate::models::vulnerabilities::Vulnerability;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ApplicationTypes {
    Database,
    Web,
    CICD,
    Authentication,
    GraphQL,
    Websockets,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Application {
    pub node: NodeDetails,
    name: String,
    version: String,
    configuration: Option<ID>,
    vulnerabilities: Vec<ID>,
    problems: Vec<ID>,
    dummies: Vec<ID>,
}

impl Node for Application {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A application")]
impl Application {
    pub fn id(&self) -> juniper::ID {
        self.node.id().into()
    }

    fn date_created(&self) -> DateTime<Utc> {
        self.node.date_created()
    }

    fn date_modified(&self) -> DateTime<Utc> {
        self.node.date_modified()
    }

    fn created_by(&self) -> juniper::ID {
        self.node.created_by_id().into()
    }

    fn updated_by(&self) -> juniper::ID {
        self.node.updated_by_id().into()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn version(&self) -> String {
        self.version.to_owned()
    }

    fn configuration(&self, ctx: &Clients) -> Option<Configuration> {
        match &self.configuration {
            None => None,
            Some(configuration_id) => {
                let service = &ctx.mongo.get_mongo_service("configurations").unwrap();
                let result: Result<Option<Configuration>, ServiceError> =
                    service.find_one_by_id(configuration_id.clone());
                match result {
                    Ok(configuration) => configuration,
                    Err(e) => {
                        warn!(
                            "unable to retrieve configuration by id {:?}",
                            configuration_id
                        );
                        None
                    }
                }
            }
        }
    }

    fn vulnerabilities(&self, ctx: &Clients) -> Vec<Vulnerability> {
        let service = &ctx.mongo.get_mongo_service("vulnerabilities").unwrap();
        let filter = doc! { "application": self.node.id.to_string() };
        let result: Result<FindResult<Vulnerability>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
    fn problems(&self, ctx: &Clients) -> Vec<Problem> {
        let service = &ctx.mongo.get_mongo_service("problems").unwrap();
        let filter = doc! { "application": self.node.id.to_string() };
        let result: Result<FindResult<Problem>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
    fn dummies(&self, ctx: &Clients) -> Vec<Dummy> {
        let service = &ctx.mongo.get_mongo_service("dummies").unwrap();
        let filter = doc! { "application": self.node.id.to_string() };
        let result: Result<FindResult<Dummy>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ApplicationConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Application>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl ApplicationConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Application> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Application>> for ApplicationConnection {
    fn from(fr: FindResult<Application>) -> ApplicationConnection {
        ApplicationConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewApplication {
    pub name: String,
    version: String,
    configuration: Option<juniper::ID>,
    vulnerabilities: Option<Vec<juniper::ID>>,
    problems: Option<Vec<juniper::ID>>,
    dummies: Option<Vec<juniper::ID>>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<juniper::ID>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<juniper::ID>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub problems: Option<Vec<juniper::ID>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dummies: Option<Vec<juniper::ID>>,
}
