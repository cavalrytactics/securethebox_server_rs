use chrono::{DateTime, Utc};
use mongodb_base_service::{Node, NodeDetails};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rank {
    pub node: NodeDetails,
    position: Option<i32>,
}

impl Node for Rank {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A rank")]
impl Rank {
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

    fn position(&self) -> Option<i32> {
        match self.position {
            Some(position) => Some(position as i32),
            None => None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RankConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Rank>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl RankConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Rank> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Rank>> for RankConnection {
    fn from(fr: FindResult<Rank>) -> RankConnection {
        RankConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewRank {
    position: Option<i32>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateRank {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}
