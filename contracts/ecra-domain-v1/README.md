# Ecra Domain v1 Fixtures

This directory contains normative, human-inspectable fixtures for ECR-001.

- `valid/` contains values that MUST parse, validate, round-trip and canonicalize as documented.
- `invalid/` contains values that MUST fail deterministically with a machine-readable error category/code.

Fixture names SHOULD describe the semantic case. Invalid fixtures SHOULD record the expected error code in an adjacent convention or test table; callers MUST NOT assert human display strings.

Security-sensitive v1 objects use strict parsing. Missing/empty scope never means unrestricted, actor attribution never authenticates a principal, and executor receipts never imply independent verification.
