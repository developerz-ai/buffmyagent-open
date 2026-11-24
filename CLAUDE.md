# buffmyagent-open

The open-source parts of BuffMyAgent: the credential broker core, the cloud sidecar and the desktop helper. One Rust workspace. The broker lets an agent reach an API or a database without ever holding the secret: it resolves the credential, checks that the request goes only to the host the credential is bound to, and makes the call itself.

Status: bootstrap. The workspace, the gate and one crate (`bma-types`) exist. The broker, the vault and the binaries arrive crate by crate.

This file is the contract for anyone, person or agent, who changes this repository. `AGENTS.md` points here.

## Commands

| Task | Command |
|---|---|
| Set up a fresh clone | `bin/setup` |
| The gate, the same one CI runs | `bin/check` |
| Tests only | `bin/test` (arguments go to `cargo test`) |
| Inner loop | `bin/dev` (type-checks every crate and test; starts the sidecar once it exists) |
| Vectors hash | `bin/vectors --check` |
| Secret scan | `bin/secrets` |

`bin/check` runs `cargo fmt --check`, `cargo clippy` with warnings as errors, `cargo deny check`, `cargo test --workspace --locked` and `bin/vectors --check`. Run it before every commit. Exit code 75 means the environment cannot run the gate: run `bin/setup`.

## Structure

| Path | Holds |
|---|---|
| `Cargo.toml` | The workspace: members by glob, shared dependency versions, shared lints, the release profile |
| `crates/*` | Libraries. `bma-types`: identifiers and the refusal code list |
| `bins/*` | Binaries. None yet |
| `tests/workspace_policy.rs` | Rules over the whole workspace |
| `vectors/` | The canonical request vectors, copied by hash |
| `deny.toml` | Licenses, advisories, banned crates, sources |
| `bin/` | The commands above |
| `.githooks/pre-push` | The secret scan, before anything is published |

A new crate goes under `crates/` or `bins/`. It inherits `edition`, `license`, `rust-version` and `[lints]` from the workspace and takes dependencies with `name = { workspace = true }`. It does not edit the root `Cargo.toml`.

## Security rules

These hold in every crate. A change that breaks one is refused in review, whatever else it does.

1. No secret in any `Debug`, `Display`, log field or error. A type that holds a secret prints a placeholder. A refused input is not echoed back.
2. Parse a URL once and use the parsed value. Never check one string and connect with another.
3. No HTTP client other than `bma-broker`'s. `reqwest` is banned.
4. No `std::env` reads outside `bins/`. A library takes its configuration as arguments.
5. Every refusal maps to a `Code` in `bma-types`. No free-text refusal.
6. One TLS stack: rustls with the bundled webpki roots. `openssl`, `native-tls` and the system certificate store are banned.
7. `unsafe` is forbidden. A crate that needs it for memory locking opts out by name in its own manifest, with a comment that says why.

## Conventions

- Rust edition 2024. The toolchain is pinned in `rust-toolchain.toml` and bumped on purpose, in its own pull request.
- No file under `crates/` is longer than 300 lines. Split by topic before it gets there.
- `unwrap`, `expect`, `panic!`, `println!` and `dbg!` are denied outside tests. Return an error.
- Errors are `thiserror` enums. Async is tokio, with named features only. The HTTP server is axum.
- A test comes first and fails first. A rule is proven on a planted case before it is run on the real tree.
- Actions in workflows are pinned by full commit id.
- `bma-types::Code` holds the full list. A new code is its own small pull request.
- `vectors/canonical-request.json` is never edited here. It is copied with `bin/vectors` and pinned by `vectors/canonical-request.sha256`.

## This repository is public

- History is public from the first push and stays public. Never commit a secret, a real hostname, a customer name or a private document. Test fixtures use `example.com` and plain words.
- `bin/secrets` (gitleaks) runs before every push and in CI. A finding blocks the push. Do not bypass the hook.
- Two status checks are required on `main`: `ci`, from this repository's workflow, and `public-repo-scan`, posted by the maintainers' own scan. The scan can take a few minutes to post after a push. On a pull request from a fork it stays pending until a maintainer approves that exact commit.
- Pull requests merge by squash only. The scan reads the tip of a pull request, so nothing but the tip may reach `main`.
- Text about larger customers says Contact us. Name no company.
