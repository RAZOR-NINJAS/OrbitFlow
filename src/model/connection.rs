use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from_id: usize,
    pub to_id: usize,
    pub strength: f64,
    pub rest_length: f64,
    #[serde(skip)]
    pub current_tension: f64,
}

impl Connection {
    pub fn new(from_id: usize, to_id: usize, rest_length: f64) -> Self {
        Self {
            from_id,
            to_id,
            strength: 0.12,
            rest_length,
            current_tension: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }
}
