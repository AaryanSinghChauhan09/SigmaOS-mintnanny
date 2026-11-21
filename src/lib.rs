// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mintnanny: mintnanny component model.
//!
//! Real implementation: mintnanny_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintnannyItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintnannyError {
    EmptyMintnannyItemName,
    DuplicateMintnannyItem(String),
    EmptyEventSubject,
}

impl fmt::Display for MintnannyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMintnannyItemName => write!(f, "mintnanny_item name must not be empty"),
            Self::DuplicateMintnannyItem(id) => write!(f, "duplicate mintnanny_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintnannyError {}

pub struct Mintnanny {
    pub name: String,
    mintnanny_items: BTreeMap<String, MintnannyItem>,
    events: BTreeMap<String, Event>,
}

impl Mintnanny {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mintnanny_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_mintnanny_item(&mut self, item: MintnannyItem) -> Result<(), MintnannyError> {
        if item.name.trim().is_empty() {
            return Err(MintnannyError::EmptyMintnannyItemName);
        }
        let id = item.id.clone();
        if self.mintnanny_items.contains_key(&id) {
            return Err(MintnannyError::DuplicateMintnannyItem(id.clone()));
        }
        self.mintnanny_items.insert(id, item);
        Ok(())
    }

    pub fn mintnanny_item_count(&self) -> usize {
        self.mintnanny_items.len()
    }

    pub fn get_mintnanny_item(&self, id: &str) -> Option<&MintnannyItem> {
        self.mintnanny_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintnannyError> {
        if event.subject.trim().is_empty() {
            return Err(MintnannyError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mintnanny_item(id: &str, name: &str) -> MintnannyItem {
        MintnannyItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Mintnanny::new("Mintnanny");
        assert_eq!(m.name, "Mintnanny");
        assert_eq!(m.mintnanny_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_mintnanny_item() {
        let mut m = Mintnanny::new("Mintnanny");
        m.add_mintnanny_item(sample_mintnanny_item("a1", "Alpha")).unwrap();
        assert_eq!(m.mintnanny_item_count(), 1);
        assert!(m.get_mintnanny_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Mintnanny::new("Mintnanny");
        assert!(m.add_mintnanny_item(sample_mintnanny_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Mintnanny::new("Mintnanny");
        m.add_mintnanny_item(sample_mintnanny_item("x", "One")).unwrap();
        assert!(m.add_mintnanny_item(sample_mintnanny_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Mintnanny::new("Mintnanny");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

/// Content filtering / screen-time nanny (extracted from SigmaOS-system).
pub mod sigmananny;
