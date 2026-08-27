use ecra_core::{ActionIntent, ActionRef, SecurityDigest, Versioned, to_jcs_vec};
use serde::Deserialize;

const ACTION_INTENT_V1_DOMAIN: &[u8] = b"ecra/action-intent/v1\0";
const GOLDEN_DIGEST: &str = "6ccf10a5d1db36cb9637b4ed2ee6d3f0167c44eec585b543f58d86287710e3d4";

fn golden_intent() -> ActionIntent {
    serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("golden action intent fixture")
}

fn digest_from_json(value: &serde_json::Value) -> String {
    let intent: ActionIntent =
        serde_json::from_value(value.clone()).expect("valid action mutation");
    intent.digest().expect("action digest").hex().to_owned()
}

#[test]
fn golden_action_digest_matches_domain_separated_contract() {
    let intent = golden_intent();
    let digest = intent.digest().expect("action digest");
    assert_eq!(digest.hex(), GOLDEN_DIGEST);

    let canonical = to_jcs_vec(&Versioned::v1(&intent)).expect("canonical action intent");
    let mut bytes = Vec::with_capacity(ACTION_INTENT_V1_DOMAIN.len() + canonical.len());
    bytes.extend_from_slice(ACTION_INTENT_V1_DOMAIN);
    bytes.extend_from_slice(&canonical);
    let independent = SecurityDigest::sha256(&bytes);
    assert_eq!(digest.hex(), independent.hex());
}

#[test]
fn every_security_relevant_action_intent_field_changes_digest() {
    let base: serde_json::Value = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("golden JSON");
    let base_digest = digest_from_json(&base);

    let mut mutations = Vec::new();

    let mut value = base.clone();
    value["id"] = serde_json::json!("00000000-0000-0000-0000-000000000201");
    mutations.push(value);

    let mut value = base.clone();
    value["actor"] = serde_json::json!("00000000-0000-0000-0000-000000000002");
    mutations.push(value);

    let mut value = base.clone();
    value["principal"] = serde_json::json!({"id":"00000000-0000-0000-0000-000000000002"});
    mutations.push(value);

    let mut value = base.clone();
    value["identity_assertion"] = serde_json::json!({
        "id":"00000000-0000-0000-0000-000000000003",
        "principal":"00000000-0000-0000-0000-000000000002"
    });
    mutations.push(value);

    let mut value = base.clone();
    value["operation"]["name"] = serde_json::json!("inspect_alt");
    mutations.push(value);

    let mut value = base.clone();
    value["target"]["id"] = serde_json::json!("00000000-0000-0000-0000-000000000035");
    mutations.push(value);

    let mut value = base.clone();
    value["scope"]["purpose"] = serde_json::json!({"namespace":"ecra","name":"audit"});
    mutations.push(value);

    let mut value = base.clone();
    value["parameters"] = serde_json::json!({
        "kind":"bound_external",
        "external_ref":"provider:params/golden",
        "binding_digest":{
            "algorithm":"sha256",
            "hex":"3333333333333333333333333333333333333333333333333333333333333333"
        }
    });
    mutations.push(value);

    let mut value = base.clone();
    value["information_use"] = serde_json::json!([{
        "sources":[{"kind":"observation","id":"00000000-0000-0000-0000-000000000040"}],
        "kind":"local_compute",
        "destination":null,
        "destination_origin":null,
        "declared_output_classification":null
    }]);
    mutations.push(value);

    let mut value = base.clone();
    value["effect"] = serde_json::json!({"mutation":"local","reversibility":"irreversible"});
    mutations.push(value);

    let mut value = base.clone();
    value["idempotency"] = serde_json::json!({"class":"non_idempotent","key_ref":null});
    mutations.push(value);

    let mut value = base.clone();
    value["retry"] = serde_json::json!("safe");
    mutations.push(value);

    let mut value = base.clone();
    value["created_at"] = serde_json::json!(1001);
    mutations.push(value);

    let mut value = base.clone();
    value["correlation_id"] = serde_json::json!("digest-golden-alt");
    mutations.push(value);

    for mutation in mutations {
        assert_ne!(
            digest_from_json(&mutation),
            base_digest,
            "security-relevant action mutation must change ActionDigest"
        );
    }
}

#[test]
fn parameter_binding_digest_change_changes_action_digest() {
    let mut first: serde_json::Value = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("golden JSON");
    first["parameters"] = serde_json::json!({
        "kind":"bound_external",
        "external_ref":"provider:params/golden",
        "binding_digest":{
            "algorithm":"sha256",
            "hex":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
    });
    let mut second = first.clone();
    second["parameters"]["binding_digest"]["hex"] =
        serde_json::json!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");

    assert_ne!(digest_from_json(&first), digest_from_json(&second));
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionRefFixture {
    action: ActionIntent,
    reference: ActionRef,
}

#[test]
fn wrong_action_ref_digest_fails_validation() {
    let fixture: ActionRefFixture = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/invalid/action-ref-wrong-digest.json"
    ))
    .expect("wrong digest fixture remains structurally parseable");
    assert!(fixture.reference.validate_for(&fixture.action).is_err());
}
