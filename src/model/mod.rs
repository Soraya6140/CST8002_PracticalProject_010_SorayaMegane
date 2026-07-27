// =============================================================================
// File:       src/model/mod.rs
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
// [2] The Rust Foundation, "std::fmt," The Rust Standard Library. [Online].
//     Available: https://doc.rust-lang.org/std/fmt/index.html.
//     [Accessed: May 2026].
// [3] K. Fakhroutdinov. (2009-2024). "Multi-Layered Application: UML Model
//     Diagram Example." uml-diagrams.org. [Online]. Available:
//     https://www.uml-diagrams.org/multi-layered-application-uml-model-diagram-example.html.
//     [Accessed: May 2026].
// =============================================================================

//! # Model Layer
//!
//! Defines [`MercuryRecord`], the record-object (entity / data-transfer object)
//! for the Mercury Concentrations in the Canadian Arctic Marine Ecosystem
//! dataset. Field names match the dataset column names exactly.

use std::fmt;

/// Record-object representing one row from the dataset.
///
/// Field names are taken directly from the dataset column headers:
/// `SiteName`, `SiteNumber`, `Year`, `Water_Column_Depth`, `THg`, `DMHg`.
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
        SiteName: String,
        SiteNumber: String,
        Year: String,
        Water_Column_Depth: String,
        THg: String,
        DMHg: String,
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
/// dataset column name so output maps clearly back to the source data.
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
