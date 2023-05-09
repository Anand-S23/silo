#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL not provided");

        let jwt_secret = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET not provided");

        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN")
            .expect("JWT_EXPIRED_IN not provided");

        let jwt_maxage = std::env::var("JWT_MAXAGE")
            .expect("JWT_MAXAGE not provided");

        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}

