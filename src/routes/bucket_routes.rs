use actix_web::{post, web, HttpResponse, Responder};
use crate::models::bucket::{BucketRequest};
use crate::models::error::ErrorResponse;
use crate::services::bucket_service::solve_bucket_problem;
use crate::validators::security::validate_request;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(solve);
}

#[utoipa::path(
    post,
    path = "/solve",
    request_body = BucketRequest,
    responses(
        (status = 200, description = "Solve bucket problem", body = BucketResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    )
)]
#[post("/solve")]
pub async fn solve(req: web::Json<BucketRequest>) -> impl Responder {
    if let Err(validation_error) = validate_request(&req) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: validation_error,
        });
    }

    match solve_bucket_problem(req.x_capacity, req.y_capacity, req.z_amount_wanted) {
        Some(response) => HttpResponse::Ok().json(response),
        None => HttpResponse::BadRequest().json(ErrorResponse {
            error: "no_solution_possible".to_string(),
        }),
    }
}
