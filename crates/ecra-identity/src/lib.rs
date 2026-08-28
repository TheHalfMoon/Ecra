#![forbid(unsafe_code)]

//! Local identity, trust-root and protected-storage foundations for Ecra.
//!
//! ECR-031 owns authenticated local principal context, local enrollment and
//! trust-root lifecycle, bounded assertion issuance/validation, protected
//! trust-state, native secret custody abstractions, protected envelopes and
//! protected anchors.
//!
//! This crate builds on ECR-001 identifiers and domain primitives rather than
//! redefining them. Identity evidence answers **who / on whose behalf** under
//! a bounded trusted local context. It never grants capability authority,
//! approval, declassification, disclosure permission or an execution lease.
//!
//! # Misuse resistance
//!
//! - Actor attribution is not authenticated principal identity.
//! - An assertion reference is not a validated assertion.
//! - Ordinary file/database metadata is not authoritative key lifecycle state.
//! - A protected anchor is not an independent verification receipt.
//! - Native backend absence, lock or unsupported state must fail closed.
//! - ECR-031 has no browser, model, network, provider or protocol execution
//!   surface.
//! - Production secret material must never fall back to plaintext files,
//!   environment variables or an unprotected in-memory substitute.
//!
//! Semantic modules are added only in `tasks.md` dependency order. Phase 1
//! intentionally establishes the trusted crate and dependency/CI boundary
//! before cryptographic or native-backend behavior is implemented.

pub const ECR_031_CONTRACT_MAJOR: u16 = 1;
pub const ECR_031_CONTRACT_MINOR: u16 = 0;
