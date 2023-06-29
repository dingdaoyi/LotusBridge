use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
}

impl DatabaseConfig {
    pub fn postgres_database_url(&self) -> String {
        let username = self.username.as_deref().unwrap_or("postgres");
        let password = self.password.as_deref().unwrap_or("postgres");
        let host = self.host.as_deref().unwrap_or("127.0.0.1");
        let port = self.port.unwrap_or(5432);
        let database = self.database.as_deref().unwrap_or("postgres");

        format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, database
        )
    }

    pub fn sqlite_database_url(&self) -> String {
        let database = self.database.as_deref().unwrap_or("sqlite.db");
        format!("sqlite:{}", database)
    }
}
