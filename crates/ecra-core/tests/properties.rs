use std::any::TypeId;

use ecra_core::{
    ActionId, ActionIntent, ActionParametersRef, ActorId, DomainError, EffectProfile, EpochMillis,
    ErrorCode, I_JSON_MAX_SAFE_INTEGER, I_JSON_MIN_SAFE_INTEGER, IdempotencyClass, IdempotencySpec,
    InformationClass, InformationClassification, MutationDomain, OperationRef, PrincipalId,
    ResourceId, ResourceKind, ResourceRef, RetryClass, Reversibility, SchemaVersion, Scope,
    SecurityDigest, TemporalValidity, Versioned,
};
use proptest::prelude::*;
use uuid::Uuid;

#[test]
fn id_newtypes_are_distinct_rust_types() {
    assert_ne!(TypeId::of::<ActorId>(), TypeId::of::<PrincipalId>());

    let uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("fixture UUID");
    let actor = ActorId::from_uuid(uuid);
    let principal = PrincipalId::from_uuid(uuid);

    assert_eq!(actor.to_string(), principal.to_string());
}

#[test]
fn invalid_identifier_returns_machine_readable_code() {
    let error = ActorId::parse_str("not-a-uuid").expect_err("must reject malformed UUID");
    assert_eq!(error.code(), ErrorCode::InvalidIdentifier);
}

#[test]
fn version_dispatch_rejects_unsupported_major_and_minor() {
    let major = SchemaVersion::new(2, 0)
        .validate_supported()
        .expect_err("major 2 is unsupported");
    assert_eq!(major.code(), ErrorCode::UnsupportedMajorVersion);

    let minor = SchemaVersion::new(1, 1)
        .validate_supported()
        .expect_err("newer minor is unsupported");
    assert_eq!(minor.code(), ErrorCode::UnsupportedMinorVersion);
}

#[test]
fn versioned_json_dispatch_is_strict() {
    let input = br#"{"schema_version":{"major":2,"minor":0},"value":{}}"#;
    let error = Versioned::<serde_json::Value>::from_json_slice(input)
        .expect_err("unsupported major must fail");
    assert_eq!(error.code(), ErrorCode::UnsupportedMajorVersion);
}

#[test]
fn temporal_range_rejects_reverse_order() {
    let start = EpochMillis::new(2).expect("safe time");
    let end = EpochMillis::new(1).expect("safe time");
    let error = TemporalValidity::new(Some(start), Some(end)).expect_err("range must fail");
    assert_eq!(error.code(), ErrorCode::InvalidTemporalRange);
}

#[test]
fn security_digest_is_distinct_and_fixed_width() {
    let digest = SecurityDigest::sha256(b"ecra");
    assert_eq!(digest.hex().len(), 64);
    assert_eq!(
        digest.hex(),
        "0f5e365208ae18525b3d5e3cc074cd5e5f2b38bc04a8fe13dd037a94a60ec0d4"
    );

    let error = SecurityDigest::new_sha256("aa").expect_err("short digest must fail");
    assert_eq!(error.code(), ErrorCode::InvalidSecurityDigest);
}

proptest! {
    #[test]
    fn all_i_json_safe_epoch_millis_round_trip(value in I_JSON_MIN_SAFE_INTEGER..=I_JSON_MAX_SAFE_INTEGER) {
        let time = EpochMillis::new(value).expect("generated value is in safe range");
        prop_assert_eq!(time.get(), value);
        let json = serde_json::to_string(&time).expect("serialize");
        let decoded: EpochMillis = serde_json::from_str(&json).expect("deserialize");
        prop_assert_eq!(decoded, time);
    }

    #[test]
    fn information_classification_round_trip_never_changes_class(index in 0u8..5) {
        let class = match index {
            0 => InformationClass::Public,
            1 => InformationClass::Private,
            2 => InformationClass::Sensitive,
            3 => InformationClass::Secret,
            _ => InformationClass::Unknown,
        };
        let classification = InformationClassification::new(class, Vec::new());
        let json = serde_json::to_string(&classification).expect("serialize classification");
        let decoded: InformationClassification =
            serde_json::from_str(&json).expect("deserialize classification");
        prop_assert_eq!(decoded.class(), class);
    }
}

