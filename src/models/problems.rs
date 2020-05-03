use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::solutions::Solution;
use crate::models::submissions::Submission;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ProblemScopes {
    Scripting,
    IncidentResponse,
    IntrustionDetection,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Problem {
    pub node: NodeDetails,
    solutions: Vec<ID>,
    submissions: Vec<ID>,
    problem_scope: ProblemScopes,
    attempts: Option<i32>,
    instructions: String,
    points: Option<i32>,
    number: Option<i32>,
    start_date: String,
    due_date: String,
    reject_date: String,
}

impl Node for Problem {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A problem")]
impl Problem {
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

    fn attempts(&self) -> Option<i32> {
        match self.attempts {
            Some(attempts) => Some(attempts as i32),
            None => None,
        }
    }

    fn instructions(&self) -> String {
        self.instructions.to_owned()
    }

    fn points(&self) -> Option<i32> {
        match self.points {
            Some(points) => Some(points as i32),
            None => None,
        }
    }

    fn number(&self) -> Option<i32> {
        match self.number {
            Some(number) => Some(number as i32),
            None => None,
        }
    }

    fn start_date(&self) -> String {
        self.start_date.to_owned()
    }

    fn due_date(&self) -> String {
        self.due_date.to_owned()
    }
    fn reject_date(&self) -> String {
        self.reject_date.to_owned()
    }

    fn problem_scope(&self) -> ProblemScopes {
        self.problem_scope
    }

    fn solutions(&self, ctx: &Clients) -> Vec<Solution> {
        let service = &ctx.mongo.get_mongo_service("solutions").unwrap();
        let filter = doc! { "problem": self.node.id.to_string() };
        let result: Result<FindResult<Solution>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }

    fn submissions(&self, ctx: &Clients) -> Vec<Submission> {
        let service = &ctx.mongo.get_mongo_service("submissions").unwrap();
        let filter = doc! { "problem": self.node.id.to_string() };
        let result: Result<FindResult<Submission>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProblemConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Problem>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl ProblemConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Problem> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Problem>> for ProblemConnection {
    fn from(fr: FindResult<Problem>) -> ProblemConnection {
        ProblemConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewProblem {
    solutions: Option<Vec<juniper::ID>>,
    submissions: Option<Vec<juniper::ID>>,
    problem_scope: ProblemScopes,
    attempts: Option<i32>,
    instructions: String,
    points: Option<i32>,
    number: Option<i32>,
    start_date: String,
    due_date: String,
    reject_date: String,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateProblem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ProblemScopes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub submissions: Option<Vec<juniper::ID>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub solutions: Option<Vec<juniper::ID>>,
}
