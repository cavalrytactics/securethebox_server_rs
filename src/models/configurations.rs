use bson::doc;
use chrono::{DateTime, Utc};
use log::warn;
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::credentials::Credential;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ConfigurationTypes {
    Database,
    Web,
    CICD,
    Authentication,
    GraphQL,
    Websockets,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub node: NodeDetails,
    port: i32,
    url: String,
    credential: Option<ID>,
}

impl Node for Configuration {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A configuration")]
impl Configuration {
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

    fn port(&self) -> i32 {
        self.port.to_owned()
    }

    fn url(&self) -> String {
        self.url.to_owned()
    }

    fn credential(&self, ctx: &Clients) -> Option<Credential> {
        match &self.credential {
            None => None,
            Some(credential_id) => {
                let service = &ctx.mongo.get_mongo_service("credentials").unwrap();
                let result: Result<Option<Credential>, ServiceError> =
                    service.find_one_by_id(credential_id.clone());
                match result {
                    Ok(credential) => credential,
                    Err(e) => {
                        warn!("unable to retrieve credential by id {:?}", credential_id);
                        None
                    }
                }
            }
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigurationConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Configuration>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl ConfigurationConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Configuration> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Configuration>> for ConfigurationConnection {
    fn from(fr: FindResult<Configuration>) -> ConfigurationConnection {
        ConfigurationConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewConfiguration {
    pub port: Option<i32>,
    url: String,
    credential: Option<juniper::ID>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<juniper::ID>,
}
