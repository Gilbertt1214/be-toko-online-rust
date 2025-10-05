use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL harus di-set di file .env"),
            
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("SERVER_PORT harus berupa angka valid (contoh: 8000)"),
            
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| {
                    eprintln!("WARNING: JWT_SECRET tidak di-set, menggunakan default (TIDAK AMAN untuk production!)");
                    "default-secret-key-change-in-production".to_string()
                }),
        }
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.database_url.is_empty() {
            return Err("DATABASE_URL tidak boleh kosong".to_string());
        }
        
        if self.jwt_secret.len() < 32 {
            eprintln!("WARNING: JWT_SECRET sebaiknya minimal 32 karakter untuk keamanan");
        }
        
        if self.server_port < 1024 {
            eprintln!("WARNING: Port < 1024 memerlukan privileges khusus");
        }
        
        Ok(())
    }
}