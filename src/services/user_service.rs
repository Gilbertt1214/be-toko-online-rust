use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::{user, prelude::User};
use crate::models::user::UserRole;

pub struct UserService;

impl UserService {
    /// Register user baru
    pub async fn create_user(
        db: &DatabaseConnection,
        username: String,
        email: String,
        password: String,
        role: UserRole,
    ) -> Result<user::Model, String> {
        // Validasi: cek username sudah ada
        let existing_username = User::find()
            .filter(user::Column::Username.eq(&username))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing_username.is_some() {
            return Err("Username sudah digunakan".to_string());
        }

        // Validasi: cek email sudah ada
        let existing_email = User::find()
            .filter(user::Column::Email.eq(&email))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing_email.is_some() {
            return Err("Email sudah digunakan".to_string());
        }

        // Hash password
        let hashed_password = hash(&password, DEFAULT_COST)
            .map_err(|_| "Gagal hash password")?;

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
            .map_err(|e| format!("Gagal membuat user: {}", e))
    }

    /// Get user by ID
    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Option<user::Model>, String> {
        User::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get user by username
    pub async fn get_user_by_username(
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
    pub async fn get_user_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, String> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get all users
    pub async fn get_all_users(
        db: &DatabaseConnection,
    ) -> Result<Vec<user::Model>, String> {
        User::find()
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Update user
    pub async fn update_user(
        db: &DatabaseConnection,
        user_id: i64,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
        role: Option<UserRole>,
    ) -> Result<Option<user::Model>, String> {
        let Some(existing) = User::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        else {
            return Ok(None);
        };

        let mut active: user::ActiveModel = existing.into();

        if let Some(u) = username {
            // Validasi username unik
            let username_exists = User::find()
                .filter(user::Column::Username.eq(&u))
                .filter(user::Column::Id.ne(user_id))
                .one(db)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            if username_exists.is_some() {
                return Err("Username sudah digunakan".to_string());
            }
            active.username = Set(u);
        }

        if let Some(e) = email {
            // Validasi email unik
            let email_exists = User::find()
                .filter(user::Column::Email.eq(&e))
                .filter(user::Column::Id.ne(user_id))
                .one(db)
                .await
                .map_err(|err| format!("Database error: {}", err))?;

            if email_exists.is_some() {
                return Err("Email sudah digunakan".to_string());
            }
            active.email = Set(e);
        }

        if let Some(p) = password {
            let hashed_password = hash(&p, DEFAULT_COST)
                .map_err(|_| "Gagal hash password")?;
            active.password = Set(Some(hashed_password));
        }

        if let Some(r) = role {
            active.role = Set(r);
        }

        active
            .update(db)
            .await
            .map(Some)
            .map_err(|e| format!("Gagal update user: {}", e))
    }

    /// Delete user
    pub async fn delete_user(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<bool, String> {
        let res = User::delete_by_id(user_id)
            .exec(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(res.rows_affected > 0)
    }

    /// Verify password untuk login
    pub async fn verify_password(
        db: &DatabaseConnection,
        username_or_email: &str,
        password: &str,
    ) -> Result<Option<user::Model>, String> {
        // Coba cari by username
        let mut user = User::find()
            .filter(user::Column::Username.eq(username_or_email))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // Jika tidak ketemu, coba by email
        if user.is_none() {
            user = User::find()
                .filter(user::Column::Email.eq(username_or_email))
                .one(db)
                .await
                .map_err(|e| format!("Database error: {}", e))?;
        }

        if let Some(u) = user {
            if let Some(hashed_pw) = &u.password {
                let is_valid = verify(password, hashed_pw)
                    .map_err(|_| "Gagal verifikasi password")?;

                if is_valid {
                    return Ok(Some(u));
                }
            }
        }

        Ok(None)
    }
}