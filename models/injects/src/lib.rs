use async_graphql::{Context, FieldResult, Schema, SimpleBroker, ID};
use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use slab::Slab;
use std::sync::Arc;

pub type InjectsSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Clone)]
pub struct Inject {
    id: ID,
    title: String,
    visiblity: Vec<String>,
    points: u32,
    start_date: String,
    due_date: String,
    reject_date: String,
    time_remaining: u32,
}

#[async_graphql::Object]
impl Inject {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn visiblity(&self) -> &Vec<String> {
        &self.visiblity
    }
    async fn points(&self) -> &u32{
        &self.points
    }
    async fn start_date(&self) -> &String {
        &self.start_date
    }
    async fn due_date(&self) -> &String {
        &self.due_date
    }
    async fn reject_date(&self) -> &String {
        &self.due_date
    }
    async fn time_remaining(&self) -> &u32 {
        &self.time_remaining
    }
}

pub type Storage = Arc<Mutex<Slab<Inject>>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn injects(&self, ctx: &Context<'_>) -> Vec<Inject> {
        let injects = ctx.data::<Storage>().lock().await;
        injects.iter().map(|(_, inject)| inject).cloned().collect()
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_inject(&self, ctx: &Context<'_>, title: String, visiblity: Vec<String>, points: u32, start_date: String, due_date: String, reject_date: String, time_remaining: u32) -> ID {
        let mut injects = ctx.data::<Storage>().lock().await;
        let entry = injects.vacant_entry();
        let id: ID = entry.key().into();
        let inject = Inject {
            id: id.clone(),
            title: title.clone(),
            visiblity: visiblity.clone(),
            points: points.clone(),
            start_date: start_date.clone(),
            due_date: due_date.clone(),
            reject_date: reject_date.clone(),
            time_remaining: time_remaining.clone(),
        };
        entry.insert(inject);
        SimpleBroker::publish(InjectChanged {
            mutation_type: MutationType::Created,
            id: id.clone(),
            title: title.clone(),
            visiblity: visiblity.clone(),
            points: points.clone(),
            start_date: start_date.clone(),
            due_date: due_date.clone(),
            reject_date: reject_date.clone(),
            time_remaining: time_remaining.clone(),
        });
        id
    }

    async fn delete_inject(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let mut injects = ctx.data::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        if injects.contains(id) {
            injects.remove(id);
            SimpleBroker::publish(InjectChanged {
                mutation_type: MutationType::Deleted,
                id: id.into(),
                title: "".to_string(),
                visiblity: "".to_string(),
                points: "".to_string(),
                start_date: "".to_string(),
                due_date: "".to_string(),
                reject_date: "".to_string(),
                time_remaining: "".to_string(),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[async_graphql::Enum]
#[derive(Copy, Clone)]
enum MutationType {
    Created,
    Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct InjectChanged {
    mutation_type: MutationType,
    id: ID,
    name: String,
    title: String,
    visiblity: Vec<String>,
    points: u32,
    start_date: String,
    due_date: String,
    reject_date: String,
    time_remaining: u32,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn injects(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = InjectChanged> {
        SimpleBroker::<InjectChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
