use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::users::User;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum JobResponsibilies {
    Scripting,
    CodeReview,
    Policy,
}

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum JobQualifications {
    YearExp1,
    YearExp2,
    YearExp3,
    YearExp5,
    YearExp10,
    YearExp15,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    pub node: NodeDetails,
    description: String,
    minimum_rank: Option<i32>,
    job_responsibilies: JobResponsibilies,
    job_qualifications: JobQualifications,
    referral_link: String,
    recruiter_email: String,
    pay_range: String,
    qualified_users: Option<Vec<ID>>,
}

impl Node for Job {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A job")]
impl Job {
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

    fn description(&self) -> String {
        self.description.to_owned()
    }

    fn minimum_rank(&self) -> Option<i32> {
        match self.minimum_rank {
            Some(minimum_rank) => Some(minimum_rank as i32),
            None => None,
        }
    }

    fn job_responsibilites(&self) -> JobResponsibilies {
        self.job_responsibilies
    }

    fn job_qualifications(&self) -> JobQualifications {
        self.job_qualifications
    }

    fn referral_link(&self) -> String {
        self.referral_link.to_owned()
    }
    fn recruiter_email(&self) -> String {
        self.recruiter_email.to_owned()
    }
    fn pay_range(&self) -> String {
        self.pay_range.to_owned()
    }

    fn qualified_users(&self, ctx: &Clients) -> Vec<User> {
        let service = &ctx.mongo.get_mongo_service("users").unwrap();
        let filter = doc! { "jobs": self.node.id.to_string() };
        let result: Result<FindResult<User>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JobConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Job>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl JobConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Job> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Job>> for JobConnection {
    fn from(fr: FindResult<Job>) -> JobConnection {
        JobConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewJob {
    description: String,
    minimum_rank: i32,
    responsibilies: Vec<String>,
    qualifications: Vec<String>,
    referral_link: String,
    recruiter_email: String,
    pay_range: String,
    qualified_users: Option<Vec<juniper::ID>>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_rank: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibilies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifications: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recruiter_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualified_users: Option<Vec<juniper::ID>>,
}
