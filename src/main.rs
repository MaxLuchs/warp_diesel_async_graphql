use warp::hyper::Body;
use warp::*;
use warp_test_2::db;
use warp_test_2::graphql::graphql_context::GraphqlContext;
use std::convert::Infallible;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::http::playground_source;
use warp_test_2::graphql::graphql_schema::{Query, MySchema};
use warp_test_2::graphql::graphql_schema::schema;
use std::error::Error;
use async_graphql_warp::{graphql};
use hyper::{Response, StatusCode};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("Envs loaded");

    // Create Graphql-Context:
    let db_url = std::env::var("DATABASE_URL").expect("Env missing");
    let db = db::DB::new(&db_url).unwrap();
    let context = GraphqlContext::new(db);

    // Create graphql:
    let schema = schema(context);
    let graphql = async_graphql_warp::graphql(schema)
        .and_then(
            |(schema, request): (MySchema, async_graphql::Request)| async move
                {
                    Ok::<_, Infallible>(async_graphql_warp::Response::from(schema.execute(request).await))
                }
        );
    let graphql_filter = warp::path("graphql").and(warp::post().and(graphql));

    // Create playground:
    let playground = playground_source(GraphQLPlaygroundConfig::new("graphql"));
    let graphiql = warp::any().and(warp::path::end().and(warp::path::end().map(move ||
        hyper::Response::builder().header("Content-Type", "text/html").status(200).body(Body::from(playground.clone()))
    )));

    // serve:
    let result = warp::serve(warp::any().and(graphql_filter.or(graphiql))).run(([127 ,0,0,1], 8000)).await;
    return Ok(result);
}


