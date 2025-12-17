use crate::constant::constants::*;
use crate::dto::health::Health;
use axum::{Router, response::IntoResponse, routing::get};
use serde_json::json;

/// Health controller
pub trait HealthControllerTrait {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router;
}

/// Health controller implementation struct
pub struct HealthController {}

/// Health controller implementation logic
impl HealthControllerTrait for HealthController {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router {
        /// Maps health check end-point
        async fn map_health() -> impl IntoResponse {
            format!(
                "{}",
                json!(Health {
                    status: String::from(SERVER_RUNNING_STATUS)
                })
            )
        }

        Router::new().route(API_HEALTH_CHECK_PATH, get(map_health))
    }
}

/// Unit test cases
#[cfg(test)]
mod tests {
}
