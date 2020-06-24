mod model;
pub mod shared;
use async_graphql::Schema;
use slab::Slab;
use std::collections::HashMap;
pub use shared::Securethebox;

pub use model::{QueryRoot, MutationRoot, SubscriptionRoot};

pub type SecuretheboxSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

impl Securethebox {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {

        // Create a K/V temp database
        let mut challenges = Slab::new();
        let mut problems = Slab::new();

        // Create Challenge
        let challenge1 = challenges.insert( shared::SecuretheboxChallenge {
            id: "1000",
            name: "Challenge 1",
            problems: vec![],
        });

        let challenge2 = challenges.insert( shared::SecuretheboxChallenge {
            id: "1001",
            name: "Challenge 2",
            problems: vec![],
        });

        // Create Problem
        let problem1 = problems.insert ( shared::SecuretheboxProblem {
            id: "2000",
        });

        // Add problem to Challenge
        challenges[challenge1].problems = vec![problem1];
        challenges[challenge2].problems = vec![problem1];

        let mut challenge_data = HashMap::new();
        challenge_data.insert("1000", challenge1 );
        challenge_data.insert("1001", challenge2 );

        let mut problem_data = HashMap::new();
        problem_data.insert("2000", problem1 );

        Self {
            challenges,
            problems,
            challenge_data,
            problem_data,
        }
    }

    pub fn challenge(&self, id: &str) -> Option<usize> {
        self.challenge_data.get(id).cloned()
    }

    pub fn problem(&self, id: &str) -> Option<usize> {
        self.problem_data.get(id).cloned()
    }

    pub fn challenges(&self) -> Vec<usize> {
        self.challenge_data.values().cloned().collect()
    }

    pub fn problems(&self) -> Vec<usize> {
        self.problem_data.values().cloned().collect()
    }
}

