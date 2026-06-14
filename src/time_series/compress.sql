-- TimescaleDB hypertable continuous compression policy
-- Compatible with both PostgreSQL and TimescaleDB

CREATE TABLE IF NOT EXISTS meter_readings (
    meter_id TEXT NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    reading_kwh DOUBLE PRECISION,
    voltage DOUBLE PRECISION,
    current_amps DOUBLE PRECISION,
    temperature_c DOUBLE PRECISION,
    metadata JSONB
);

-- TimescaleDB-specific setup (silently skipped on plain PostgreSQL)
DO $$
BEGIN
    PERFORM * FROM pg_extension WHERE extname = 'timescaledb';
    IF FOUND THEN
        PERFORM create_hypertable('meter_readings', 'recorded_at', if_not_exists => TRUE);
        ALTER TABLE meter_readings SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'meter_id',
            timescaledb.compress_orderby = 'recorded_at DESC'
        );
        PERFORM add_compression_policy('meter_readings', INTERVAL '7 days', if_not_exists => TRUE);
        PERFORM add_retention_policy('meter_readings', INTERVAL '365 days', if_not_exists => TRUE);
    END IF;
END
$$;

CREATE INDEX IF NOT EXISTS idx_meter_readings_meter_id_ts
    ON meter_readings (meter_id, recorded_at DESC);
