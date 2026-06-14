use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TariffTier {
    Peak,
    OffPeak,
    Shoulder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariffSchedule {
    pub tier: TariffTier,
    pub rate_per_unit: f64,
    pub start_hour: u8,
    pub end_hour: u8,
}

pub struct TariffEngine {
    schedules: Vec<TariffSchedule>,
}

impl TariffEngine {
    pub fn new(schedules: Vec<TariffSchedule>) -> Self {
        Self { schedules }
    }

    pub fn evaluate(&self, timestamp: DateTime<Utc>, volume: f64) -> f64 {
        let hour = timestamp.hour() as u8;
        for schedule in &self.schedules {
            if hour >= schedule.start_hour && hour < schedule.end_hour {
                let cost = volume * schedule.rate_per_unit;
                info!(
                    tier = ?schedule.tier,
                    hour = hour,
                    volume = volume,
                    cost = cost,
                    "tariff evaluated"
                );
                return cost;
            }
        }
        volume * 0.12
    }

    pub fn evaluate_batch(&self, readings: &[(DateTime<Utc>, f64)]) -> f64 {
        readings
            .iter()
            .map(|(ts, vol)| self.evaluate(*ts, *vol))
            .sum()
    }
}
