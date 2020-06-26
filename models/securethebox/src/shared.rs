use slab::Slab;
use std::collections::HashMap;
// use async_graphql::{Context, FieldResult, Schema, SimpleBroker, ID};

#[derive(Clone)]
pub struct SecuretheboxChallenge {
    pub id: &'static str,
    pub name: &'static str,
    pub problems: Vec<usize>,
}

#[derive(Clone)]
pub struct SecuretheboxProblem {
    pub id: &'static str,
}

#[derive(Clone)]
pub struct Securethebox {
    pub challenges: Slab<SecuretheboxChallenge>,
    pub problems: Slab<SecuretheboxProblem>,
    pub challenge_data: HashMap<&'static str, usize>,
    pub problem_data: HashMap<&'static str, usize>,
}

