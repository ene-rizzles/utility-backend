use utility_backend::time_series::analytics::analyze_consumption;

#[test]
fn test_anomaly_detection_baseline() {
    use chrono::Utc;
    let readings = vec![(Utc::now(), 100.0), (Utc::now(), 102.0), (Utc::now(), 98.0)];
    let result = analyze_consumption("MTR-001", &readings, 300.0, 50.0);
    assert!(!result.anomaly_detected);
}

#[test]
fn test_anomaly_detection_leak() {
    use chrono::Utc;
    let readings = vec![
        (Utc::now(), 500.0),
        (Utc::now(), 600.0),
        (Utc::now(), 550.0),
    ];
    let result = analyze_consumption("MTR-002", &readings, 300.0, 50.0);
    assert!(result.anomaly_detected);
}
