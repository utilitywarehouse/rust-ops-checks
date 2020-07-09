use std::convert::TryInto;

use crate::proto::{
    health_check_response::ServingStatus, health_client::HealthClient, HealthCheckRequest,
};

use ops_core::{async_trait, CheckResponse, Checker};
use tonic::transport::{Channel, Endpoint, Error};

const ACTION: &str = "Check state of GRPC server";

mod proto {
    tonic::include_proto!("grpc.health.v1");
}

/// Checks a gRPC server is healthy and responding to requests.
pub struct GrpcChecker {
    client: HealthClient<Channel>,
    service: Option<String>,
    impact: String,
}

impl GrpcChecker {
    /// Creates a new gRPC health checker.
    pub async fn new<D>(dst: D, service: Option<&str>, impact: &str) -> Result<Self, Error>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        let client = HealthClient::connect(dst).await?;

        Ok(Self {
            client,
            service: service.map(String::from),
            impact: impact.to_owned(),
        })
    }
}

#[async_trait]
impl Checker for GrpcChecker {
    async fn check(&self) -> CheckResponse {
        let mut client = self.client.clone();

        let service = match &self.service {
            Some(service) => service.to_owned(),
            None => "".to_owned(),
        };

        let req = tonic::Request::new(HealthCheckRequest { service });

        let resp = match client.check(req).await {
            Ok(resp) => resp.into_inner(),
            Err(err) => return CheckResponse::unhealthy(&err.to_string(), ACTION, &self.impact),
        };

        match ServingStatus::from_i32(resp.status) {
            Some(ServingStatus::Serving) => {
                CheckResponse::healthy("GRPC API returned status serving")
            }
            status => CheckResponse::unhealthy(
                &format!("Unexpected status: {:?}", status),
                ACTION,
                &self.impact,
            ),
        }
    }
}
