use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::jobs::Job;

#[derive(Clone, Serialize, Deserialize)]
pub struct Company {
    pub node: NodeDetails,
    jobs: Vec<ID>,
}

impl Node for Company {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A company")]
impl Company {
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

    fn jobs(&self, ctx: &Clients) -> Vec<Job> {
        let service = &ctx.mongo.get_mongo_service("jobs").unwrap();
        let filter = doc! { "companies": self.node.id.to_string() };
        let result: Result<FindResult<Job>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompanyConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Company>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl CompanyConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Company> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Company>> for CompanyConnection {
    fn from(fr: FindResult<Company>) -> CompanyConnection {
        CompanyConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewCompany {
    jobs: Option<Vec<juniper::ID>>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateCompany {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<juniper::ID>>,
}
