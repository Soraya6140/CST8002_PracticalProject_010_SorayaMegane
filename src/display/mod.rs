// =============================================================================
// File:       src/display/mod.rs
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
// [1] The Rust Foundation, "std::fmt – Formatting," The Rust Standard Library.
//     [Online]. Available: https://doc.rust-lang.org/std/fmt/index.html.
//     [Accessed: May 2026].
// [2] The Rust Foundation, "Rust API Guidelines – Documentation," Rust API
//     Guidelines. [Online]. Available:
//     https://rust-lang.github.io/api-guidelines/documentation.html.
//     [Accessed: May 2026].
// =============================================================================

//! # Display Module
//!
//! Handles all console output for the CST8002 Practical Project Part 1
//! application. Keeps display logic separated from data loading and the
//! record model, following a modular design.

use crate::models::MercuryRecord;

/// Prints the application header banner.
///
/// Displays the program title, dataset attribution, and the student's full
/// name so it is always visible at the top of the output.
pub fn print_header() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   CST8002 – Practical Project Part 1                        ║");
    println!("║   Mercury Concentrations: Canadian Arctic Marine Ecosystem  ║");
    println!("║   Dataset: Environment and Climate Change Canada (2018)     ║");
    println!("║   Student: Soraya Megane Kaji                               ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

/// Prints a single [`MercuryRecord`] to standard output.
///
/// Outputs the record number followed by all six field values, each
/// labelled with the corresponding dataset column name.
///
/// # Arguments
///
/// * `index`  – 1-based position of the record in the loaded data.
/// * `record` – Reference to the [`MercuryRecord`] to print.
pub fn print_record(index: usize, record: &MercuryRecord) {
    println!("─── Record {} ───────────────────────────────────────────────────", index);
    println!("{}", record);
    println!();
}

/// Loops over a slice of [`MercuryRecord`] values and prints each one.
///
/// Prints a summary header showing the total record count, then calls
/// [`print_record`] for each entry in the slice.
///
/// # Arguments
///
/// * `records` – A slice of [`MercuryRecord`] instances to display.
pub fn print_all_records(records: &[MercuryRecord]) {
    println!("Loaded {} record(s) from dataset:\n", records.len());
    for (i, record) in records.iter().enumerate() {
        print_record(i + 1, record);
    }
}

/// Prints the application footer.
///
/// Re-displays the student name to ensure it remains visible at the
/// bottom of the output as well.
pub fn print_footer() {
    println!("──────────────────────────────────────────────────────────────────");
    println!("  Program complete.  Student: Soraya Megane Kaji");
    println!("──────────────────────────────────────────────────────────────────");
}
