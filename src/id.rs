//! Federated identifiers.
//!
//! * A server is identified by its DNS name, e.g. `svet-a.example.org`.
//! * A player is `@localpart:server`, in the style of Matrix.
//! * An item is `server/uuidv7`; the server part names the world that
//!   minted the item and vouches for it.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IdError {
    #[error("invalid server name: {0}")]
    ServerName(String),
    #[error("invalid player id: {0}")]
    PlayerId(String),
    #[error("invalid item id: {0}")]
    ItemId(String),
}

/// DNS name of a world server.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ServerName(String);

impl ServerName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ServerName {
    type Error = IdError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let valid = !s.is_empty()
            && s.len() <= 253
            && s.split('.').all(|label| {
                !label.is_empty()
                    && label.len() <= 63
                    && !label.starts_with('-')
                    && !label.ends_with('-')
                    && label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
            });
        if valid {
            Ok(ServerName(s.to_ascii_lowercase()))
        } else {
            Err(IdError::ServerName(s))
        }
    }
}

impl From<ServerName> for String {
    fn from(s: ServerName) -> String {
        s.0
    }
}

impl FromStr for ServerName {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ServerName::try_from(s.to_string())
    }
}

impl fmt::Display for ServerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Federated player identity: `@localpart:home.server`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PlayerId {
    pub localpart: String,
    pub home: ServerName,
}

impl TryFrom<String> for PlayerId {
    type Error = IdError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let rest = s
            .strip_prefix('@')
            .ok_or_else(|| IdError::PlayerId(s.clone()))?;
        let (local, server) = rest
            .split_once(':')
            .ok_or_else(|| IdError::PlayerId(s.clone()))?;
        let local_ok = !local.is_empty()
            && local
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "._-".contains(c));
        if !local_ok {
            return Err(IdError::PlayerId(s));
        }
        let home = server.parse().map_err(|_| IdError::PlayerId(s.clone()))?;
        Ok(PlayerId {
            localpart: local.to_string(),
            home,
        })
    }
}

impl From<PlayerId> for String {
    fn from(p: PlayerId) -> String {
        p.to_string()
    }
}

impl FromStr for PlayerId {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PlayerId::try_from(s.to_string())
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}:{}", self.localpart, self.home)
    }
}

/// Globally unique item id: `origin.server/uuidv7`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ItemId {
    pub origin: ServerName,
    pub uuid: Uuid,
}

impl ItemId {
    /// Mint a new item id on the given origin server.
    pub fn mint(origin: ServerName) -> Self {
        ItemId {
            origin,
            uuid: Uuid::now_v7(),
        }
    }
}

impl TryFrom<String> for ItemId {
    type Error = IdError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let (server, uuid) = s
            .split_once('/')
            .ok_or_else(|| IdError::ItemId(s.clone()))?;
        let origin = server.parse().map_err(|_| IdError::ItemId(s.clone()))?;
        let uuid = Uuid::parse_str(uuid).map_err(|_| IdError::ItemId(s.clone()))?;
        Ok(ItemId { origin, uuid })
    }
}

impl From<ItemId> for String {
    fn from(i: ItemId) -> String {
        i.to_string()
    }
}

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.origin, self.uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_id_roundtrip() {
        let p: PlayerId = "@vitex:svet-a.example.org".parse().unwrap();
        assert_eq!(p.localpart, "vitex");
        assert_eq!(p.home.as_str(), "svet-a.example.org");
        assert_eq!(p.to_string(), "@vitex:svet-a.example.org");
    }

    #[test]
    fn player_id_rejects_garbage() {
        assert!("vitex:svet.org".parse::<PlayerId>().is_err());
        assert!("@Vitex:svet.org".parse::<PlayerId>().is_err());
        assert!("@vitex:-bad.org".parse::<PlayerId>().is_err());
    }

    #[test]
    fn item_id_roundtrip_serde() {
        let id = ItemId::mint("svet-a.example.org".parse().unwrap());
        let json = serde_json::to_string(&id).unwrap();
        let back: ItemId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
        assert_eq!(back.uuid.get_version_num(), 7);
    }
}
