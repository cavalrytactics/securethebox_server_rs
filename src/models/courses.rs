use chrono::{DateTime, Utc};
use log::warn;
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::clusters::Cluster;
use crate::models::reports::Report;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CourseCategories {
    Database,
    Web,
    CICD,
    Authentication,
    GraphQL,
    Websockets,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Course {
    pub node: NodeDetails,
    title: String,
    description: String,
    start_date: String,
    due_date: String,
    destroy_date: String,
    category: CourseCategories,
    cluster: Option<ID>,
    report: Option<ID>,
}

impl Node for Course {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A lovable course")]
impl Course {
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

    fn title(&self) -> String {
        self.title.to_owned()
    }

    fn description(&self) -> String {
        self.description.to_owned()
    }

    fn start_date(&self) -> String {
        self.start_date.to_owned()
    }

    fn due_date(&self) -> String {
        self.due_date.to_owned()
    }

    fn destroy_date(&self) -> String {
        self.destroy_date.to_owned()
    }

    fn category(&self) -> CourseCategories {
        self.category.to_owned()
    }

    fn cluster(&self, ctx: &Clients) -> Option<Cluster> {
        match &self.cluster {
            None => None,
            Some(cluster_id) => {
                let service = &ctx.mongo.get_mongo_service("cluster").unwrap();
                let result: Result<Option<Cluster>, ServiceError> =
                    service.find_one_by_id(cluster_id.clone());
                match result {
                    Ok(cluster) => cluster,
                    Err(e) => {
                        warn!("unable to retrieve cluster by id {:?}", cluster_id);
                        None
                    }
                }
            }
        }
    }
    fn report(&self, ctx: &Clients) -> Option<Report> {
        match &self.report {
            None => None,
            Some(report_id) => {
                let service = &ctx.mongo.get_mongo_service("reports").unwrap();
                let result: Result<Option<Report>, ServiceError> =
                    service.find_one_by_id(report_id.clone());
                match result {
                    Ok(report) => report,
                    Err(e) => {
                        warn!("unable to retrieve report by id {:?}", report_id);
                        None
                    }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CourseConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Course>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl CourseConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Course> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Course>> for CourseConnection {
    fn from(fr: FindResult<Course>) -> CourseConnection {
        CourseConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewCourse {
    pub name: String,
    title: String,
    description: String,
    start_date: String,
    due_date: String,
    destroy_date: String,
    category: CourseCategories,
    cluster: Option<juniper::ID>,
    report: Option<juniper::ID>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateCourse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CourseCategories>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<juniper::ID>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<juniper::ID>,
}
