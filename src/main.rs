mod config;
mod db;
mod models;
mod scalars;
mod schema;
mod services;
mod utils;

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use serde_json::json;
use tower_http::cors::CorsLayer;

type AppSchema = Schema<schema::QueryRoot, schema::MutationRoot, EmptySubscription>;

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

async fn health_check() -> Response {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "be-toko-online-rust",
            "version": env!("CARGO_PKG_VERSION"),
        })),
    )
        .into_response()
}

async fn root() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Toko Online API</title>
            <style>
                body {
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    max-width: 800px;
                    margin: 50px auto;
                    padding: 20px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                }
                .container {
                    background: rgba(255, 255, 255, 0.1);
                    backdrop-filter: blur(10px);
                    border-radius: 20px;
                    padding: 40px;
                    box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
                }
                h1 { margin-top: 0; }
                a {
                    color: #fff;
                    text-decoration: none;
                    padding: 10px 20px;
                    background: rgba(255, 255, 255, 0.2);
                    border-radius: 8px;
                    display: inline-block;
                    margin: 10px 5px;
                    transition: all 0.3s;
                }
                a:hover {
                    background: rgba(255, 255, 255, 0.3);
                    transform: translateY(-2px);
                }
                .endpoints {
                    margin-top: 30px;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Toko Online API</h1>
                <p>Selamat datang di Toko Online GraphQL API!</p>
                
                <div class="endpoints">
                    <h3>Available Endpoints:</h3>
                    <div>
                        <a href="/graphql" target="_blank">GraphQL Playground</a>
                        <a href="/health" target="_blank">Health Check</a>
                    </div>
                </div>

                <div style="margin-top: 30px; font-size: 14px; opacity: 0.8;">
                    <p>Built with Rust + Axum + SeaORM + async-graphql</p>
                </div>
            </div>
        </body>
        </html>
        "#,
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env();
    config.validate()?;

    println!("Connecting to database...");
    let pool = db::create_pool(&config.database_url).await?;
    println!("Database connected!");

    println!("Building GraphQL schema...");
    let schema = Schema::build(
        schema::QueryRoot,
        schema::MutationRoot,
        EmptySubscription,
    )
    .data(pool.clone())
    .finish();
    println!("Schema built!");

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
        .layer(CorsLayer::permissive());

    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("\nServer started successfully!");
    println!("Address: http://{}", addr);
    println!("GraphQL Playground: http://{}/graphql", addr);
    println!("Health Check: http://{}/health", addr);
    println!("\nPress Ctrl+C to stop the server\n");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}