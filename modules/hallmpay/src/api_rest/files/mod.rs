use crate::api_rest::AppState;
use axum::routing::get;
use axum::Router;

mod router;

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/v1/logo/{uuid}", get(router::get_logo))
        .route("/v1/invoice/pdf/{uuid}", get(router::get_invoice_pdf))
        .route("/v1/quotation/pdf/{uuid}", get(router::get_quotation_pdf))
        .route(
            "/v1/batch_job/errors/{job_id}",
            get(router::get_batch_job_error_csv),
        )
        .route(
            "/v1/batch_job/input/{job_id}",
            get(router::get_batch_job_input_file),
        )
        .route(
            "/v1/batch_job/output/{job_id}",
            get(router::get_batch_job_output_file),
        )
}
