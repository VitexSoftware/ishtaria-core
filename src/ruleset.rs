//! Ruleset versions (`core-rules@1.4`).
//!
//! Two worlds can federate only if their rulesets share a major version:
//! item semantics (recipes, categories, fallbacks) must agree.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RulesetVersion {
    pub name: String,
    pub major: u32,
    pub minor: u32,
}

impl RulesetVersion {
    pub fn is_compatible_with(&self, other: &RulesetVersion) -> bool {
        self.name == other.name && self.major == other.major
    }
}

impl FromStr for RulesetVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, ver) = s.split_once('@').ok_or("missing '@'")?;
        let (major, minor) = ver.split_once('.').ok_or("missing '.'")?;
        Ok(RulesetVersion {
            name: name.to_string(),
            major: major.parse().map_err(|_| "bad major")?,
            minor: minor.parse().map_err(|_| "bad minor")?,
        })
    }
}

impl fmt::Display for RulesetVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}.{}", self.name, self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatibility_by_major() {
        let a: RulesetVersion = "core-rules@1.4".parse().unwrap();
        let b: RulesetVersion = "core-rules@1.9".parse().unwrap();
        let c: RulesetVersion = "core-rules@2.0".parse().unwrap();
        assert!(a.is_compatible_with(&b));
        assert!(!a.is_compatible_with(&c));
    }
}
