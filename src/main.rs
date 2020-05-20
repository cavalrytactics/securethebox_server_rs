use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{guard, http, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;

use async_graphql::http::{playground_source, GQLResponse};
use async_graphql::Schema;
use async_graphql_actix_web::{GQLRequest, WSSubscription};

use books::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};

async fn index(schema: web::Data<BooksSchema>, gql_request: GQLRequest) -> web::Json<GQLResponse> {
    web::Json(GQLResponse(gql_request.into_inner().execute(&schema).await))
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", Some("/"))))
}

async fn index_ws(
    schema: web::Data<BooksSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .wrap(Logger::new("REQUEST:\"%U\" STATUS: %s"))
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8000")
                    .allowed_origin("http://localhost:7000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
