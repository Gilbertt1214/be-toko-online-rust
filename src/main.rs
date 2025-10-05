mod config;
mod db;
mod handlers;
mod models;
mod scalars;
mod schema;
mod services;
mod utils;
mod graphql;



use axum::{
    extract::Extension,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use sea_orm::DatabaseConnection;
use serde_json::json;
use std::{env, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

// ==================== TYPE ALIASES ====================

type AppSchema = Schema<schema::QueryRoot, schema::MutationRoot, EmptySubscription>;

// ==================== GRAPHQL HANDLERS ====================

/// GraphQL request handler
async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Apollo Sandbox playground untuk testing GraphQL
async fn graphql_playground() -> impl IntoResponse {
    Html(include_str!("templates/playground.html"))
}

// ==================== API ENDPOINTS ====================

/// Health check endpoint untuk monitoring
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

/// Root endpoint dengan informasi API
async fn root() -> Html<&'static str> {
    Html(include_str!("templates/index.html"))
}

// ==================== INITIALIZATION FUNCTIONS ====================

/// Inisialisasi tracing subscriber untuk logging
fn initialize_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,be_toko_online_rust=debug".into()),
        )
        .init();
}

/// Inisialisasi dan validasi konfigurasi aplikasi
async fn initialize_config() -> Result<config::Config, Box<dyn std::error::Error>> {
    println!("Loading configuration...");
    let config = config::Config::from_env();
    config.validate()?;
    println!("Configuration loaded successfully!");
    Ok(config)
}

/// Inisialisasi Xendit payment service
async fn initialize_payment_service() -> Arc<services::payment_service::PaymentService> {
    println!("Initializing Xendit payment gateway...");
    let xendit_config = config::xendit::XenditConfig::from_env();
    let payment_service = Arc::new(services::payment_service::PaymentService::new(xendit_config));
    println!("Xendit payment gateway initialized!");
    payment_service
}

/// Koneksi ke database
async fn connect_database(
    database_url: &str,
) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    println!("Connecting to database...");
    let pool = db::create_pool(database_url).await?;
    println!("Database connected successfully!");
    Ok(pool)
}

/// Build GraphQL schema dengan semua dependencies
fn build_graphql_schema(
    pool: DatabaseConnection,
    payment_service: Arc<services::payment_service::PaymentService>,
) -> AppSchema {
    println!("Building GraphQL schema...");
    let schema = Schema::build(
        schema::QueryRoot,
        schema::MutationRoot,
        EmptySubscription,
    )
    .data(pool)
    .data(payment_service)
    .enable_federation()
    .finish();
    println!("GraphQL schema built successfully!");
    schema
}

/// Konfigurasi CORS untuk production
fn configure_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
}

/// Build aplikasi router dengan semua endpoints
fn build_app_router(
    schema: AppSchema,
    payment_service: Arc<services::payment_service::PaymentService>,
    db: DatabaseConnection,
) -> Router {
    Router::new()
        // Public endpoints
        .route("/", get(root))
        .route("/health", get(health_check))
        
        // GraphQL endpoints
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        
        // Webhook endpoints (Xendit payment notifications)
        .route("/webhook/xendit", 
            get(handlers::webhook::webhook_info)
            .post(handlers::webhook::xendit_webhook)
        )
        
        // Status API endpoints
        .route("/api/status/xendit", get(handlers::status::xendit_status))
        .route("/api/status/webhook", get(handlers::status::webhook_status))
        .route("/api/status/system", get(handlers::status::system_status))
        
        // Extensions untuk dependency injection
        .layer(Extension(schema))
        .layer(Extension(payment_service))
        .layer(Extension(db))
        
        // CORS layer
        .layer(configure_cors())
}

/// Print informasi server setelah start
fn print_server_info(addr: &str) {
    println!("\n========================================");
    println!("🚀 Server started successfully!");
    println!("========================================");
    println!("📍 Local:    http://{}", addr);
    println!("🔧 GraphQL:  http://{}/graphql", addr);
    println!("💚 Health:   http://{}/health", addr);
    println!("🔔 Webhook:  http://{}/webhook/xendit", addr);
    println!("========================================");
    println!("📋 Commands:");
    println!("   - Seeder: cargo run seed");
    println!("   - Stop:   Press Ctrl+C");
    println!("========================================\n");
}

/// Handle database seeding command
async fn handle_seeding(
    pool: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n========================================");
    println!("🌱 Running database seeder...");
    println!("========================================\n");
    
    // Jalankan seeder menggunakan fungsi dari db module
    db::seed_all(pool).await?;
    
    println!("\n========================================");
    println!("✅ Seeding completed successfully!");
    println!("========================================\n");
    Ok(())
}

/// Print help message untuk command line arguments
fn print_help() {
    println!("\n========================================");
    println!("BE Toko Online Rust - Help");
    println!("========================================");
    println!("Usage:");
    println!("  cargo run           - Start server");
    println!("  cargo run seed      - Run database seeder");
    println!("  cargo run help      - Show this help message");
    println!("========================================\n");
}

// ==================== MAIN FUNCTION ====================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Initialize tracing for structured logging
    initialize_tracing();

    // Step 2: Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Step 3: Initialize application configuration
    let config = initialize_config().await?;

    // Step 4: Initialize Xendit payment service
    let payment_service = initialize_payment_service().await;

    // Step 5: Connect to database
    let pool = connect_database(&config.database_url).await?;

    // Step 6: Check for command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "seed" => {
                // Run database seeder
                handle_seeding(&pool).await?;
                return Ok(());
            }
            "help" | "--help" | "-h" => {
                // Show help message
                print_help();
                return Ok(());
            }
            _ => {
                println!("⚠️  Unknown command: {}", args[1]);
                println!("Run 'cargo run help' for available commands.\n");
                return Ok(());
            }
        }
    }

    // Step 7: Build GraphQL schema with dependencies
    let schema = build_graphql_schema(pool.clone(), payment_service.clone());

    // Step 8: Build application router
    let app = build_app_router(
        schema,
        payment_service,
        pool,
    );

    // Step 9: Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    // Step 10: Print server info
    print_server_info(&addr);

    // Step 11: Run server
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}