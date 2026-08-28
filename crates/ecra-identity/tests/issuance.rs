use ecra_core::{EpochMillis, PrincipalId};
use ecra_identity::{EnrollmentId, EnrollmentRecord, TrustRootId};

fn block<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = source.find(start).expect("start marker");
    let tail = &source[start_index..];
    let end_index = tail.find(end).expect("end marker");
    &tail[..end_index]
}

#[test]
fn ordinary_enrollment_metadata_round_trips_without_becoming_a_handle() {
    let record = EnrollmentRecord::new(
        EnrollmentId::parse_str("00000000-0000-0000-0000-000000000030").unwrap(),
        PrincipalId::parse_str("00000000-0000-0000-0000-000000000004").unwrap(),
        TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        EpochMillis::new(1_000).unwrap(),
    );
    let wire = serde_json::to_vec(&record).unwrap();
    let parsed: EnrollmentRecord = serde_json::from_slice(&wire).unwrap();
    parsed.validate().unwrap();
    assert_eq!(parsed.principal_id(), record.principal_id());
}

#[test]
fn public_issuance_request_has_no_subject_principal_injection_field() {
    let source = include_str!("../src/issuance.rs");
    let request = block(
        source,
        "pub struct AssertionIssuanceRequest {",
        "impl AssertionIssuanceRequest",
    );
    assert!(!request.contains("PrincipalId"));
    assert!(!request.contains("subject_principal"));

    let session = block(source, "impl IssuerSession {", "#[cfg(test)]");
    assert!(session.contains("pub(crate) fn from_verified_state"));
    assert!(!session.contains("pub fn from_verified_state"));
    assert!(!session.contains("issue_for_principal"));
}
