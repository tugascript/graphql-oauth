use migrations::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new(environment: &str) -> Self {
        let con_str = match environment {
            "production" => std::env::var("DATABASE_URL").unwrap(),
            _ => "sqlite::memory:".to_owned(),
        };
        let connection = sea_orm::Database::connect(con_str)
            .await
            .expect("Could not connect to database");

        if environment != "production" {
            Migrator::up(&connection, None)
                .await
                .expect("Could not run migrations");
        }

        Database { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
