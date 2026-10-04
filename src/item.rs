//! Items and their categories.
//!
//! Categories are what federation import/export policies operate on, so they
//! are part of the stable core vocabulary.

use crate::id::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategory {
    RawMaterial,
    Food,
    Tool,
    Weapon,
    Clothing,
    Cosmetic,
    Building,
    Currency,
    QuestItem,
    Legendary,
}

/// A concrete item instance in the world.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    /// Item kind, namespaced: `core:iron_axe`, `svet-a:dragon_axe`.
    pub kind: String,
    pub category: ItemCategory,
    /// 0.0 = broken, 1.0 = new.
    pub durability: f32,
    /// Fallback core kind for worlds that do not know a custom `kind`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
}

impl Item {
    /// True if `kind` belongs to the shared `core:` namespace understood by
    /// every compliant world.
    pub fn is_core_kind(&self) -> bool {
        self.kind.starts_with("core:")
    }
}
