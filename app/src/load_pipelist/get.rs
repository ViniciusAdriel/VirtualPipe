use serde_json::Value;
use slint::SharedString;

pub fn i32(obj: &Value, key:&str) -> Option<i32> {
    obj.get(key)
        .and_then(|v|v.as_i64())
        .map(|v| v as i32)
}

pub fn string(obj: &Value, key:&str) -> Option<SharedString> {
    obj.get(key)
        .and_then(|v|v.as_str())
        .map(|v| v.into())
}

pub fn bool_or(obj: &Value, key:&str, default:bool) -> bool {
    obj.get(key)
        .and_then(|v|v.as_bool())
        .unwrap_or(default)
}