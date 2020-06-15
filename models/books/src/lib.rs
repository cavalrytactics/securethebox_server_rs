use async_graphql::{Context, FieldResult, Schema, SimpleBroker, ID};
use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use slab::Slab;
use std::sync::Arc;

pub type BooksSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Clone)]
pub struct Book {
    id: ID,
    name: String,
    author: String,
    points: String,
}

#[async_graphql::Object]
impl Book {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn author(&self) -> &str {
        &self.author
    }

    async fn points(&self) -> &str {
        &self.points
    }
}

//
// Create a Key/Value Store
// Similar to python Dict
//
pub type Storage = Arc<Mutex<Slab<Book>>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn books(&self, ctx: &Context<'_>) -> Vec<Book> {
        //
        // Database Init
        //
        let books = ctx.data::<Storage>().lock().await;
        //
        // Query Response
        //
        books.iter().map(|(_, book)| book).cloned().collect()
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_book(&self, ctx: &Context<'_>, name: String, author: String, points: String) -> ID {
        //
        // Database Init
        //
        let mut books = ctx.data::<Storage>().lock().await;
        let entry = books.vacant_entry();
        let id: ID = entry.key().into();
        //
        // Database Item
        //
        let book = Book {
            id: id.clone(),
            name: name.clone(),
            author: author.clone(),
            points: points.clone(),
        };
        //
        // Database Action
        //
        entry.insert(book);
        //
        // Query Response
        //
        SimpleBroker::publish(BookChanged {
            mutation_type: MutationType::Created,
            id: id.clone(),
            name: name.clone(),
            author: author.clone(),
            points: points.clone(),
        });
        //
        // Return Value
        //
        id
    }

    async fn update_book(&self, ctx: &Context<'_>, id: ID, name: String, author: String, points: String) -> FieldResult<bool> {
        let mut books = ctx.data::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        if books.contains(id) {
            //
            // Database Action
            //
            books[id] = Book {
                id: id.into(),
                name: name.clone(),
                author: author.clone(),
                points: points.clone(),
            };
            //
            // Query Response
            //
            SimpleBroker::publish(BookChanged {
                mutation_type: MutationType::Updated,
                id: id.into(),
                name: name.clone(),
                author: author.clone(),
                points: points.clone(),
            });
            //
            // Return Value
            //
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_book(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        //
        // Database Init
        //
        let mut books = ctx.data::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        if books.contains(id) {
            //
            // Database Action
            //
            books.remove(id);
            //
            // Query Response
            //
            SimpleBroker::publish(BookChanged {
                mutation_type: MutationType::Deleted,
                id: id.into(),
                name: "".to_string(),
                author: "".to_string(),
                points: "".to_string(),
            });
        //
        // Return Value
        //
            //
            // Return Value
            //
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
    Updated,
    Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct BookChanged {
    mutation_type: MutationType,
    id: ID,
    name: String,
    author: String,
    points: String,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn books(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = BookChanged> {
        //
        // Query Response
        //
        SimpleBroker::<BookChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
