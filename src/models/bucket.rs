use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct BucketRequest {
    pub x_capacity: i32,
    pub y_capacity: i32,
    pub z_amount_wanted: i32,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct BucketResponse {
    pub solution: Vec<BucketStep>,
    pub status: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct BucketStep {
    pub step: usize,
    pub bucket_x: i32,
    pub bucket_y: i32,
    pub action: String,
}

#[derive(Debug, Clone, ToSchema)]
pub struct State {
    pub x: i32,
    pub y: i32,
    pub steps: Vec<BucketStep>,
}

impl State {
    pub fn new(x: i32, y: i32, steps: Vec<BucketStep>) -> Self {
        Self { x, y, steps }
    }
}
