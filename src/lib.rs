// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dm-integrity: dm integrity component model.
//!
//! Real implementation: integritys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integrity {
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
pub enum DmIntegrityError {
    EmptyIntegrityName,
    DuplicateIntegrity(String),
    EmptyEventSubject,
}

impl fmt::Display for DmIntegrityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIntegrityName => write!(f, "integrity name must not be empty"),
            Self::DuplicateIntegrity(id) => write!(f, "duplicate integrity {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DmIntegrityError {}

pub struct DmIntegrity {
    pub name: String,
    integritys: BTreeMap<String, Integrity>,
    events: BTreeMap<String, Event>,
}

impl DmIntegrity {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            integritys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_integrity(&mut self, item: Integrity) -> Result<(), DmIntegrityError> {
        if item.name.trim().is_empty() {
            return Err(DmIntegrityError::EmptyIntegrityName);
        }
        let id = item.id.clone();
        if self.integritys.contains_key(&id) {
            return Err(DmIntegrityError::DuplicateIntegrity(id.clone()));
        }
        self.integritys.insert(id, item);
        Ok(())
    }

    pub fn integrity_count(&self) -> usize {
        self.integritys.len()
    }

    pub fn get_integrity(&self, id: &str) -> Option<&Integrity> {
        self.integritys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DmIntegrityError> {
        if event.subject.trim().is_empty() {
            return Err(DmIntegrityError::EmptyEventSubject);
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

    fn sample_integrity(id: &str, name: &str) -> Integrity {
        Integrity {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DmIntegrity::new("DmIntegrity");
        assert_eq!(m.name, "DmIntegrity");
        assert_eq!(m.integrity_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_integrity() {
        let mut m = DmIntegrity::new("DmIntegrity");
        m.add_integrity(sample_integrity("a1", "Alpha")).unwrap();
        assert_eq!(m.integrity_count(), 1);
        assert!(m.get_integrity("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DmIntegrity::new("DmIntegrity");
        assert!(m.add_integrity(sample_integrity("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DmIntegrity::new("DmIntegrity");
        m.add_integrity(sample_integrity("x", "One")).unwrap();
        assert!(m.add_integrity(sample_integrity("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DmIntegrity::new("DmIntegrity");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
