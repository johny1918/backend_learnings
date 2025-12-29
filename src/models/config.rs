
#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: Secret<String>,
    pub port: u16,
}

pub struct Secret<T>(pub T);

impl <T> std::fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"***SECRET***")
    }
}