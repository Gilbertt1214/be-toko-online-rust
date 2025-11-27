use async_graphql::*;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::models::{user, prelude::*};
use crate::utils::jwt;

#[derive(Default)]
pub struct UserMutations;

#[Object]
impl UserMutations {
    /// Update user profile
    async fn update_profile(
        &self,
        ctx: &Context<'_>,
        token: String,
        username: Option<String>,
        email: Option<String>,
    ) -> Result<user::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if let Some(ref u) = username {
            if u.trim().is_empty() {
                return Err(Error::new("Username tidak boleh kosong"));
            }
        }

        if let Some(ref e) = email {
            if e.trim().is_empty() || !e.contains('@') {
                return Err(Error::new("Email tidak valid"));
            }
        }

        let user = User::find_by_id(claims.user_id)
            .one(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?
            .ok_or_else(|| Error::new("User not found"))?;

        let mut user_active: user::ActiveModel = user.into();
        
        if let Some(u) = username {
            user_active.username = Set(u);
        }
        if let Some(e) = email {
            user_active.email = Set(e);
        }

        user_active
            .update(db)
            .await
            .map_err(|e| Error::new(format!("Failed to update profile: {}", e)))
    }

    /// Change password
    async fn change_password(
        &self,
        ctx: &Context<'_>,
        token: String,
        old_password: String,
        new_password: String,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if new_password.len() < 6 {
            return Err(Error::new("Password baru minimal 6 karakter"));
        }

        let user = User::find_by_id(claims.user_id)
            .one(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?
            .ok_or_else(|| Error::new("User not found"))?;

        let current_password = user.password
            .as_ref()
            .ok_or_else(|| Error::new("User password not set"))?;

        let is_valid = bcrypt::verify(&old_password, current_password)
            .map_err(|_| Error::new("Failed to verify password"))?;

        if !is_valid {
            return Err(Error::new("Password lama salah"));
        }

        let hashed = bcrypt::hash(&new_password, bcrypt::DEFAULT_COST)
            .map_err(|_| Error::new("Failed to hash password"))?;

        let mut user_active: user::ActiveModel = user.into();
        user_active.password = Set(Some(hashed));

        user_active
            .update(db)
            .await
            .map_err(|e| Error::new(format!("Failed to change password: {}", e)))?;

        Ok("Password berhasil diubah".to_string())
    }
}
