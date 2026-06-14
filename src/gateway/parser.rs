use bytes::Bytes;
use tracing::warn;

pub struct CompressedEnvelope {
    pub meter_id: String,
    pub payload: Vec<u8>,
    pub checksum: [u8; 32],
}

pub fn parse_envelope(data: &[u8]) -> Result<CompressedEnvelope, &'static str> {
    if data.len() < 40 {
        return Err("envelope too short");
    }
    let meter_id_len = u16::from_be_bytes([data[0], data[1]]) as usize;
    if data.len() < 2 + meter_id_len + 32 {
        return Err("malformed envelope: meter_id truncated");
    }
    let meter_id_bytes = &data[2..2 + meter_id_len];
    let meter_id = String::from_utf8(meter_id_bytes.to_vec())
        .map_err(|_| "invalid utf-8 meter_id")?;
    let payload_start = 2 + meter_id_len;
    let payload_end = data.len() - 32;
    let payload = data[payload_start..payload_end].to_vec();
    let mut checksum = [0u8; 32];
    checksum.copy_from_slice(&data[payload_end..]);
    Ok(CompressedEnvelope {
        meter_id,
        payload,
        checksum,
    })
}
