use async_graphql::{Context, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{routing::get, routing::post, Router, response::Html, extract::State};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use std::env;
use tokio::net::TcpListener;

mod entities;
use entities::user;

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
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> async_graphql::Result<user::Model> {
        let db = ctx.data::<DatabaseConnection>()?;

        let new_user = user::ActiveModel {
            username: Set(username),
            password: Set(Some(password)),
            ..Default::default()
        };

        let res = new_user.insert(db).await?;
        Ok(res)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: i32,
        username: Option<String>,
        password: Option<String>,
    ) -> async_graphql::Result<Option<user::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;

        if let Some(u) = user::Entity::find_by_id(id).one(db).await? {
            let mut active: user::ActiveModel = u.into();

            if let Some(n) = username {
                active.username = Set(n);
            }
            if let Some(p) = password {
                active.password = Set(Some(p));
            }

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
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = Database::connect(&database_url).await.expect("DB connect error");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .with_state(schema);

    println!("🚀 Server running at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn graphql_handler(
    State(schema): State<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Html<String> {
    Html(async_graphql::http::GraphiQLSource::build().endpoint("/graphql").finish())
}
