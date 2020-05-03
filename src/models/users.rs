use chrono::{DateTime, Utc};
use log::warn;
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::ranks::Rank;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub node: NodeDetails,
    email: String,
    rank: Option<ID>,
    logged_in: bool,
}

impl Node for User {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A user")]
impl User {
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

    fn email(&self) -> String {
        self.email.to_owned()
    }

    fn logged_in(&self) -> bool {
        self.logged_in.to_owned()
    }

    fn rank(&self, ctx: &Clients) -> Option<Rank> {
        match &self.rank {
            None => None,
            Some(rank_id) => {
                let service = &ctx.mongo.get_mongo_service("ranks").unwrap();
                let result: Result<Option<Rank>, ServiceError> =
                    service.find_one_by_id(rank_id.clone());
                match result {
                    Ok(rank) => rank,
                    Err(e) => {
                        warn!("unable to retrieve rank by id {:?}", rank_id);
                        None
                    }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<User>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl UserConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<User> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<User>> for UserConnection {
    fn from(fr: FindResult<User>) -> UserConnection {
        UserConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewUser {
    email: String,
    logged_in: bool,
    rank: Option<juniper::ID>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logged_in: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<juniper::ID>,
}
