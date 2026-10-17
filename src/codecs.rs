//! The concrete codecs the `EncodingRegistry` composes.
//!
//! They live beside the trait rather than in `lib-base64`/`lib-hex` because
//! `encoding.encode`/`encoding.decode` dispatch **by name** through one
//! registry; putting the impls in the codec repos would make `lib-encoding`
//! depend on both (a cycle, since they implement its trait). The codec repos
//! own the `.hnl` surface and the op handlers, and import these types.

use serde_json::Value;

use crate::{Encoding, EncodingError};

/// Base64 (standard alphabet, padded) codec.
#[derive(Debug, Default, Clone, Copy)]
pub struct Base64;

impl Encoding for Base64 {
    fn encode(&self, value: &Value) -> Result<Value, EncodingError> {
        use base64::{engine::Engine as _, prelude::BASE64_STANDARD};
        let s = value
            .as_str()
            .ok_or_else(|| EncodingError::Message("base64.encode expects a string".to_string()))?;
        Ok(Value::String(BASE64_STANDARD.encode(s.as_bytes())))
    }

    fn decode(&self, value: &Value) -> Result<Value, EncodingError> {
        use base64::{engine::Engine as _, prelude::BASE64_STANDARD};
        let s = value
            .as_str()
            .ok_or_else(|| EncodingError::Message("base64.decode expects a string".to_string()))?;
        let decoded = BASE64_STANDARD
            .decode(s)
            .map_err(|e| EncodingError::Message(format!("base64 decode error: {e}")))?;
        let out = String::from_utf8(decoded).map_err(|e| {
            EncodingError::Message(format!("base64 decode is not valid utf-8: {e}"))
        })?;
        Ok(Value::String(out))
    }
}

/// Lowercase hex codec.
#[derive(Debug, Default, Clone, Copy)]
pub struct Hex;

impl Encoding for Hex {
    fn encode(&self, value: &Value) -> Result<Value, EncodingError> {
        let s = value
            .as_str()
            .ok_or_else(|| EncodingError::Message("hex.encode expects a string".to_string()))?;
        Ok(Value::String(hex::encode(s.as_bytes())))
    }

    fn decode(&self, value: &Value) -> Result<Value, EncodingError> {
        let s = value
            .as_str()
            .ok_or_else(|| EncodingError::Message("hex.decode expects a string".to_string()))?;
        let decoded =
            hex::decode(s).map_err(|e| EncodingError::Message(format!("hex decode error: {e}")))?;
        let out = String::from_utf8(decoded)
            .map_err(|e| EncodingError::Message(format!("hex decode is not valid utf-8: {e}")))?;
        Ok(Value::String(out))
    }
}
