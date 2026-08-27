#![forbid(unsafe_code)]

//! Ecra's zero-I/O trusted domain kernel.
//!
//! ECR-001 intentionally contains only provider-neutral value objects,
//! validation, serialization and canonical security-binding helpers.
//! Runtime execution, authorization, persistence, browser/model integration,
//! protocols, secrets and telemetry belong to downstream slices.
