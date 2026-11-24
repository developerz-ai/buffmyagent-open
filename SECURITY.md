# Security policy

This repository holds the code that touches credentials: the broker, the vault format and the helper. Reports are welcome and are handled privately.

## Reporting a vulnerability

Do not open a public issue for a vulnerability.

Report it privately, either way:

- Email **security@developerz.ai**.
- Or use GitHub's "Report a vulnerability" button under the Security tab of this repository.

Include what you found, how to reproduce it, the commit or version, and proof-of-concept code if it is safe to share. Never include a real credential: use a made-up one.

## What to expect

| Step | When |
|---|---|
| We confirm we received the report | Within 2 working days |
| We tell you whether we can reproduce it and how severe we think it is | Within 7 days |
| We agree a disclosure date with you | Before any fix is public |
| We credit you in the advisory, if you want | At disclosure |

## In scope

- A secret sent to a host it is not bound to, by any route: redirect, DNS answer, proxy, URL parsing difference, header smuggling.
- A secret in a log, an error, a `Debug` or `Display` output, a crash report or a response body.
- A request to a private, loopback, link-local or cloud metadata address from the cloud side.
- A bypass of a read-only SQL guard, or of a statement limit.
- A weakness in the vault format, key derivation or any use of a cryptographic primitive.
- A bypass of an approval, a signature check or a pinned key in the helper.
- Memory that holds a secret and is not cleared.

## Out of scope

- A vulnerability in a dependency with no way to reach it through this code. Report it to that project.
- A report that needs an attacker who already controls the machine the helper runs on, as the same user.
- Denial of service that needs more traffic than a published limit allows.
- Social engineering.

## Supported versions

Until the first release, only `main` is supported.
