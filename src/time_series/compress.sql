-- TimescaleDB hypertable continuous compression policy
-- Run on time_series.meter_readings

CREATE TABLE IF NOT EXISTS meter_readings (
    meter_id TEXT NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    reading_kwh DOUBLE PRECISION,
    voltage DOUBLE PRECISION,
    current_amps DOUBLE PRECISION,
    temperature_c DOUBLE PRECISION,
    metadata JSONB
);

SELECT create_hypertable('meter_readings', 'recorded_at', if_not_exists => TRUE);

ALTER TABLE meter_readings SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'meter_id',
    timescaledb.compress_orderby = 'recorded_at DESC'
);

SELECT add_compression_policy('meter_readings', INTERVAL '7 days', if_not_exists => TRUE);

SELECT add_retention_policy('meter_readings', INTERVAL '365 days', if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS idx_meter_readings_meter_id_ts
    ON meter_readings (meter_id, recorded_at DESC);
