use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NociceptorConfig {
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub threshold: f32,
}

impl NociceptorConfig {
    pub fn new(alpha: f32, beta: f32, gamma: f32, threshold: f32) -> Result<Self, String> {
        if alpha.is_nan() || beta.is_nan() || gamma.is_nan() || threshold.is_nan() {
            return Err("Config weights/threshold cannot be NaN".to_string());
        }
        if alpha.is_infinite()
            || beta.is_infinite()
            || gamma.is_infinite()
            || threshold.is_infinite()
        {
            return Err("Config weights/threshold cannot be infinite".to_string());
        }
        if threshold < 0.0 {
            return Err("Config threshold cannot be negative".to_string());
        }
        Ok(Self {
            alpha,
            beta,
            gamma,
            threshold,
        })
    }

    pub fn default_p0() -> Self {
        Self::new(1.0, 1.0, 1.0, 10.0).unwrap()
    }

    pub fn strict() -> Self {
        Self::new(2.0, 2.0, 2.0, 5.0).unwrap()
    }

    pub fn lenient() -> Self {
        Self::new(0.5, 0.5, 0.5, 20.0).unwrap()
    }
}