#[test]
fn outside_i_json_range_is_rejected() {
    for value in [I_JSON_MIN_SAFE_INTEGER - 1, I_JSON_MAX_SAFE_INTEGER + 1] {
        let error = EpochMillis::new(value).expect_err("unsafe integer must fail");
        assert!(matches!(error, DomainError::InvalidEpochMillis { .. }));
    }
}

fn action_with(
    effect: EffectProfile,
    idempotency: IdempotencySpec,
    retry: RetryClass,
) -> Result<ActionIntent, DomainError> {
    ActionIntent::new(
        ActionId::parse_str("00000000-0000-0000-0000-000000000101").expect("action id"),
        ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        OperationRef::new("test", "operation").expect("operation"),
        ResourceRef::new(
            ResourceId::parse_str("00000000-0000-0000-0000-000000000030").expect("resource id"),
            ResourceKind::Abstract,
            None,
            None,
        )
        .expect("resource"),
        Scope::not_applicable(),
        ActionParametersRef::None,
        effect,
        idempotency,
        retry,
    )
}

#[test]
fn phase7_effect_idempotency_retry_matrix_is_fail_closed() {
    let mutations = [
        MutationDomain::None,
        MutationDomain::Local,
        MutationDomain::External,
        MutationDomain::Unknown,
    ];
    let reversibilities = [
        Reversibility::NotApplicable,
        Reversibility::Reversible,
        Reversibility::Conditional,
        Reversibility::Irreversible,
        Reversibility::Unknown,
    ];
    let idempotencies = [
        IdempotencyClass::NaturallyIdempotent,
        IdempotencyClass::IdempotentWithKey,
        IdempotencyClass::NonIdempotent,
        IdempotencyClass::Unknown,
    ];
    let retries = [
        RetryClass::Safe,
        RetryClass::RequiresSameIdempotencyKey,
        RetryClass::RequiresExternalReconciliation,
        RetryClass::NeverBlindRetry,
    ];

    for mutation in mutations {
        for reversibility in reversibilities {
            let effect_expected = match mutation {
                MutationDomain::None => reversibility == Reversibility::NotApplicable,
                MutationDomain::Local | MutationDomain::External => {
                    reversibility != Reversibility::NotApplicable
                }
                MutationDomain::Unknown => reversibility == Reversibility::Unknown,
            };
            let effect = EffectProfile::new(mutation, reversibility);
            assert_eq!(
                effect.is_ok(),
                effect_expected,
                "effect matrix mismatch: {mutation:?}/{reversibility:?}"
            );
            let Ok(effect) = effect else {
                continue;
            };

            for class in idempotencies {
                let key_ref =
                    (class == IdempotencyClass::IdempotentWithKey).then(|| "stable-key".to_owned());
                let idempotency =
                    IdempotencySpec::new(class, key_ref).expect("valid idempotency shape");

                for retry in retries {
                    let expected = match retry {
                        RetryClass::Safe => {
                            class == IdempotencyClass::NaturallyIdempotent
                                && mutation != MutationDomain::Unknown
                        }
                        RetryClass::RequiresSameIdempotencyKey => {
                            class == IdempotencyClass::IdempotentWithKey
                                && mutation != MutationDomain::Unknown
                        }
                        RetryClass::RequiresExternalReconciliation => {
                            matches!(mutation, MutationDomain::External | MutationDomain::Unknown)
                        }
                        RetryClass::NeverBlindRetry => true,
                    };
                    assert_eq!(
                        action_with(effect, idempotency.clone(), retry).is_ok(),
                        expected,
                        "retry matrix mismatch: {mutation:?}/{reversibility:?}/{class:?}/{retry:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn phase7_idempotency_key_shape_is_fail_closed() {
    assert!(IdempotencySpec::new(IdempotencyClass::IdempotentWithKey, None).is_err());
    assert!(
        IdempotencySpec::new(IdempotencyClass::IdempotentWithKey, Some(String::new())).is_err()
    );
    for class in [
        IdempotencyClass::NaturallyIdempotent,
        IdempotencyClass::NonIdempotent,
        IdempotencyClass::Unknown,
    ] {
        assert!(IdempotencySpec::new(class, Some("not-authority".to_owned())).is_err());
    }
}
