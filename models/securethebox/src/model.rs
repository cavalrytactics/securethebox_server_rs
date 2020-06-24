use super::Securethebox;

use crate::shared;

use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use slab::Slab;
use std::sync::Arc;
use std::collections::HashMap;

use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, FieldResult, SimpleBroker, ID};

pub struct Challenge(usize);

#[async_graphql::Object]
impl Challenge {
    async fn id(&self, ctx: &Context<'_>) -> &str {
        ctx.data::<Securethebox>().challenges[self.0].id
    }

    async fn name(&self, ctx: &Context<'_>) -> &str {
        ctx.data::<Securethebox>().challenges[self.0].name
    }

    async fn problems(&self, ctx: &Context<'_>) -> Vec<Problem> {
        ctx.data::<Securethebox>().challenges[self.0]
            .problems
            .iter()
            .map(|id|Problem(*id).into())
            .collect()
    }
}


pub struct Problem(usize);

#[async_graphql::Object]
impl Problem {
    async fn id(&self, ctx: &Context<'_>) -> &str {
        ctx.data::<Securethebox>().problems[self.0].id
    }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn challenge( &self, ctx: &Context<'_>, #[arg(desc = "id of the challenge")] id: String,) -> Option<Challenge> {
        ctx.data::<Securethebox>().challenge(&id).map(Challenge)
    }

    async fn challenges( &self, ctx: &Context<'_>, after: Option<String>, before: Option<String>, first: Option<i32>, last: Option<i32>,) -> FieldResult<Connection<usize, Challenge, EmptyFields, EmptyFields>> {
        let challenges = ctx
            .data::<Securethebox>()
            .challenges()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        query_contests(after, before, first, last, &challenges)
            .await
            .map(|conn| conn.map_node(Challenge))
    }

    async fn problem( &self, ctx: &Context<'_>, #[arg(desc = "id of the problem")] id: String,) -> Option<Problem> {
        ctx.data::<Securethebox>().problem(&id).map(Problem)
    }

    async fn problems( &self, ctx: &Context<'_>, after: Option<String>, before: Option<String>, first: Option<i32>, last: Option<i32>,) -> FieldResult<Connection<usize,Problem, EmptyFields, EmptyFields>> {
        let problems = ctx
            .data::<Securethebox>()
            .problems()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        query_contests(after, before, first, last, &problems)
            .await
            .map(|conn| conn.map_node(Problem))
    }
}

// Common fields
#[async_graphql::Interface(
    field(name = "id", type = "&str", context),
)]
pub enum Contest {
    Challenge(Challenge),
    Problem(Problem),
}

async fn query_contests(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    contests: &[usize],
) -> FieldResult<Connection<usize, usize, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = contests.len();

            if let Some(after) = after {
                if after >= contests.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &contests[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < contests.len());
            connection.append(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, *item)),
            );
            Ok(connection)
        },
    )
    .await
}


pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_challenge( &self, ctx: &Context<'_>, #[arg(desc = "create challenge")] name: String) -> Option<Challenge> {
        // 
        // Database Init
        //
        let mut challenges = Slab::new();
        let new_challenge = challenges.insert( shared::SecuretheboxChallenge {
            id: "1008",
            name: "C8",
            problems: vec![],
        });


        ctx.data::<Securethebox>().challenge("1008").map(Challenge)
        
        // let mut challenges = ctx.data::<Securethebox>().challenges();
        // let entry = securethebox.vacant_entry();
        // // let id: ID = entry.key().into();
        // // //
        // // // Database Item
        // // //
        // let mut challenge = Slab::new();
        // let challenge1 = challenge.insert(
        //     shared::SecuretheboxChallenge {
        //         id: "100",
        //         name: "test",
        //         problems: vec![]
        //     }
        // );
        // let mut problem = Slab::new();
        // let problem1 = problem.insert(
        //     shared::SecuretheboxProblem {
        //         id: "101",
        //     }
        // );
        //
        // let mut challenge_data = HashMap::new();
        // challenge_data.insert("1000", challenge1);
        //
        // let mut problem_data = HashMap::new();
        // problem_data.insert("2000",problem1);
        //
        // let stb = Securethebox {
        //     challenges: challenge,
        //     problems: problem,
        //     problem_data,
        //     challenge_data,
        // };
        // //
        // // // Database Action
        // // //
        // entry.insert(stb);
        // // //
        // // // Query Response
        // // //
        // // SimpleBroker::publish(ChallengeChanged {
        // //     mutation_type: MutationType::Created,
        // //     id: id.clone(),
        // // });
        // // //
        // // // Return Value
        // // //
        // // id
        // "Hello".to_string()
    }
}

#[async_graphql::Enum]
enum MutationType {
    Created,
    Updated,
    Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct ChallengeChanged {
    mutation_type: MutationType,
    id: ID,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct ProblemChanged {
    mutation_type: MutationType,
    id: ID,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn challenges(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = ChallengeChanged> {
        //
        // Query Response
        //
        SimpleBroker::<ChallengeChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
    async fn problems(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = ProblemChanged> {
        //
        // Query Response
        //
        SimpleBroker::<ProblemChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}

