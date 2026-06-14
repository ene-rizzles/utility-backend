use utility_backend::api::middleware::TokenBucket;

#[tokio::test]
async fn test_token_bucket_rate_limit() {
    let bucket = TokenBucket::new(5, 1);
    for _ in 0..5 {
        assert!(bucket.try_consume(1));
    }
    assert!(!bucket.try_consume(1));
}

#[test]
fn test_meter_api_serialization() {
    let info = utility_backend::api::handlers::MeterInfo {
        id: "MTR-X".into(),
        tenant_id: "grid-north".into(),
        location: "dam-beta".into(),
        last_reading: 987.65,
    };
    let json = serde_json::to_string(&info).unwrap();
    assert!(json.contains("MTR-X"));
    assert!(json.contains("grid-north"));
}
