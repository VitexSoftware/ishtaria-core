# ishtaria-core

Shared Rust types: identities, items, ruleset versions. Licence: MIT. Part of the Ishtaria workspace of several repositories side by side
(`ishtaria-server`, `ishtaria-client`, `ishtaria-worldgen`, `ishtaria-core`, `ishtaria-content`, `ishtaria-protocol`,
`ishtaria-docs`): run Git, Cargo and `make` inside the repository you change. Work on `main`; do not commit,
push or deploy unless asked. Debian/Ubuntu x86-64 is the only supported platform.

Developer guide: https://github.com/VitexSoftware/ishtaria-docs/tree/main/source/development (`contributing`, `local-setup`, `invariants`, `cookbook`, the code tours).

## Checks before you say you are done

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Where things are

* `src/` the library. It is linked by the server (from Git `main`; for local changes use the uncommitted Cargo path override described in the server README).

## Working notes

* A change to a public type is a change to a shared contract: update the consumers and say so in the pull request.

## Rules that never change (full text: docs `development/invariants`)

* The server decides; clients send intentions. Never trust a client value; validate every request and every answer.
* Related writes in one transaction; economy changes are atomic, bounded and repeat-safe.
* Migrations are append-only. Terrain is generated; only changes are stored. Never touch a live database in tests.
* A new character gets 100 gold exactly once. Death is permanent. Money buys space and appearance, not power.
* Language is a client preference: the server may receive it per request but never stores it.
* Content is data, not code. Federation is bilateral and signed.
* Never log or commit secrets. Argon2id passwords, hashed expiring tokens.
* Approved assets (Kenney, Quaternius, generated with the origin recorded); keep licence notices.
* Report honestly: what is implemented and tested, what is planned, what is blocked. Do not weaken a failing check.
