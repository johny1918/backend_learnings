use std::env;

use crate::models::config::AppConfig;

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL is required"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET is required"),
            port: env::var("PORT").unwrap_or_else(|_|"3000".into()).parse().unwrap(),
        }   
    }
}