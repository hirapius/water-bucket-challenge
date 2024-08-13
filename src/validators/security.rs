use crate::models::bucket::BucketRequest;

pub fn validate_request(req: &BucketRequest) -> Result<(), String> {
    if req.x_capacity <= 0 || req.y_capacity <= 0 || req.z_amount_wanted < 0 {
        return Err("capacities_must_be_positive_integers.".to_string());
    }
    if req.z_amount_wanted > req.x_capacity && req.z_amount_wanted > req.y_capacity {
        return Err("the_target_amount_cannot_be_greater_than_both_bucket_capacities.".to_string());
    }
    Ok(())
}
