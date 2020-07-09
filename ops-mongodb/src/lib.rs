use mongodb::Client;
use ops_core::{async_trait, CheckResponse, Checker};

const ACTION: &str = "Check state of mongo db cluster";

/// Checks a Mongo database is healthy and responding to requests.
pub struct MongoChecker {
    client: Client,
    db: String,
    impact: String,
}

impl MongoChecker {
    /// Creates a new Mongo health checker
    pub fn new(client: Client, db: &str, impact: &str) -> Self {
        Self {
            client,
            db: db.to_owned(),
            impact: impact.to_owned(),
        }
    }
}

#[async_trait]
impl Checker for MongoChecker {
    async fn check(&self) -> CheckResponse {
        match self
            .client
            .database(&self.db)
            .run_command(mongodb::bson::doc! { "ping": 1 }, None)
            .await
        {
            Ok(_) => CheckResponse::healthy("Connection to the mongo server is healthy"),
            Err(err) => CheckResponse::unhealthy(
                &format!("Cannot get in touch with server: {}", err),
                ACTION.into(),
                &self.impact,
            ),
        }
    }
}
