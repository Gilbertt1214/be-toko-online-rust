use async_graphql::*;
use sea_orm::DatabaseConnection;
use crate::graphql::graphql_types::AuthResponse;
 use crate::models::user::UserRole;
use crate::services::UserService;
use crate::utils::jwt;

#[derive(Default)]
pub struct AuthMutations;

#[Object]
impl AuthMutations {
    /// Register new user
    async fn register(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        password: String,
        role: Option<UserRole>,
    ) -> Result<AuthResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        if username.trim().is_empty() {
            return Err(Error::new("Username tidak boleh kosong"));
        }
        if email.trim().is_empty() || !email.contains('@') {
            return Err(Error::new("Email tidak valid"));
        }
        if password.len() < 6 {
            return Err(Error::new("Password minimal 6 karakter"));
        }

        let user_role = role.unwrap_or(UserRole::Pengguna);
        if user_role != UserRole::Pengguna {
            return Err(Error::new("Cannot register as Admin or Pengusaha through this endpoint"));
        }

        let user = UserService::create_user(db, username, email, password, user_role)
            .await
            .map_err(Error::new)?;

        let token = jwt::create_jwt(user.id, &user.email, &user.username, user.role)
            .map_err(Error::new)?;

        Ok(AuthResponse { token, user })
    }

    /// Login with username/email and password
    async fn login(
        &self,
        ctx: &Context<'_>,
        username_or_email: String,
        password: String,
    ) -> Result<AuthResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        if username_or_email.trim().is_empty() {
            return Err(Error::new("Username/email tidak boleh kosong"));
        }
        if password.is_empty() {
            return Err(Error::new("Password tidak boleh kosong"));
        }

        let user = UserService::verify_password(db, &username_or_email, &password)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Username/email atau password salah"))?;

        let token = jwt::create_jwt(user.id, &user.email, &user.username, user.role)
            .map_err(Error::new)?;

        Ok(AuthResponse { token, user })
    }
}
