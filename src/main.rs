use async_graphql::{Context, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sea_orm::{ Database, ColumnTrait, DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use std::env;
use sea_orm::QueryFilter;
use tokio::net::TcpListener;
use utils::auth::{hash_password, verify_password};
use utils::jwt::create_jwt;
mod utils;
mod entities;
use entities::user;

#[derive(Clone)]
struct AppState {
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<user::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let users = user::Entity::find().all(db).await?;
        Ok(users)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        password: String,
    ) -> async_graphql::Result<user::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let hashed = hash_password(&password);

        let new_user = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password: Set(Some(hashed)),
            ..Default::default()
        };

        let res = new_user.insert(db).await?;
        Ok(res)
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> async_graphql::Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;

        if let Some(user) = user::Entity::find()
            .filter(user::Column::Email.eq(email.clone()))
            .one(db)
            .await?
        {
            if let Some(hashed) = user.password {
                if verify_password(&hashed, &password) {
                    let token = create_jwt(&email);
                    return Ok(token); // FE akan simpan JWT ini
                }
            }
        }
        Err("Invalid email or password".into())
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: i32,
        email : String,
        username: Option<String>,
        password: String,
    ) -> async_graphql::Result<Option<user::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;

        if let Some(u) = user::Entity::find_by_id(id).one(db).await? {
            let mut active: user::ActiveModel = u.into();

            if let Some(n) = username {
                active.username = Set(n);
            }
            active.password = Set(Some(password));
            active.email = Set(email);
            let updated = active.update(db).await?;
            return Ok(Some(updated));
        }
        Ok(None)
    }

    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let result = user::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // URL database (ganti sesuai .env)
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/toko_online".to_string());

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    let state = AppState { schema };

    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .with_state(state);

    println!("🚀 Server running at http://localhost:4000");
    println!("📊 GraphiQL available at http://localhost:4000");

    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("Failed to bind to port 4000");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn graphql_handler(
    State(state): State<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Html<String> {
    Html(async_graphql::http::GraphiQLSource::build()
        .endpoint("/graphql")
        .finish())
}
