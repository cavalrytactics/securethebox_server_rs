use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::applications::Application;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum VulnerabilityTypes {
    Injection,
    Misconfiguration,
    Authentication,
}

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum VulnerabilityScopes {
    Infrastructure,
    Application,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub node: NodeDetails,
    vulnerability_scope: VulnerabilityScopes,
    vulnerability_type: VulnerabilityTypes,
    applications: Option<Vec<ID>>,
}

impl Node for Vulnerability {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A vulnerability")]
impl Vulnerability {
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

    fn vulnerability_type(&self) -> VulnerabilityTypes {
        self.vulnerability_type
    }

    fn vulnerability_scope(&self) -> VulnerabilityScopes {
        self.vulnerability_scope
    }

    fn applications(&self, ctx: &Clients) -> Vec<Application> {
        let service = &ctx.mongo.get_mongo_service("applications").unwrap();
        let filter = doc! { "vulnerability": self.node.id.to_string() };
        let result: Result<FindResult<Application>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VulnerabilityConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Vulnerability>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl VulnerabilityConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Vulnerability> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Vulnerability>> for VulnerabilityConnection {
    fn from(fr: FindResult<Vulnerability>) -> VulnerabilityConnection {
        VulnerabilityConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewVulnerability {
    vulnerability_type: VulnerabilityTypes,
    vulnerability_scope: VulnerabilityScopes,
    applications: Option<Vec<juniper::ID>>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateVulnerability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_type: Option<VulnerabilityTypes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_scope: Option<VulnerabilityScopes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<juniper::ID>>,
}
