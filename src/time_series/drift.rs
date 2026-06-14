use chrono::{DateTime, Utc};
use tracing::{info, warn};

pub struct DriftCalibration {
    pub meter_id: String,
    pub accumulated_drift_ppb: f64,
    pub last_calibration: DateTime<Utc>,
}

pub struct DriftWorker {
    calibrations: Vec<DriftCalibration>,
}

impl DriftWorker {
    pub fn new() -> Self {
        Self {
            calibrations: Vec::new(),
        }
    }

    pub fn apply_drift_correction(
        &mut self,
        meter_id: &str,
        raw_reading: f64,
        ambient_temp: f64,
    ) -> f64 {
        let calibration = self
            .calibrations
            .iter()
            .find(|c| c.meter_id == meter_id);
        match calibration {
            Some(c) => {
                let temp_coefficient = 0.00015;
                let temp_correction = 1.0 + (ambient_temp - 25.0) * temp_coefficient;
                let drift_correction = 1.0 + c.accumulated_drift_ppb / 1_000_000_000.0;
                let corrected = raw_reading * temp_correction * drift_correction;
                info!(meter_id, raw = raw_reading, corrected, "drift correction applied");
                corrected
            }
            None => {
                self.calibrations.push(DriftCalibration {
                    meter_id: meter_id.to_string(),
                    accumulated_drift_ppb: 0.0,
                    last_calibration: Utc::now(),
                });
                raw_reading
            }
        }
    }
}
