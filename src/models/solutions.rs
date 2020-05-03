use crate::db::Clients;
use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{Node, NodeDetails, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SolutionTypes {
    Flag,
    Script,
    UnitTest,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Solution {
    pub node: NodeDetails,
    solution_type: SolutionTypes,
    applications: Option<Vec<ID>>,
}

impl Node for Solution {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A solution")]
impl Solution {
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

    fn solution_type(&self) -> SolutionTypes {
        self.solution_type
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct SolutionConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Solution>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl SolutionConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Solution> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Solution>> for SolutionConnection {
    fn from(fr: FindResult<Solution>) -> SolutionConnection {
        SolutionConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewSolution {
    solution_type: SolutionTypes,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateSolution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_type: Option<SolutionTypes>,
}
