use janus::Statuser;
use ops_core::{async_trait, CheckResponse, Checker};

const ACTION: &str = "Troubleshoot Janus";

/// Checks a Janus backend is healthy.
pub struct JanusChecker<S>
where
    S: Statuser + Send + Sync,
{
    statuser: S,
    impact: String,
}

impl<S> JanusChecker<S>
where
    S: Statuser + Send + Sync,
{
    /// Creates a new Janus health checker
    pub fn new(statuser: S, impact: &str) -> Self {
        Self {
            statuser,
            impact: impact.to_owned(),
        }
    }
}

#[async_trait]
impl<S> Checker for JanusChecker<S>
where
    S: Statuser + Send + Sync,
{
    async fn check(&self) -> CheckResponse {
        match self.statuser.status() {
            Ok(_) => CheckResponse::healthy("Janus is reachable"),
            Err(err) => CheckResponse::unhealthy(
                &format!("Cannot get Janus status: {}", err),
                ACTION,
                &self.impact,
            ),
        }
    }
}
