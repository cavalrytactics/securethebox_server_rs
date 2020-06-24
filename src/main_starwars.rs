use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{guard, http, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};

// use books::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use starwars::{QueryRoot, StarWars, StarWarsSchema};

async fn index(schema: web::Data<StarWarsSchema>, req: GQLRequest) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

// async fn index_ws(
//     schema: web::Data<StarWarsSchema>,
//     req: HttpRequest,
//     payload: web::Payload,
// ) -> Result<HttpResponse> {
//     ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            // .service(
            //     web::resource("/")
            //         .guard(guard::Get())
            //         .guard(guard::Header("upgrade", "websocket"))
            //         .to(index_ws),
            // )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .wrap(Logger::new("REQUEST:\"%U\" STATUS: %s IP: %a"))
            .wrap(
                //
                // Default allow any origin
                //
                Cors::new()
                    //
                    // Filter origin
                    //
                    // .allowed_origin("http://localhost:8000")
                    // .allowed_methods(vec!["GET", "POST"])
                    // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    // .allowed_header(http::header::CONTENT_TYPE)
                    // .max_age(3600)
                    .finish(),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
