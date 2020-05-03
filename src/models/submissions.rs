use crate::db::Clients;
use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{Node, NodeDetails};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SubmissionTypes {
    Flag,
    Script,
    UnitTest,
    Document,
}

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SubmissionVerdicts {
    Pass,
    Fail,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Submission {
    pub node: NodeDetails,
    author: String,
    submission_type: SubmissionTypes,
    submission_verdict: SubmissionVerdicts,
    creation_time: String,
    relative_time: String,
    points: Option<i32>,
    content: String,
}

impl Node for Submission {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A submission")]
impl Submission {
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
    fn creation_time(&self) -> String {
        self.creation_time.to_owned()
    }
    fn relative_time(&self) -> String {
        self.relative_time.to_owned()
    }
    fn points(&self) -> Option<i32> {
        match self.points {
            Some(points) => Some(points as i32),
            None => None,
        }
    }

    fn submission_type(&self) -> SubmissionTypes {
        self.submission_type
    }

    fn submission_verdict(&self) -> SubmissionVerdicts {
        self.submission_verdict
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct SubmissionConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Submission>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl SubmissionConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Submission> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Submission>> for SubmissionConnection {
    fn from(fr: FindResult<Submission>) -> SubmissionConnection {
        SubmissionConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewSubmission {
    submission_type: SubmissionTypes,
    submission_verdict: SubmissionVerdicts,
    author: String,
    points: Option<i32>,
    creation_time: String,
    relative_time: String,
    content: String,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateSubmission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_type: Option<SubmissionTypes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_verdict: Option<SubmissionVerdicts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
