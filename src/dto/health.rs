use serde::{Deserialize, Serialize};

/// Health struct
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Default))]
pub struct Health {
    /// health status
    pub status: String,
}

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::dto::health::Health;

    /// Scenario:
    /// Creates a [Health] struct with valid values
    /// Expectation:
    /// A [Health] with proper values should be created
    #[test]
    fn when_create_health_with_proper_values_should_retrieve_set_values() {
        let status_value = "some_value";
        let health = Health {
            status: String::from(status_value),
        };

        assert_eq!(status_value, health.status);
    }
}
