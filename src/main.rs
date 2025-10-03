mod config;
mod db;
mod models;
mod scalars;
mod schema;
mod services;
mod utils;

use axum::{
    extract::Extension,
    http::{StatusCode, Method},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};
use std::env;

type AppSchema = Schema<schema::QueryRoot, schema::MutationRoot, EmptySubscription>;

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Apollo Sandbox - Toko Online NUVELLA</title>
        <style>
            body {
                margin: 0;
                padding: 0;
                overflow: hidden;
            }
            #loading {
                position: fixed;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                font-family: Arial, sans-serif;
                color: #666;
            }
        </style>
    </head>
    <body>
        <div id="loading">Loading Apollo Sandbox...</div>
        <div style="width: 100%; height: 100vh; display: none;" id='embedded-sandbox'></div>
        <script src="https://embeddable-sandbox.cdn.apollographql.com/_latest/embeddable-sandbox.umd.production.min.js"></script>
        <script>
          const currentUrl = window.location.origin;
          const graphqlEndpoint = currentUrl + '/graphql';
          
          console.log('Detected endpoint:', graphqlEndpoint);
          
          new window.EmbeddedSandbox({
            target: '#embedded-sandbox',
            initialEndpoint: graphqlEndpoint,
            includeCookies: true,
          });
          
          setTimeout(() => {
            document.getElementById('loading').style.display = 'none';
            document.getElementById('embedded-sandbox').style.display = 'block';
          }, 1000);
        </script>
    </body>
    </html>
    "#)
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
            <title>be-toko-online-nuvella</title>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <style>
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                
                body {
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                    min-height: 100vh;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    background: #0f172a;
                    padding: 20px;
                }
                
                .container {
                    max-width: 600px;
                    width: 100%;
                    background: #1e293b;
                    border-radius: 24px;
                    padding: 48px;
                    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
                    border: 1px solid rgba(255, 255, 255, 0.1);
                }
                
                .header {
                    text-align: center;
                    margin-bottom: 40px;
                }
                
                .header h1 {
                    font-size: 36px;
                    font-weight: 700;
                    color: #ffffff;
                    margin-bottom: 12px;
                    letter-spacing: -0.5px;
                }
                
                .header p {
                    font-size: 16px;
                    color: #94a3b8;
                    font-weight: 400;
                }
                
                .endpoints {
                    margin: 32px 0;
                }
                
                .endpoints h3 {
                    font-size: 14px;
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                    color: #64748b;
                    margin-bottom: 16px;
                    font-weight: 600;
                }
                
                .endpoint-grid {
                    display: grid;
                    gap: 12px;
                }
                
                .endpoint-link {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    padding: 16px 20px;
                    background: #334155;
                    border: 1px solid rgba(255, 255, 255, 0.1);
                    border-radius: 12px;
                    text-decoration: none;
                    color: #ffffff;
                    font-weight: 500;
                    transition: all 0.2s ease;
                }
                
                .endpoint-link:hover {
                    background: #3b4a61;
                    border-color: rgba(99, 102, 241, 0.5);
                    transform: translateY(-2px);
                    box-shadow: 0 8px 20px rgba(99, 102, 241, 0.15);
                }
                
                .endpoint-icon {
                    font-size: 20px;
                }
                
                .endpoint-text {
                    flex: 1;
                }
                
                .badge {
                    background: rgba(99, 102, 241, 0.2);
                    color: #a5b4fc;
                    padding: 4px 10px;
                    border-radius: 6px;
                    font-size: 11px;
                    font-weight: 600;
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                }
                
                .footer {
                    margin-top: 40px;
                    padding-top: 32px;
                    border-top: 1px solid rgba(255, 255, 255, 0.1);
                    text-align: center;
                }
                
                .footer p {
                    color: #64748b;
                    font-size: 14px;
                    line-height: 1.6;
                }
                
                .tech-stack {
                    display: inline-flex;
                    align-items: center;
                    gap: 6px;
                    color: #94a3b8;
                    font-weight: 500;
                }
                
                .status-indicator {
                    display: inline-flex;
                    align-items: center;
                    gap: 8px;
                    margin-top: 12px;
                    padding: 8px 16px;
                    background: rgba(16, 185, 129, 0.1);
                    border: 1px solid rgba(16, 185, 129, 0.2);
                    border-radius: 8px;
                    font-size: 13px;
                    color: #6ee7b7;
                }
                
                .status-dot {
                    width: 8px;
                    height: 8px;
                    background: #10b981;
                    border-radius: 50%;
                    animation: pulse 2s ease-in-out infinite;
                }
                
                @keyframes pulse {
                    0%, 100% { opacity: 1; }
                    50% { opacity: 0.5; }
                }
                
                @media (max-width: 640px) {
                    .container {
                        padding: 32px 24px;
                    }
                    
                    .header h1 {
                        font-size: 28px;
                    }
                }
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>Toko Online NUVELLA</h1>
                    <p>GraphQL API untuk sistem e-commerce modern</p>
                    <div class="status-indicator">
                        <span class="status-dot"></span>
                        <span>Server Online</span>
                    </div>
                </div>
                
                <div class="endpoints">
                    <h3>Available Endpoints</h3>
                    <div class="endpoint-grid">
                        <a href="/graphql" class="endpoint-link" target="_blank">
                            <span class="endpoint-icon">-</span>
                            <span class="endpoint-text">Apollo Sandbox</span>
                            <span class="badge">Auto-Detect</span>
                        </a>
                        <a href="/health" class="endpoint-link" target="_blank">
                            <span class="endpoint-icon">-</span>
                            <span class="endpoint-text">Health Check</span>
                        </a>
                    </div>
                </div>

                <div class="footer">
                    <p class="tech-stack">
                        Built with Rust + Axum + SeaORM + async-graphql
                    </p>
                    <p style="margin-top: 8px; font-size: 12px;">
                        Endpoint auto-detects based on your URL
                    </p>
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

    // Check command seeder
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "seed" {
        println!("\nRunning database seeder...\n");
        db::seed_all(&pool).await?;
        println!("\nSeeding complete!\n");
        return Ok(());
    }

    println!("Building GraphQL schema...");
    let schema = Schema::build(
        schema::QueryRoot,
        schema::MutationRoot,
        EmptySubscription,
    )
    .data(pool.clone())
    .enable_federation()
    .finish();
    println!("Schema built!");

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers(Any)
        );

    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("\nServer started successfully!");
    println!("Local: http://{}", addr);
    println!("GraphQL: /graphql");
    println!("Health: /health");
    println!("\nTo run seeder: cargo run seed");
    println!("Press Ctrl+C to stop\n");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}