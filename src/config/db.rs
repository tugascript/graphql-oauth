use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        let con_str = std::env::var("DATABASE_URL").unwrap();
        let connection = sea_orm::Database::connect(con_str)
            .await
            .expect("Could not connect to database");

        Database { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
