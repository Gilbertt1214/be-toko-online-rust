use sea_orm::{Database, DatabaseConnection, DbErr};


/// Buat koneksi database menggunakan SeaORM
pub async fn create_pool(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(database_url).await?;
    
    Ok(db)
}