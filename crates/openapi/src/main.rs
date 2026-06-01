use axum::Router;
use scalar_api_reference::axum::router;
use serde_json::json;
#[tokio::main]
async fn main() {
    let configuration = json!({
        // URL to your OpenAPI document
        // Learn more about the configuration: https://scalar.com/products/api-references/configuration
        "url": "https://registry.scalar.com/@scalar/apis/galaxy?format=json",
    });
    let app = Router::new().merge(router("/scalar", &configuration));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000/scalar");
    axum::serve(listener, app).await.unwrap();
}
