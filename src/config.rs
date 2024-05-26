use once_cell::sync::Lazy;
use std::env;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::from_env());

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
}

impl Config {
    fn from_env() -> Self {
        dotenvy::from_filename(if env::var("TEST_ENV").is_ok() {
            ".env.test"
        } else {
            ".env"
        })
        .expect("failed to load environment variables from file");

        let server = ServerConfig {
            host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
            port: env::var("PORT")
                .unwrap_or_else(|_| String::from("8000"))
                .parse::<u16>()
                .expect("PORT must be a number"),
        };

        let database = DatabaseConfig {
            url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        };

        Self { server, database }
    }

    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }
}
