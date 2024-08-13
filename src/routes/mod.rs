pub mod bucket_routes;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    bucket_routes::init(cfg);
}
