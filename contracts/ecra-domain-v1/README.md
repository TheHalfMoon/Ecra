# Ecra Domain v1 Fixtures

This directory contains normative, human-inspectable fixtures for ECR-001.

- `valid/` contains values that MUST parse, validate, round-trip and canonicalize as documented.
- `invalid/` contains values that MUST fail deterministically with a machine-readable error category/code.

Most fixture files intentionally store the inner semantic value `T` rather than repeating a version envelope in every file. The exhaustive valid-fixture runner MUST wrap each typed value in `Versioned<T>` and round-trip it through the supported v1 dispatch before the fixture is accepted. Compatibility fixtures that test version dispatch are complete `Versioned<T>` envelopes.

Fixture names SHOULD describe the semantic case. Invalid fixtures are bound to an expected machine-readable error code by the exhaustive invalid-fixture manifest; callers MUST NOT assert human display strings.

Security-sensitive v1 objects use strict parsing. Missing/empty scope never means unrestricted, actor attribution never authenticates a principal, and executor receipts never imply independent verification.
