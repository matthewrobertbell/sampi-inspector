use wasm_bindgen::prelude::*;

use sampi;

use std::collections::HashMap;
use std::str::FromStr;

#[wasm_bindgen]
pub fn sampi_decode(data: &str) -> Result<JsValue, JsValue> {
    let s = sampi::Sampi::from_str(data).map_err(|e| e.to_string())?;

    let mut h = HashMap::new();
    h.insert("unix_time".to_string(), s.unix_time.to_string());
    h.insert("data_string".to_string(), s.data.human_readable());
    h.insert("data_variant".to_string(), s.data.to_string());
    h.insert("public_key".to_string(), s.get_public_key_as_hex());
    h.insert("pow_score".to_string(), s.get_pow_score().to_string());
    h.insert("hash".to_string(), s.get_hash_as_hex());
    h.insert("data_len".to_string(), s.serialized_length.to_string());

    Ok(JsValue::from_serde(&h).map_err(|e| e.to_string())?)
}

#[wasm_bindgen]
pub fn sampi_encode(data: &str, hex: bool, unix_time: Option<bool>) -> Result<String, JsValue> {
    let kp = sampi::SampiKeyPair::new();
    let s = kp.new_sampi().build(sampi::SampiData::String(data.to_string())).map_err(|e| e.to_string())?;

    if hex {
        return Ok(s.to_hex());
    }
    Ok(s.to_base64())
}
