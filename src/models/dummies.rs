use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{Node, NodeDetails};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DummyIntents {
    User,
    Shopper,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Dummy {
    pub node: NodeDetails,
    active: bool,
    intent: DummyIntents,
}

impl Node for Dummy {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A dummy")]
impl Dummy {
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

    fn intent(&self) -> DummyIntents {
        self.intent.to_owned()
    }

    fn active(&self) -> bool {
        self.active.to_owned()
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct DummyConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Dummy>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl DummyConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Dummy> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Dummy>> for DummyConnection {
    fn from(fr: FindResult<Dummy>) -> DummyConnection {
        DummyConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewDummy {
    pub intent: DummyIntents,
    active: bool,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateDummy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<DummyIntents>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
