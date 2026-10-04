# ishtaria-core

Shared Rust core of Ishtaria: federated identifiers (`@user:server`, `server/uuidv7` item ids), item categories and ruleset versions. Linked into the server, the generator and – through GDExtension – the Godot client, so client prediction and the server share one implementation of the rules.

```sh
cargo test
```

Not packaged on its own; it is built into the packages that use it.

License: MIT

## Part of Ishtaria

Ishtaria is an open-source, persistent, federated virtual planet of Earth size.
Documentation: https://vitexsoftware.github.io/ishtaria-docs/ · All repositories: https://github.com/VitexSoftware?q=ishtaria
