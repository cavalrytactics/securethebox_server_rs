use chrono::{DateTime, Utc};
use mongodb_base_service::{Node, NodeDetails};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;

#[derive(Clone, Serialize, Deserialize)]
pub struct University {
    pub node: NodeDetails,
    name: String,
}

impl Node for University {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A university")]
impl University {
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

    fn name(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TeamConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<University>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl TeamConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<University> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<University>> for TeamConnection {
    fn from(fr: FindResult<University>) -> TeamConnection {
        TeamConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewTeam {
    name: String,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateTeam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
