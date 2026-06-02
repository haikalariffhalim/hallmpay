use axum::Router;
use scalar_api_reference::axum::router;
use serde_json::json;

#[tokio::main]
async fn main() {
    let configuration = json!({
        "url": "https://registry.scalar.com/@scalar/apis/galaxy?format=json",
    });
    let app = Router::new().merge(router("/spec/api/v1", &configuration));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running on http://localhost:8080/spec/api/v1");
    axum::serve(listener, app).await.unwrap();
}
