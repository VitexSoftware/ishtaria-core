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
