// =============================================================================
// File:       src/model/mod.rs
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
// [2] The Rust Foundation. (2026, Jul. 28). str::contains – Returns true if
//     the given pattern matches a sub-slice. The Rust Standard Library.
//     [Online]. Available:
//     https://doc.rust-lang.org/std/primitive.str.html#method.contains
//     [Accessed: Jul. 28, 2026].
// [3] The Rust Foundation. (2026, Jul. 28). Processing a Series of Items
//     with Iterators. The Rust Programming Language Book, Ch. 13. [Online].
//     Available: https://doc.rust-lang.org/book/ch13-02-iterators.html
//     [Accessed: Jul. 28, 2026].
// [4] The Rust Foundation. (2026, Jul. 28). std::vec::Vec::sort_by. The Rust
//     Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
//     [Accessed: Jul. 28, 2026].
// [5] The Rust Foundation. (2026, Jul. 28). Structs: Defining and
//     Instantiating Structs. The Rust Programming Language Book. [Online].
//     Available: https://doc.rust-lang.org/book/ch05-00-structs.html
//     [Accessed: Jul. 28, 2026].
// =============================================================================

//! # Model Layer
//!
//! Defines the [`MercuryRecord`] entity, the [`SortColumn`] enum and
//! [`sort_records`] function (PP3 Algorithms feature), and the new
//! [`SearchCriteria`] struct and [`filter_records`] function (PP4 novel
//! Search/Filter feature). All field names match the dataset column names.

use std::fmt;

// =============================================================================
// MercuryRecord — dataset entity / record-object
// =============================================================================

/// Record-object representing one row from the Mercury Concentrations dataset.
/// Field names match the dataset column headers exactly [5].
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct MercuryRecord {
    /// SiteName column — name of the Arctic sampling site.
    pub SiteName: String,
    /// SiteNumber column — numeric site identifier.
    pub SiteNumber: String,
    /// Year column — year of sample collection.
    pub Year: String,
    /// Water_Column_Depth column — sample depth in metres.
    pub Water_Column_Depth: String,
    /// THg column — total mercury concentration (ng/L).
    pub THg: String,
    /// DMHg column — dimethyl mercury concentration (pg/L).
    pub DMHg: String,
}

impl MercuryRecord {
    /// Creates a new [`MercuryRecord`] from individual field values.
    #[allow(non_snake_case)]
    pub fn new(
        SiteName: String, SiteNumber: String, Year: String,
        Water_Column_Depth: String, THg: String, DMHg: String,
    ) -> Self {
        MercuryRecord { SiteName, SiteNumber, Year, Water_Column_Depth, THg, DMHg }
    }
}

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

// =============================================================================
// PP3 Algorithms Feature — SortColumn and sort_records
// =============================================================================

/// Identifies which dataset column to sort [`MercuryRecord`] values by.
/// Part of the PP3 Algorithms (sorting) advanced topic.
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

/// Sorts a mutable `Vec<MercuryRecord>` in-place by the specified column.
/// Uses Rust's built-in `Vec::sort_by` API — no SQL statement [4].
///
/// # Arguments
///
/// * `records` – Mutable slice to sort.
/// * `column`  – The [`SortColumn`] that determines the comparison key.
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

// =============================================================================
// PP4 Novel Feature — SearchCriteria and filter_records
// =============================================================================

/// Holds optional search criteria for multi-column filtering.
///
/// Each field is `Option<String>`:
/// - `None`  → wildcard, matches all records for that column.
/// - `Some(term)` → only records where the column value contains `term`
///   (case-insensitive, partial match).
///
/// This is the PP4 novel feature: Search / Filter records based on
/// multiple columns at the same time [1][2][3].
pub struct SearchCriteria {
    /// Optional filter on the SiteName column.
    pub site_name: Option<String>,
    /// Optional filter on the Year column.
    pub year: Option<String>,
    /// Optional filter on the THg column.
    pub thg: Option<String>,
}

impl SearchCriteria {
    /// Creates a new [`SearchCriteria`] from three optional search terms.
    ///
    /// # Arguments
    ///
    /// * `site_name` – Optional partial match for SiteName column.
    /// * `year`      – Optional partial match for Year column.
    /// * `thg`       – Optional partial match for THg column.
    pub fn new(
        site_name: Option<String>,
        year:      Option<String>,
        thg:       Option<String>,
    ) -> Self {
        SearchCriteria { site_name, year, thg }
    }

    /// Returns `true` if all fields are `None` (no filter applied).
    pub fn is_empty(&self) -> bool {
        self.site_name.is_none() && self.year.is_none() && self.thg.is_none()
    }
}

/// Filters a slice of [`MercuryRecord`] values by the given
/// [`SearchCriteria`], returning references to matching records.
///
/// Uses `Iterator::filter()` with chained `&&` conditions — one pass O(n).
/// `str::contains()` with `to_lowercase()` provides case-insensitive partial
/// matching. `None` criteria always pass (wildcard) [1][2][3].
///
/// # Arguments
///
/// * `records`  – The full collection of records to search.
/// * `criteria` – The [`SearchCriteria`] defining which columns to filter.
///
/// # Returns
///
/// A `Vec<&MercuryRecord>` of matching records. Original data unchanged.
pub fn filter_records<'a>(
    records:  &'a [MercuryRecord],
    criteria: &SearchCriteria,
) -> Vec<&'a MercuryRecord> {
    records
        .iter()
        .filter(|r| {
            let name_ok = criteria.site_name.as_ref().map_or(true, |term| {
                r.SiteName.to_lowercase().contains(term.to_lowercase().as_str())
            });
            let year_ok = criteria.year.as_ref().map_or(true, |term| {
                r.Year.to_lowercase().contains(term.to_lowercase().as_str())
            });
            let thg_ok = criteria.thg.as_ref().map_or(true, |term| {
                r.THg.to_lowercase().contains(term.to_lowercase().as_str())
            });
            name_ok && year_ok && thg_ok
        })
        .collect()
}
