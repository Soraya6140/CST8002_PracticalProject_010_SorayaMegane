// =============================================================================
// File:       src/model/mod.rs
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
// [1] The Rust Foundation. (2026, Jul. 8). Structs: Defining and
//     Instantiating Structs. The Rust Programming Language Book. [Online].
//     Available: https://doc.rust-lang.org/book/ch05-00-structs.html
//     [Accessed: Jul. 8, 2026].
// [2] The Rust Foundation. (2026, Jul. 8). std::fmt::Display. The Rust
//     Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/fmt/trait.Display.html
//     [Accessed: Jul. 8, 2026].
// [3] The Rust Foundation. (2026, Jul. 8). std::vec::Vec::sort_by. The Rust
//     Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
//     [Accessed: Jul. 8, 2026].
// =============================================================================

//! # Model Layer
//!
//! Defines [`MercuryRecord`], the record-object (entity) for the Mercury
//! Concentrations in the Canadian Arctic Marine Ecosystem dataset. Field names
//! match the dataset column names exactly so the professor can verify this
//! project uses the correct dataset.
//!
//! New in Part 3: [`SortColumn`] enum and [`sort_records`] function implement
//! the Algorithms (sorting) advanced topic. The sort is performed entirely
//! within this model layer using Rust's built-in `sort_by` API, not via SQL.

use std::fmt;

/// Identifies which dataset column to sort [`MercuryRecord`] values by.
///
/// This enum is part of the Algorithms advanced topic implementation for
/// Practical Project Part 3. It is used by [`sort_records`] to select the
/// comparison key without any SQL statement.
#[derive(Debug, Clone, PartialEq)]
pub enum SortColumn {
    /// Sort by SiteName column (alphabetical, ascending).
    SiteName,
    /// Sort by SiteNumber column (lexicographic, ascending).
    SiteNumber,
    /// Sort by Year column (lexicographic, ascending).
    Year,
    /// Sort by THg (total mercury) column (lexicographic, ascending).
    THg,
}

impl fmt::Display for SortColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortColumn::SiteName   => write!(f, "SiteName"),
            SortColumn::SiteNumber => write!(f, "SiteNumber"),
            SortColumn::Year       => write!(f, "Year"),
            SortColumn::THg        => write!(f, "THg"),
        }
    }
}

/// Sorts a mutable slice of [`MercuryRecord`] values in-place by the
/// specified [`SortColumn`], in ascending order.
///
/// This function implements the **Algorithms (sorting)** advanced topic for
/// Practical Project Part 3. It uses Rust's built-in `Vec::sort_by` API with
/// a closure that selects the comparison key based on `column`. The sort is
/// stable (equal elements retain their original relative order) [3].
///
/// # Arguments
///
/// * `records` – A mutable slice of [`MercuryRecord`] to sort in-place.
/// * `column`  – The [`SortColumn`] that determines the comparison key.
///
/// # Examples
///
/// ```
/// sort_records(&mut records, SortColumn::Year);
/// // records is now sorted by the Year dataset column, ascending.
/// ```
pub fn sort_records(records: &mut Vec<MercuryRecord>, column: &SortColumn) {
    records.sort_by(|a, b| {
        let key_a = match column {
            SortColumn::SiteName   => a.SiteName.as_str(),
            SortColumn::SiteNumber => a.SiteNumber.as_str(),
            SortColumn::Year       => a.Year.as_str(),
            SortColumn::THg        => a.THg.as_str(),
        };
        let key_b = match column {
            SortColumn::SiteName   => b.SiteName.as_str(),
            SortColumn::SiteNumber => b.SiteNumber.as_str(),
            SortColumn::Year       => b.Year.as_str(),
            SortColumn::THg        => b.THg.as_str(),
        };
        key_a.cmp(key_b)
    });
}

/// Record-object representing one row from the Mercury Concentrations dataset.
///
/// Field names are taken directly from the dataset column headers to verify
/// this program uses the correct dataset as required by the course.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct MercuryRecord {
    /// Sampling site name (SiteName column).
    pub SiteName: String,
    /// Numeric site identifier (SiteNumber column).
    pub SiteNumber: String,
    /// Year of sample collection (Year column).
    pub Year: String,
    /// Depth in the water column in metres (Water_Column_Depth column).
    pub Water_Column_Depth: String,
    /// Total mercury concentration in ng/L (THg column).
    pub THg: String,
    /// Dimethyl mercury concentration in pg/L (DMHg column).
    pub DMHg: String,
}

impl MercuryRecord {
    /// Creates a new [`MercuryRecord`] from individual field values.
    ///
    /// # Arguments
    ///
    /// * `SiteName`           – Name of the sampling site.
    /// * `SiteNumber`         – Numeric site identifier.
    /// * `Year`               – Year of collection.
    /// * `Water_Column_Depth` – Sample depth in metres.
    /// * `THg`                – Total mercury (ng/L).
    /// * `DMHg`               – Dimethyl mercury (pg/L).
    #[allow(non_snake_case)]
    pub fn new(
        SiteName: String, SiteNumber: String, Year: String,
        Water_Column_Depth: String, THg: String, DMHg: String,
    ) -> Self {
        MercuryRecord { SiteName, SiteNumber, Year, Water_Column_Depth, THg, DMHg }
    }

    /// Returns the SiteName field value.
    pub fn site_name(&self) -> &str { &self.SiteName }

    /// Returns the SiteNumber field value.
    pub fn site_number(&self) -> &str { &self.SiteNumber }

    /// Returns the Year field value.
    pub fn year(&self) -> &str { &self.Year }

    /// Returns the Water_Column_Depth field value.
    pub fn water_column_depth(&self) -> &str { &self.Water_Column_Depth }

    /// Returns the THg field value.
    pub fn thg(&self) -> &str { &self.THg }

    /// Returns the DMHg field value.
    pub fn dmhg(&self) -> &str { &self.DMHg }
}

/// Formats a [`MercuryRecord`] for display, labelling each field with its
/// dataset column name [2].
impl fmt::Display for MercuryRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  SiteName           : {}\n  \
               SiteNumber         : {}\n  \
               Year               : {}\n  \
               Water_Column_Depth : {}\n  \
               THg (ng/L)         : {}\n  \
               DMHg (pg/L)        : {}",
            self.SiteName, self.SiteNumber, self.Year,
            self.Water_Column_Depth, self.THg, self.DMHg
        )
    }
}
