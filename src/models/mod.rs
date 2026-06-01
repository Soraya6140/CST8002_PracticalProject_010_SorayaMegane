// =============================================================================
// File:       src/models/mod.rs
// Author:     Soraya Megane Kaji
// Course:     CST8002 – Programming Language Research Project
// Professor:  Stanley Pieda
// Assessment: Practical Project Part 1
//
// Dataset Attribution:
//   Environment and Climate Change Canada. (Aug 28, 2018).
//   Mercury concentrations in the Canadian Arctic marine ecosystem.
//   open.canada.ca. [Online]. Available:
//   https://open.canada.ca/data/en/dataset/d4285538-afa5-4644-931a-36d6ab6e25a1
//   [Accessed: May 2026].
//   Contains information licensed under the Open Government Licence – Canada.
//   https://open.canada.ca/en/open-government-licence-canada
//
// References:
// [1] The Rust Foundation, "Structs," The Rust Programming Language Book.
//     [Online]. Available:
//     https://doc.rust-lang.org/book/ch05-00-structs.html.
//     [Accessed: May 2026].
// [2] The Rust Foundation, "std::fmt – Formatting," The Rust Standard Library.
//     [Online]. Available: https://doc.rust-lang.org/std/fmt/index.html.
//     [Accessed: May 2026].
// [3] The Rust Foundation, "Rust API Guidelines – Documentation," Rust API
//     Guidelines. [Online]. Available:
//     https://rust-lang.github.io/api-guidelines/documentation.html.
//     [Accessed: May 2026].
// =============================================================================

//! # Models Module
//!
//! Defines the [`MercuryRecord`] data-transfer object (record object) whose
//! field names are derived directly from the dataset column names:
//! `SiteName`, `SiteNumber`, `Year`, `Water_Column_Depth`, `THg`, `DMHg`.

use std::fmt;

/// A record object (data-transfer object) representing one row from the
/// `NCP_ArcticMarineEcosystems_Mercury_Concentrations_EN_FR.csv` dataset.
///
/// Field names match the dataset column names exactly so a reviewer can
/// confirm this program was written specifically for the assigned dataset.
///
/// # Dataset Columns Used
///
/// | Field              | Dataset Column     | Unit / Description              |
/// |--------------------|--------------------|---------------------------------|
/// | `SiteName`         | SiteName           | Name of the sampling location   |
/// | `SiteNumber`       | SiteNumber         | Numeric site identifier         |
/// | `Year`             | Year               | Year of sample collection       |
/// | `Water_Column_Depth` | Water_Column_Depth | Depth in metres                |
/// | `THg`              | THg                | Total mercury (ng/L)            |
/// | `DMHg`             | DMHg               | Dimethyl mercury (pg/L)         |
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct MercuryRecord {
    /// Name of the Arctic sampling site (SiteName column).
    pub SiteName: String,

    /// Numeric identifier for the sampling site (SiteNumber column).
    pub SiteNumber: String,

    /// Year in which the sample was collected (Year column).
    pub Year: String,

    /// Depth in the water column at which the sample was taken, in metres
    /// (Water_Column_Depth column).
    pub Water_Column_Depth: String,

    /// Total mercury concentration in nanograms per litre (THg column).
    pub THg: String,

    /// Dimethyl mercury concentration in picograms per litre (DMHg column).
    pub DMHg: String,
}

impl MercuryRecord {
    /// Creates a new `MercuryRecord` from the given field values.
    ///
    /// # Arguments
    ///
    /// * `SiteName`           – Name of the sampling site.
    /// * `SiteNumber`         – Numeric site identifier.
    /// * `Year`               – Year of sample collection.
    /// * `Water_Column_Depth` – Sample depth in metres.
    /// * `THg`                – Total mercury concentration (ng/L).
    /// * `DMHg`               – Dimethyl mercury concentration (pg/L).
    ///
    /// # Returns
    ///
    /// A new [`MercuryRecord`] instance populated with the supplied values.
    #[allow(non_snake_case)]
    pub fn new(
        SiteName: String,
        SiteNumber: String,
        Year: String,
        Water_Column_Depth: String,
        THg: String,
        DMHg: String,
    ) -> Self {
        MercuryRecord {
            SiteName,
            SiteNumber,
            Year,
            Water_Column_Depth,
            THg,
            DMHg,
        }
    }
}

/// Formats a [`MercuryRecord`] for display on the console.
///
/// Produces a multi-line block showing all six fields with their dataset
/// column names so the output clearly maps back to the source data.
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
            self.SiteName,
            self.SiteNumber,
            self.Year,
            self.Water_Column_Depth,
            self.THg,
            self.DMHg,
        )
    }
}
