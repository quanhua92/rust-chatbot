use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

use llm::InferenceSession;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use zstd::{stream::{read::Decoder, write::Encoder}, zstd_safe::CompressionLevel};

const COMPRESSION_LEVEL: CompressionLevel = 1;

// read the session from file
pub fn read_session(model: &llm::models::Llama, path: &Path) -> Result<InferenceSession, String> {
    if path.exists() {
        if let Ok(file) = File::open(path) {
            if let Ok(decoder) = Decoder::new(BufReader::new(file)) {
                if let Ok(snapshot) = bincode::deserialize_from(decoder) {
                    if let Ok(session) = InferenceSession::from_snapshot(snapshot, model) {
                        return Ok(session);
                    }
                }
            }
        }
    }
    Err("Could not load session from {path:?}".to_string())
}

// Write the session to file
pub unsafe fn write_session(mut session: InferenceSession, path: &Path) {
    let snapshot = session.get_snapshot();
    if let Ok(file) = File::create(path) {
        if let Ok(encoder) = Encoder::new(BufWriter::new(file), COMPRESSION_LEVEL) {
            if let Ok(_) = bincode::serialize_into(encoder.auto_finish(), &snapshot) {
                println!("Saved session to {path:?}");
            } else {
                println!("Could not serialize session to {path:?}");
            }
        } else {
            println!("Could not create encoder {path:?}");
        }
    } else {
        println!("Could not create file {path:?}");
    }
}

    }
}
