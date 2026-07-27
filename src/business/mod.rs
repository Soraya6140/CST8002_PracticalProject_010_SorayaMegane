// =============================================================================
// File:       src/business/mod.rs
// Author:     Soraya Megane Kaji
// Course:     CST8002 – Programming Language Research Project
// Professor:  Stanley Pieda
// Assessment: Practical Project Part 3
//
// Dataset: Environment and Climate Change Canada. (Aug 28, 2018).
//   Mercury concentrations in the Canadian Arctic marine ecosystem.
//   [Online]. Available:
//   https://open.canada.ca/data/en/dataset/d4285538-afa5-4644-931a-36d6ab6e25a1
//   Licensed under the Open Government Licence – Canada.
//
// References:
// [1] The Rust Foundation. (2026, Jul. 8). std::vec::Vec::sort_by. The Rust
//     Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
//     [Accessed: Jul. 8, 2026].
// [2] The Rust Foundation. (2026, Jul. 8). Structs: Defining and
//     Instantiating Structs. The Rust Programming Language Book. [Online].
//     Available: https://doc.rust-lang.org/book/ch05-00-structs.html
//     [Accessed: Jul. 8, 2026].
// [3] The Rust Foundation. (2026, Jul. 8). How to Write Tests. The Rust
//     Programming Language Book. [Online]. Available:
//     https://doc.rust-lang.org/book/ch11-01-writing-tests.html
//     [Accessed: Jul. 8, 2026].
// =============================================================================

//! # Business Layer
//!
//! Manages the in-memory `Vec<MercuryRecord>` and provides all CRUD operations.
//! No console interaction and no File-IO in this layer.
//!
//! **New in Part 3:** [`BusinessLayer::sort`] delegates to
//! [`model::sort_records`] to sort the in-memory collection by a user-selected
//! dataset column, implementing the Algorithms advanced topic. The sort is
//! performed entirely in Rust code using the built-in `Vec::sort_by` API — not
//! via any SQL statement [1].

use crate::model::{MercuryRecord, SortColumn, sort_records};

/// Holds the in-memory collection and exposes all CRUD + sort operations.
pub struct BusinessLayer {
    /// The in-memory dynamic array of [`MercuryRecord`] objects.
    records: Vec<MercuryRecord>,
}

impl BusinessLayer {
    /// Creates a new, empty [`BusinessLayer`].
    pub fn new() -> Self {
        BusinessLayer { records: Vec::new() }
    }

    /// Replaces the in-memory data with a new collection of records.
    ///
    /// # Arguments
    ///
    /// * `records` – The new `Vec<MercuryRecord>` to store.
    pub fn load(&mut self, records: Vec<MercuryRecord>) {
        self.records = records;
    }

    /// Returns the total number of records currently in memory.
    pub fn count(&self) -> usize { self.records.len() }

    /// Returns an immutable reference to the full in-memory Vec.
    pub fn all(&self) -> &Vec<MercuryRecord> { &self.records }

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
    /// `true` if the update succeeded, `false` if the index was out of range.
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
    /// `true` if deletion succeeded, `false` if the index was out of range.
    pub fn delete(&mut self, index: usize) -> bool {
        if index == 0 || index > self.records.len() { return false; }
        self.records.remove(index - 1);
        true
    }

    /// Sorts the in-memory collection in-place by the specified dataset column.
    ///
    /// This method implements the **Algorithms (sorting)** advanced topic for
    /// Practical Project Part 3. It delegates to [`sort_records`] in the model
    /// layer, which uses Rust's built-in `Vec::sort_by` API with a closure
    /// selecting the comparison key by [`SortColumn`] variant. The sort is
    /// stable and ascending. No SQL statement is used [1].
    ///
    /// # Arguments
    ///
    /// * `column` – The [`SortColumn`] that determines the comparison key.
    ///
    /// # Examples
    ///
    /// ```
    /// bl.sort(&SortColumn::Year);
    /// // in-memory Vec is now sorted by the Year column, ascending.
    /// ```
    pub fn sort(&mut self, column: &SortColumn) {
        sort_records(&mut self.records, column);
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

/// Unit tests for [`BusinessLayer`] CRUD and sort operations.
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: builds a [`MercuryRecord`] with controlled field values.
    #[allow(non_snake_case)]
    fn make_record(SiteName: &str, Year: &str) -> MercuryRecord {
        MercuryRecord::new(
            SiteName.to_string(), "1".to_string(), Year.to_string(),
            "10.0".to_string(), "50.0".to_string(), "0.0".to_string(),
        )
    }

    /// Verifies that [`BusinessLayer::create`] increases the record count.
    /// This test PASSES.
    #[test]
    fn create_adds_record() {
        let mut bl = BusinessLayer::new();
        assert_eq!(bl.count(), 0);
        bl.create(make_record("Alpha Site", "2005"));
        assert_eq!(bl.count(), 1, "count should be 1 after adding one record");
    }

    /// Verifies that [`BusinessLayer::delete`] decreases the record count.
    /// This test PASSES.
    #[test]
    fn delete_removes_record() {
        let mut bl = BusinessLayer::new();
        bl.create(make_record("Alpha Site", "2005"));
        let ok = bl.delete(1);
        assert!(ok, "delete should return true for a valid index");
        assert_eq!(bl.count(), 0);
    }

    /// Verifies that [`BusinessLayer::sort`] by SiteName orders records
    /// alphabetically. This test PASSES.
    #[test]
    fn sort_by_site_name_is_alphabetical() {
        let mut bl = BusinessLayer::new();
        bl.create(make_record("Zeta Site",  "2005"));
        bl.create(make_record("Alpha Site", "2005"));
        bl.create(make_record("Mu Site",    "2005"));
        bl.sort(&SortColumn::SiteName);
        assert_eq!(bl.get(1).unwrap().SiteName, "Alpha Site",
            "first record after sort by SiteName should be Alpha Site");
        assert_eq!(bl.get(3).unwrap().SiteName, "Zeta Site",
            "last record after sort by SiteName should be Zeta Site");
    }

    /// Verifies that [`BusinessLayer::sort`] by Year orders records
    /// chronologically. This test PASSES.
    #[test]
    fn sort_by_year_is_chronological() {
        let mut bl = BusinessLayer::new();
        bl.create(make_record("Site A", "2007"));
        bl.create(make_record("Site B", "2005"));
        bl.create(make_record("Site C", "2006"));
        bl.sort(&SortColumn::Year);
        assert_eq!(bl.get(1).unwrap().Year, "2005");
        assert_eq!(bl.get(2).unwrap().Year, "2006");
        assert_eq!(bl.get(3).unwrap().Year, "2007");
    }
}
