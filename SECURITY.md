# Security Policy

## Supported Versions

This is a pre-1.0 component library (`version = "0.1.0"` across the
workspace). Only the latest `main` is supported. There are no
maintained legacy branches.

## Reporting a Vulnerability

If you discover a security issue, **please do not open a public issue**.
Instead, report privately via GitHub's security advisory mechanism:

1. Open <https://github.com/ChiranjibChaudhuri/dioxus-kinetics/security/advisories/new>
   (or the equivalent "Report a vulnerability" link on the repo's
   Security tab).
2. Include:
   - A description of the issue and its impact.
   - A minimal reproducer (preferably a failing test or a small
     Dioxus app showing the unsafe behaviour).
   - Affected crate(s) and approximate code locations.
   - Suggested mitigation if known.

Expect an acknowledgement within 72 hours and a remediation timeline
within one week of acknowledgement.

## Scope

In scope:

- All crates under `crates/`.
- The component gallery example under `examples/component-gallery/`.
- The Playwright audit harness under `examples/component-gallery/e2e/`.

Out of scope (report upstream instead):

- Vulnerabilities in `dioxus`, `wgpu`, `wasm-bindgen`, `web-sys`,
  `tokio`, or any third-party dependency. Report to the upstream
  project's security channel.
- Vulnerabilities in the user's application that consumes Kinetics —
  this library does not perform authentication, authorization, data
  storage, or network I/O on the consumer's behalf.

## Hardening Defaults

- `#![forbid(unsafe_code)]` is enforced in every production crate.
  The only exceptions are `ui-glass-engine` (wgpu requires `unsafe`
  for FFI; isolated to clearly-marked functions) and `tests/`.
- The workspace CI runs `cargo deny check advisories bans licenses
  sources` on every PR (`.github/workflows/ci.yml`) — known
  vulnerabilities in the dep graph fail the build.
- `Cargo.lock` is committed for reproducible builds.
- No runtime `eval`, no dynamic code loading, no native binary
  dependencies beyond what `wgpu` and `web-sys` require.

## Supply-Chain Checks

Run locally before contributing dependency changes:

```bash
cargo install cargo-deny --locked
cargo deny check
```

`cargo-deny` is also run in CI; see `deny.toml` for the licence
allowlist and source policy.
