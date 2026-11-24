# buffmyagent-open

Open-source parts of BuffMyAgent: the credential broker core and the desktop helper.

The broker lets an AI agent call an API or a database without ever holding the secret. It resolves the credential, checks that the request goes only to the host the credential is bound to, and makes the call itself. The code that touches credentials is public so that anyone can read it.

Status: bootstrap. The workspace, the gate and the shared types exist. The broker, the vault and the binaries arrive crate by crate.

## Start

```sh
bin/setup    # pinned Rust toolchain, cargo-deny, gitleaks, dependencies, git hooks
bin/check    # the gate: format, clippy, cargo deny, tests, vectors hash
```

`bin/setup` needs [rustup](https://rustup.rs) and [mise](https://mise.jdx.dev).

## Layout

| Path | Holds |
|---|---|
| `crates/bma-types` | Identifiers and the refusal code list shared by every crate |
| `bins/` | Binaries, none yet |
| `vectors/` | The canonical request vectors every implementation must reproduce, copied by hash |
| `tests/workspace_policy.rs` | Rules over the whole workspace |

## Contributing

Read [CLAUDE.md](CLAUDE.md): it is the contract for every change, by a person or an agent. Report a vulnerability privately, as [SECURITY.md](SECURITY.md) describes.

## License

[MIT](LICENSE)
