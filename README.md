# ishtaria-core

<img src="https://raw.githubusercontent.com/VitexSoftware/ishtaria-client/main/assets/branding/emblem.png" alt="Ishtaria" width="96">

Shared Rust core of Ishtaria: federated identifiers (`@user:server`, `server/uuidv7` item ids), item categories and ruleset versions. Linked into the server, the generator and – through GDExtension – the Godot client, so client prediction and the server share one implementation of the rules.

```sh
cargo test
```

Not packaged on its own; it is built into the packages that use it.

`PlayerPresence` carries a permanent character UUID and whether that character
is alive. `validate_world_link` rejects a link when both worlds contain a living
character with the same UUID; dead memorials do not cause collisions. This is
a shared validation rule, not implemented federation transport. The server
currently uses core from Git; local core changes require the documented local
Cargo override. Worldgen and Godot integration with core remains planned.

License: MIT

## Part of Ishtaria

Ishtaria is an open-source, persistent, federated virtual planet of Earth size.
Documentation: https://vitexsoftware.github.io/ishtaria-docs/ · All repositories: https://github.com/VitexSoftware?q=ishtaria
