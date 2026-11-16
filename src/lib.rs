// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-buildroot: buildroot component model.
//!
//! Real implementation: buildroot_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildrootItem {
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
pub enum BuildrootError {
    EmptyBuildrootItemName,
    DuplicateBuildrootItem(String),
    EmptyEventSubject,
}

impl fmt::Display for BuildrootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBuildrootItemName => write!(f, "buildroot_item name must not be empty"),
            Self::DuplicateBuildrootItem(id) => write!(f, "duplicate buildroot_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BuildrootError {}

pub struct Buildroot {
    pub name: String,
    buildroot_items: BTreeMap<String, BuildrootItem>,
    events: BTreeMap<String, Event>,
}

impl Buildroot {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            buildroot_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_buildroot_item(&mut self, item: BuildrootItem) -> Result<(), BuildrootError> {
        if item.name.trim().is_empty() {
            return Err(BuildrootError::EmptyBuildrootItemName);
        }
        let id = item.id.clone();
        if self.buildroot_items.contains_key(&id) {
            return Err(BuildrootError::DuplicateBuildrootItem(id.clone()));
        }
        self.buildroot_items.insert(id, item);
        Ok(())
    }

    pub fn buildroot_item_count(&self) -> usize {
        self.buildroot_items.len()
    }

    pub fn get_buildroot_item(&self, id: &str) -> Option<&BuildrootItem> {
        self.buildroot_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BuildrootError> {
        if event.subject.trim().is_empty() {
            return Err(BuildrootError::EmptyEventSubject);
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

    fn sample_buildroot_item(id: &str, name: &str) -> BuildrootItem {
        BuildrootItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Buildroot::new("Buildroot");
        assert_eq!(m.name, "Buildroot");
        assert_eq!(m.buildroot_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_buildroot_item() {
        let mut m = Buildroot::new("Buildroot");
        m.add_buildroot_item(sample_buildroot_item("a1", "Alpha")).unwrap();
        assert_eq!(m.buildroot_item_count(), 1);
        assert!(m.get_buildroot_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Buildroot::new("Buildroot");
        assert!(m.add_buildroot_item(sample_buildroot_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Buildroot::new("Buildroot");
        m.add_buildroot_item(sample_buildroot_item("x", "One")).unwrap();
        assert!(m.add_buildroot_item(sample_buildroot_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Buildroot::new("Buildroot");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
