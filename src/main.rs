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

type AppSchema = Schema<schema::QueryRoot, schema::MutationRoot, EmptySubscription>;

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// ✅ Apollo Sandbox dengan Dynamic Endpoint
async fn graphql_playground() -> impl IntoResponse {
    Html(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Apollo Sandbox - Toko Online API</title>
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
        <div id="loading">🚀 Loading Apollo Sandbox...</div>
        <div style="width: 100%; height: 100vh; display: none;" id='embedded-sandbox'></div>
        <script src="https://embeddable-sandbox.cdn.apollographql.com/_latest/embeddable-sandbox.umd.production.min.js"></script>
        <script>
          // ✅ Deteksi endpoint otomatis dari URL browser
          const currentUrl = window.location.origin;
          const graphqlEndpoint = currentUrl + '/graphql';
          
          console.log('🔗 Detected endpoint:', graphqlEndpoint);
          
          // Load Apollo Sandbox
          new window.EmbeddedSandbox({
            target: '#embedded-sandbox',
            initialEndpoint: graphqlEndpoint,
            includeCookies: true,
          });
          
          // Hide loading, show sandbox
          setTimeout(() => {
            document.getElementById('loading').style.display = 'none';
            document.getElementById('embedded-sandbox').style.display = 'block';
          }, 1000);
        </script>
    </body>
    </html>
    "#)
}

// 💡 ALTERNATIF: GraphQL Playground (Lebih Stable untuk Forwarding)
// async fn graphql_playground() -> impl IntoResponse {
//     Html(async_graphql::http::playground_source(
//         async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
//             .with_setting("editor.theme", "dark")
//             .with_setting("editor.autocompleteEnabled", false)
//             .with_setting("request.credentials", "include")
//     ))
// }

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
                .badge {
                    background: rgba(102, 126, 234, 0.3);
                    padding: 4px 12px;
                    border-radius: 12px;
                    font-size: 12px;
                    margin-left: 10px;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>🛒 Toko Online API</h1>
                <p>Selamat datang di Toko Online GraphQL API!</p>
                
                <div class="endpoints">
                    <h3>Available Endpoints:</h3>
                    <div>
                        <a href="/graphql" target="_blank">
                            🚀 Apollo Sandbox
                            <span class="badge">Auto-Detect URL!</span>
                        </a>
                        <a href="/health" target="_blank">💚 Health Check</a>
                    </div>
                </div>

                <div style="margin-top: 30px; font-size: 14px; opacity: 0.8;">
                    <p>Built with Rust 🦀 + Axum + SeaORM + async-graphql</p>
                    <p style="font-size: 12px; margin-top: 10px;">
                        Endpoint auto-detects based on your URL ✨
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

    println!("🔌 Connecting to database...");
    let pool = db::create_pool(&config.database_url).await?;
    println!("✅ Database connected!");

    println!("🏗️  Building GraphQL schema...");
    let schema = Schema::build(
        schema::QueryRoot,
        schema::MutationRoot,
        EmptySubscription,
    )
    .data(pool.clone())
    .enable_federation()
    .finish();
    println!("✅ Schema built!");

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

    println!("\n🚀 Server started successfully!");
    println!("📍 Local: http://{}", addr);
    println!("🌐 Forwarded: Check VSCode PORTS tab for public URL");
    println!("🎮 GraphQL Playground: /graphql (auto-detects URL)");
    println!("💚 Health Check: /health");
    println!("\n✋ Press Ctrl+C to stop the server\n");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}