use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::models::user::UserRole;
use crate::models::{prelude::User, user};

#[allow(dead_code)]
pub struct UserService;

impl UserService {
    /// Create new user
    pub async fn create_user(
        db: &DatabaseConnection,
        username: String,
        email: String,
        password: String,
        role: UserRole,
    ) -> Result<user::Model, String> {
        // Check if username exists
        let existing_username = User::find()
            .filter(user::Column::Username.eq(&username))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing_username.is_some() {
            return Err("Username already exists".to_string());
        }

        // Check if email exists
        let existing_email = User::find()
            .filter(user::Column::Email.eq(&email))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing_email.is_some() {
            return Err("Email already exists".to_string());
        }

        // Hash password
        let hashed_password =
            hash(&password, DEFAULT_COST).map_err(|_| "Failed to hash password")?;

        let new_user = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password: Set(Some(hashed_password)),
            role: Set(role),
            ..Default::default()
        };

        new_user
            .insert(db)
            .await
            .map_err(|e| format!("Failed to create user: {}", e))
    }

    /// Get user by ID
    pub async fn get_by_id(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Option<user::Model>, String> {
        User::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get user by username
    pub async fn get_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<user::Model>, String> {
        User::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get user by email
    pub async fn get_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, String> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Verify user password
    pub async fn verify_password(
        db: &DatabaseConnection,
        username: &str,
        password: &str,
    ) -> Result<Option<user::Model>, String> {
        let user = Self::get_by_username(db, username).await?;

        if let Some(user) = user {
            if let Some(hashed_password) = &user.password {
                let is_valid = verify(password, hashed_password)
                    .map_err(|_| "Failed to verify password")?;

                if is_valid {
                    return Ok(Some(user));
                }
            }
        }

        Ok(None)
    }

    /// Get all users
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<user::Model>, String> {
        User::find()
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }
}