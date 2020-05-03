use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::users::User;

#[derive(Clone, Serialize, Deserialize)]
pub struct Team {
    pub node: NodeDetails,
    members: Option<Vec<ID>>,
}

impl Node for Team {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A team")]
impl Team {
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

    fn members(&self, ctx: &Clients) -> Vec<User> {
        let service = &ctx.mongo.get_mongo_service("users").unwrap();
        let filter = doc! { "team": self.node.id.to_string() };
        let result: Result<FindResult<User>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TeamConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Team>,
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

    fn items(&self) -> &Vec<Team> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Team>> for TeamConnection {
    fn from(fr: FindResult<Team>) -> TeamConnection {
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
    members: Option<Vec<juniper::ID>>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateTeam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<juniper::ID>>,
}
