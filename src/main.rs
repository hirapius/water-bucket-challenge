mod routes;
mod services;
mod models;
mod validators;

#[cfg(test)]
mod tests;

use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::bucket_routes::solve,
    ),
    components(schemas(
        models::bucket::BucketRequest,
        models::bucket::BucketResponse,
        models::bucket::BucketStep,
        models::error::ErrorResponse
    )),
    tags(
        (name = "bucket", description = "Water Bucket Challenge API")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::init)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
