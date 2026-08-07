// =============================================================================
// File:       src/business/mod.rs
// Author:     Soraya Megane Kaji
// Course:     CST8002 – Programming Language Research Project
// Professor:  Stanley Pieda
// Assessment: Practical Project Part 4
//
// Dataset: Environment and Climate Change Canada. (Aug 28, 2018).
//   Mercury concentrations in the Canadian Arctic marine ecosystem.
//   [Online]. Available:
//   https://open.canada.ca/data/en/dataset/d4285538-afa5-4644-931a-36d6ab6e25a1
//   Licensed under the Open Government Licence – Canada.
//
// References:
// [1] The Rust Foundation. (2026, Jul. 28). Iterator::filter – Standard
//     library iterator adaptor. The Rust Standard Library. [Online].
//     Available: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
//     [Accessed: Jul. 28, 2026].
// [2] The Rust Foundation. (2026, Jul. 28). std::vec::Vec::sort_by. The Rust
//     Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
//     [Accessed: Jul. 28, 2026].
// [3] The Rust Foundation. (2026, Jul. 28). How to Write Tests. The Rust
//     Programming Language Book. [Online]. Available:
//     https://doc.rust-lang.org/book/ch11-01-writing-tests.html
//     [Accessed: Jul. 28, 2026].
// =============================================================================

//! # Business Layer
//!
//! Manages the in-memory `Vec<MercuryRecord>` and provides all CRUD, sort
//! (PP3), and search/filter (PP4) operations. No console I/O or File-IO.
//!
//! **New in PP4:** [`BusinessLayer::search`] delegates to
//! [`model::filter_records`] to filter the in-memory collection by multiple
//! dataset columns simultaneously using `Iterator::filter()` [1].

use crate::model::{MercuryRecord, SearchCriteria, SortColumn,
                   filter_records, sort_records};

/// Manages the in-memory collection and exposes CRUD, sort, and search.
pub struct BusinessLayer {
    /// The in-memory dynamic array of [`MercuryRecord`] objects.
    records: Vec<MercuryRecord>,
}

impl BusinessLayer {
    /// Creates a new, empty [`BusinessLayer`].
    pub fn new() -> Self { BusinessLayer { records: Vec::new() } }

    /// Replaces the in-memory data with a new collection.
    pub fn load(&mut self, records: Vec<MercuryRecord>) { self.records = records; }

    /// Returns the total number of records currently in memory.
    pub fn count(&self) -> usize { self.records.len() }

    /// Returns an immutable reference to the full in-memory Vec.
    pub fn all(&self) -> &Vec<MercuryRecord> { &self.records }

    /// Returns a reference to one record by 1-based index.
    ///
    /// # Returns
    ///
    /// `Some(&MercuryRecord)` if valid, `None` if out of range.
    pub fn get(&self, index: usize) -> Option<&MercuryRecord> {
        if index == 0 || index > self.records.len() { return None; }
        Some(&self.records[index - 1])
    }

    /// Adds a new record to the end of the in-memory Vec.
    pub fn create(&mut self, record: MercuryRecord) { self.records.push(record); }

    /// Replaces the record at the given 1-based index.
    ///
    /// # Returns
    ///
    /// `true` if succeeded, `false` if index out of range.
    pub fn update(&mut self, index: usize, record: MercuryRecord) -> bool {
        if index == 0 || index > self.records.len() { return false; }
        self.records[index - 1] = record;
        true
    }

    /// Removes the record at the given 1-based index.
    ///
    /// # Returns
    ///
    /// `true` if succeeded, `false` if index out of range.
    pub fn delete(&mut self, index: usize) -> bool {
        if index == 0 || index > self.records.len() { return false; }
        self.records.remove(index - 1);
        true
    }

    /// Sorts the in-memory collection in-place by the specified column.
    /// PP3 Algorithms feature — delegates to `model::sort_records` [2].
    ///
    /// # Arguments
    ///
    /// * `column` – The [`SortColumn`] to sort by.
    pub fn sort(&mut self, column: &SortColumn) {
        sort_records(&mut self.records, column);
    }

    /// Filters the in-memory collection by the given search criteria and
    /// returns matching records as a Vec of references.
    ///
    /// **PP4 Novel Feature — Search/Filter on multiple columns.**
    /// Delegates to `model::filter_records` which uses `Iterator::filter()`
    /// with chained AND conditions. The original Vec is unchanged [1].
    ///
    /// # Arguments
    ///
    /// * `criteria` – The [`SearchCriteria`] defining which columns to filter.
    ///
    /// # Returns
    ///
    /// A `Vec<&MercuryRecord>` of records matching all criteria.
    pub fn search<'a>(&'a self, criteria: &SearchCriteria)
        -> Vec<&'a MercuryRecord>
    {
        filter_records(&self.records, criteria)
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

/// Unit tests for BusinessLayer CRUD, sort, and search operations.
#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    fn make(SiteName: &str, Year: &str, THg: &str) -> MercuryRecord {
        MercuryRecord::new(
            SiteName.to_string(), "1".to_string(), Year.to_string(),
            "10.0".to_string(), THg.to_string(), "0.0".to_string(),
        )
    }

    /// Verifies that create() adds a record. PASSES.
    #[test]
    fn create_adds_record() {
        let mut bl = BusinessLayer::new();
        bl.create(make("Alpha", "2005", "50.0"));
        assert_eq!(bl.count(), 1);
    }

    /// Verifies that delete() removes a record. PASSES.
    #[test]
    fn delete_removes_record() {
        let mut bl = BusinessLayer::new();
        bl.create(make("Alpha", "2005", "50.0"));
        assert!(bl.delete(1));
        assert_eq!(bl.count(), 0);
    }

    /// Verifies sort by SiteName is alphabetical. PASSES.
    #[test]
    fn sort_by_site_name_alphabetical() {
        let mut bl = BusinessLayer::new();
        bl.create(make("Zeta",  "2005", "50.0"));
        bl.create(make("Alpha", "2005", "50.0"));
        bl.sort(&SortColumn::SiteName);
        assert_eq!(bl.get(1).unwrap().SiteName, "Alpha");
    }

    /// Verifies search() returns only matching records. PASSES.
    #[test]
    fn search_filters_by_site_name() {
        let mut bl = BusinessLayer::new();
        bl.create(make("North Open Polynya", "2005", "71.0"));
        bl.create(make("South Bay",          "2005", "30.0"));
        let criteria = SearchCriteria::new(
            Some("North".to_string()), None, None
        );
        let results = bl.search(&criteria);
        assert_eq!(results.len(), 1);
        assert!(results[0].SiteName.contains("North"));
    }
}
