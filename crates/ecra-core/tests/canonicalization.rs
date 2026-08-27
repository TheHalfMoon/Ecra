use ecra_core::{SchemaVersion, Versioned, to_jcs_vec};
use serde_json::json;

#[test]
fn jcs_orders_object_keys_deterministically() {
    let value = json!({"b": 1, "a": 2});
    let canonical = to_jcs_vec(&value).expect("JCS canonicalization");
    assert_eq!(canonical, br#"{"a":2,"b":1}"#);
}

#[test]
fn canonicalization_is_a_fixed_point() {
    let value = Versioned::new(
        SchemaVersion::V1_0,
        json!({"z": [3, 2, 1], "a": {"b": true, "a": null}}),
    );
    let first = to_jcs_vec(&value).expect("first canonicalization");
    let reparsed: serde_json::Value = serde_json::from_slice(&first).expect("parse canonical JSON");
    let second = to_jcs_vec(&reparsed).expect("second canonicalization");
    assert_eq!(first, second);
}

#[test]
fn negative_zero_normalizes_per_jcs() {
    let value = json!({"n": -0.0});
    let canonical = to_jcs_vec(&value).expect("canonicalize negative zero");
    assert_eq!(canonical, br#"{"n":0}"#);
}
