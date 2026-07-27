// =============================================================================
// File:       src/business/mod.rs
// Author:     Soraya Megane Kaji
// Course:     CST8002 – Programming Language Research Project
// Professor:  Stanley Pieda
// Assessment: Practical Project Part 2
//
// Dataset: Environment and Climate Change Canada. (Aug 28, 2018).
//   Mercury concentrations in the Canadian Arctic marine ecosystem.
//   [Online]. Available:
//   https://open.canada.ca/data/en/dataset/d4285538-afa5-4644-931a-36d6ab6e25a1
//   Licensed under the Open Government Licence – Canada.
//
// References:
// [1] The Rust Foundation, "Structs," The Rust Programming Language Book.
//     [Online]. Available:
//     https://doc.rust-lang.org/book/ch05-00-structs.html.
//     [Accessed: May 2026].
// [2] The Rust Foundation, "std::vec::Vec," The Rust Standard Library.
//     [Online]. Available:
//     https://doc.rust-lang.org/std/vec/struct.Vec.html.
//     [Accessed: May 2026].
// [3] K. Fakhroutdinov. (2009-2024). "Multi-Layered Application: UML Model
//     Diagram Example." uml-diagrams.org. [Online]. Available:
//     https://www.uml-diagrams.org/multi-layered-application-uml-model-diagram-example.html.
//     [Accessed: May 2026].
// =============================================================================

//! # Business Layer
//!
//! Manages the in-memory `Vec<MercuryRecord>` data structure and provides
//! all CRUD (Create, Read, Update, Delete) operations. This layer has no
//! console interaction and no File-IO — it depends only on the Model layer.

use crate::model::MercuryRecord;

/// Holds the in-memory collection of [`MercuryRecord`] objects and exposes
/// all business-layer operations on that collection.
pub struct BusinessLayer {
    /// The sequential data structure holding all in-memory records.
    records: Vec<MercuryRecord>,
}

impl BusinessLayer {
    /// Creates a new, empty [`BusinessLayer`].
    ///
    /// # Returns
    ///
    /// A [`BusinessLayer`] instance with an empty `records` Vec.
    pub fn new() -> Self {
        BusinessLayer { records: Vec::new() }
    }

    /// Replaces the in-memory data with a new collection of records.
    ///
    /// Used both on startup (to load from the dataset) and when the user
    /// selects the "Reload data" menu option.
    ///
    /// # Arguments
    ///
    /// * `records` – The new `Vec<MercuryRecord>` to store in memory.
    pub fn load(&mut self, records: Vec<MercuryRecord>) {
        self.records = records;
    }

    /// Returns the total number of records currently in memory.
    pub fn count(&self) -> usize {
        self.records.len()
    }

    /// Returns an immutable reference to the full in-memory Vec.
    pub fn all(&self) -> &Vec<MercuryRecord> {
        &self.records
    }

    /// Returns an immutable reference to a single record by 1-based index.
    ///
    /// # Arguments
    ///
    /// * `index` – 1-based position of the record.
    ///
    /// # Returns
    ///
    /// `Some(&MercuryRecord)` if the index is valid, `None` otherwise.
    pub fn get(&self, index: usize) -> Option<&MercuryRecord> {
        if index == 0 || index > self.records.len() { return None; }
        Some(&self.records[index - 1])
    }

    /// Adds a new [`MercuryRecord`] to the end of the in-memory Vec.
    ///
    /// # Arguments
    ///
    /// * `record` – The record to add.
    pub fn create(&mut self, record: MercuryRecord) {
        self.records.push(record);
    }

    /// Replaces the record at the given 1-based index with an updated record.
    ///
    /// # Arguments
    ///
    /// * `index`  – 1-based position of the record to update.
    /// * `record` – The replacement [`MercuryRecord`].
    ///
    /// # Returns
    ///
    /// `true` if the update succeeded, `false` if the index was invalid.
    pub fn update(&mut self, index: usize, record: MercuryRecord) -> bool {
        if index == 0 || index > self.records.len() { return false; }
        self.records[index - 1] = record;
        true
    }

    /// Removes the record at the given 1-based index from the in-memory Vec.
    ///
    /// # Arguments
    ///
    /// * `index` – 1-based position of the record to delete.
    ///
    /// # Returns
    ///
    /// `true` if deletion succeeded, `false` if the index was invalid.
    pub fn delete(&mut self, index: usize) -> bool {
        if index == 0 || index > self.records.len() { return false; }
        self.records.remove(index - 1);
        true
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

/// Unit tests for [`BusinessLayer`] CRUD operations.
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that creates a sample [`MercuryRecord`] for use in tests.
    #[allow(non_snake_case)]
    fn sample_record() -> MercuryRecord {
        MercuryRecord::new(
            "North Open Polynya".to_string(),
            "1".to_string(),
            "2005".to_string(),
            "76.3".to_string(),
            "71.407".to_string(),
            "0".to_string(),
        )
    }

    /// Verifies that [`BusinessLayer::create`] correctly adds a record to
    /// the in-memory Vec, increasing the count by 1. This test PASSES.
    #[test]
    fn create_adds_record_to_vec() {
        let mut bl = BusinessLayer::new();
        assert_eq!(bl.count(), 0);
        bl.create(sample_record());
        assert_eq!(bl.count(), 1, "count should be 1 after adding one record");
    }

    /// Verifies that [`BusinessLayer::delete`] correctly removes a record
    /// from the in-memory Vec, decreasing the count by 1. This test PASSES.
    #[test]
    fn delete_removes_record_from_vec() {
        let mut bl = BusinessLayer::new();
        bl.create(sample_record());
        assert_eq!(bl.count(), 1);
        let removed = bl.delete(1);
        assert!(removed, "delete should return true for a valid index");
        assert_eq!(bl.count(), 0, "count should be 0 after deleting the only record");
    }
}
