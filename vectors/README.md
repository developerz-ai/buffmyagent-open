# Canonical request vectors

`canonical-request.json` is the contract between every implementation of the canonical request: each one must produce the same bytes and the same hash for every case in it.

The file is generated elsewhere and copied here unchanged. `canonical-request.sha256` pins the copy.

| Rule | Why |
|---|---|
| The copy is never edited here | An edit would make this implementation agree with itself and with nothing else |
| `bin/vectors --check` runs in the gate | A hand edit or a stale hash fails `bin/check` |
| A new copy comes through `bin/vectors <checkout>` | It writes the file and the hash together |
