use hallmpay::{ ApiVersion, HallmPayError, PaymentChannel, PaymentIntentRequest};
use serde_json::json;

#[tokio::test]
async fn test_create_payment_intent() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/payment-intents")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "pmtint_123",
                "payer_name": "Ali Yahoo",
                "payer_email": "ali@yahoo.com",
                "order_number": "ORDER123",
                "amount": 100.50,
                "url": "https://pay.hallm.io/redirect",
                "type": "payment_intent",
                "status": "pending",
                "currency": "MYR",
                "attempts": []
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = hallmpay::HallmPay::builder("test_token")
        .build_with_base_url(&server.url())
        .unwrap();

    let request = PaymentIntentRequest {
        payment_channel: PaymentChannel::Fpx as u8,
        order_number: "ORDER123".to_string(),
        amount: 100.50,
        payer_name: "Ali Yahoo".to_string(),
        payer_email: "ali@yahoo.com".to_string(),
        payer_telephone_number: None,
        currency: None,
        callback_url: None,
        return_url: None,
        metadata: None,
        checksum: None,
    };

    let result: hallmpay::PaymentIntent = client.create_payment_intent(&request).await.unwrap();
    assert_eq!(result.id, "pmtint_123");
    assert_eq!(result.status, "pending");
    assert_eq!(result.url, "https://pay.hallm.io/redirect");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_transaction_404() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/transactions/nonexistent")
        .with_status(404)
        .with_body("{}")
        .create_async()
        .await;

    let client = hallmpay::HallmPay::builder("test_token")
        .build_with_base_url(&server.url())
        .unwrap();

    let result: hallmpay::Result<hallmpay::Transaction> = client.get_transaction("nonexistent").await;
    	assert!(result.is_err());
     	assert!(matches!(result.unwrap_err(), HallmPayError::NotFound));
      	mock.assert_async().await;
}

#[tokio::test]
async fn test_v2_method_on_v1_returns_error() {
    // No server needed - this check happens before any HTTP call
    let client = hallmpay::HallmPay::builder("test_token")
        .api_version(ApiVersion::V1)
        .build_with_base_url("http://localhost:3000")
        .unwrap();

    let result: hallmpay::Result<hallmpay::PaymentIntent> = client.get_payment_intent("pmtint_123").await;
    assert!(result.is_err());
    match result.unwrap_err() {
        HallmPayError::ApiVersionMismatch(method) => assert_eq!(method, "get_payment_intent"),
        other => panic!("Expected ApiVersionMismatch, got {:?}", other),
    }
}

#[tokio::test]
async fn test_fpx_banks_list() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/banks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {"id": "1", "name": "Maybank", "code": "MBB", "status": "active"},
                {"id": "2", "name": "CIMB", "code": "CIMB", "status": "active"}
            ])
            .to_string(),
        )
        .create_async()
        .await;

    let client = hallmpay::HallmPay::builder("test_token")
        .build_with_base_url(&server.url())
        .unwrap();

    let banks: Vec<hallmpay::FpxBank> = client.fpx_banks_list().await.unwrap();
    assert_eq!(banks.len(), 2);
    assert_eq!(banks[0].name, "Maybank");
    assert_eq!(banks[1].code, "CIMB");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_validation_error_422() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/payment-intents")
        .with_status(422)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "The given data was invalid.",
                "errors": {
                    "amount": ["The amount must be greater than 0."],
                    "payer_email": ["The payer email field is required."]
                }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = hallmpay::HallmPay::builder("test_token")
        .build_with_base_url(&server.url())
        .unwrap();

    let request = PaymentIntentRequest {
        payment_channel: 1,
        order_number: "ORD1".to_string(),
        amount: 0.0,
        payer_name: "Test".to_string(),
        payer_email: "".to_string(),
        payer_telephone_number: None,
        currency: None,
        callback_url: None,
        return_url: None,
        metadata: None,
        checksum: None,
    };

    let result: hallmpay::Result<hallmpay::PaymentIntent> = client.create_payment_intent(&request).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        HallmPayError::Validation { message, errors } => {
            assert!(message.contains("invalid"));
            assert!(errors.contains_key("amount"));
            assert!(errors.contains_key("payer_email"));
        }
        other => panic!("Expected Validation, got {:?}", other),
    }
    mock.assert_async().await;
}
