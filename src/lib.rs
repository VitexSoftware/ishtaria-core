//! # ishtaria-core
//!
//! Shared simulation core of the Ishtaria federated world.
//!
//! This crate is compiled into the authoritative server and, through a Godot
//! GDExtension, into the client. Keeping rules in one place means client-side
//! prediction and the server always agree.

pub mod id;
pub mod item;
pub mod ruleset;

pub use id::{IdError, ItemId, PlayerId, ServerName};
pub use item::{Item, ItemCategory};
pub use ruleset::RulesetVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlayerPresence {
    pub uuid: uuid::Uuid,
    pub alive: bool,
}

pub fn validate_world_link(
    local: &[PlayerPresence],
    remote: &[PlayerPresence],
) -> Result<(), IdError> {
    let living: std::collections::HashSet<_> = local
        .iter()
        .filter(|player| player.alive)
        .map(|player| player.uuid)
        .collect();
    for player in remote.iter().filter(|player| player.alive) {
        if living.contains(&player.uuid) {
            return Err(IdError::LivingPlayerCollision(player.uuid));
        }
    }
    Ok(())
}

#[cfg(test)]
mod federation_tests {
    use super::*;

    #[test]
    fn world_link_rejects_only_colliding_living_players() {
        let first = uuid::Uuid::from_u128(1);
        let second = uuid::Uuid::from_u128(2);
        let local = [PlayerPresence {
            uuid: first,
            alive: true,
        }];
        assert_eq!(
            validate_world_link(&local, &local),
            Err(IdError::LivingPlayerCollision(first))
        );
        assert!(validate_world_link(
            &local,
            &[PlayerPresence {
                uuid: second,
                alive: true
            }]
        )
        .is_ok());
        assert!(validate_world_link(
            &local,
            &[PlayerPresence {
                uuid: first,
                alive: false
            }]
        )
        .is_ok());
        assert!(validate_world_link(
            &[PlayerPresence {
                uuid: first,
                alive: false
            }],
            &local
        )
        .is_ok());
        assert!(validate_world_link(&[], &local).is_ok());
    }
}
