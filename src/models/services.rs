use crate::db::Clients;
use crate::models::applications::Application;
use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ServiceTypes {
    LoadBalancer,
    WebServer,
    Database,
    DNS,
    Logging,
    SIEM,
    WebApplicationFirewall,
    IntrusionDetectionSystem,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Service {
    pub node: NodeDetails,
    service_type: ServiceTypes,
    applications: Option<Vec<ID>>,
}

impl Node for Service {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A service")]
impl Service {
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

    fn service_type(&self) -> ServiceTypes {
        self.service_type
    }

    fn applications(&self, ctx: &Clients) -> Vec<Application> {
        let service = &ctx.mongo.get_mongo_service("applications").unwrap();
        let filter = doc! { "service": self.node.id.to_string() };
        let result: Result<FindResult<Application>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ServiceConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Service>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl ServiceConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Service> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Service>> for ServiceConnection {
    fn from(fr: FindResult<Service>) -> ServiceConnection {
        ServiceConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewService {
    service_type: ServiceTypes,
    applications: Option<Vec<juniper::ID>>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateService {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceTypes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<juniper::ID>>,
}
