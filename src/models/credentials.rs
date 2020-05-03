use chrono::{DateTime, Utc};
use mongodb_base_service::{Node, NodeDetails};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AuthenticationTypes {
    Password,
    PublicKey,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Credential {
    pub node: NodeDetails,
    username: String,
    authentication_type: AuthenticationTypes,
    secret: String,
}

impl Node for Credential {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A credential")]
impl Credential {
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

    fn username(&self) -> String {
        self.username.to_owned()
    }

    fn secret(&self) -> String {
        self.secret.to_owned()
    }

    fn authentication_type(&self) -> AuthenticationTypes {
        self.authentication_type
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CredentialConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Credential>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl CredentialConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Credential> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Credential>> for CredentialConnection {
    fn from(fr: FindResult<Credential>) -> CredentialConnection {
        CredentialConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewCredential {
    pub username: String,
    secret: String,
    authentication_type: AuthenticationTypes,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<AuthenticationTypes>,
}
