use crate::payload::EpigeneticPayload;
use std::fs::{create_dir_all, File};
use std::path::Path;

pub fn write_payload_json<P: AsRef<Path>>(
    path: P,
    payload: &EpigeneticPayload,
) -> Result<(), String> {
    if let Some(parent) = path.as_ref().parent() {
        create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let file = File::create(path).map_err(|e| e.to_string())?;
    serde_json::to_writer(file, payload).map_err(|e| e.to_string())
}

pub fn write_payload_json_pretty<P: AsRef<Path>>(
    path: P,
    payload: &EpigeneticPayload,
) -> Result<(), String> {
    if let Some(parent) = path.as_ref().parent() {
        create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let file = File::create(path).map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(file, payload).map_err(|e| e.to_string())
}
