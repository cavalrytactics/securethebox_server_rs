use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::services::Service;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ClusterStatus {
    Deploying,
    Available,
    Destroyed,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub node: NodeDetails,
    status: ClusterStatus,
    services: Option<ID>,
}

impl Node for Cluster {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A cluster")]
impl Cluster {
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

    fn status(&self) -> ClusterStatus {
        self.status.to_owned()
    }

    fn services(&self, ctx: &Clients) -> Vec<Service> {
        let service = &ctx.mongo.get_mongo_service("services").unwrap();
        let filter = doc! { "cluster": self.node.id.to_string() };
        let result: Result<FindResult<Service>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ClusterConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Cluster>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl ClusterConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Cluster> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Cluster>> for ClusterConnection {
    fn from(fr: FindResult<Cluster>) -> ClusterConnection {
        ClusterConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewCluster {
    services: Option<Vec<juniper::ID>>,
    status: ClusterStatus,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateCluster {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClusterStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<juniper::ID>>,
}
