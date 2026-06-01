// =============================================================================
// File:       src/main.rs
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
// [1] The Rust Foundation, "std::fs – Filesystem manipulation operations,"
//     The Rust Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/fs/index.html. [Accessed: May 2026].
// [2] BurntSushi, "csv crate – CSV reading and writing for Rust," crates.io.
//     [Online]. Available: https://crates.io/crates/csv. [Accessed: May 2026].
// [3] The Rust Foundation, "Error Handling," The Rust Programming Language
//     Book. [Online]. Available:
//     https://doc.rust-lang.org/book/ch09-00-error-handling.html.
//     [Accessed: May 2026].
// [4] The Rust Foundation, "Structs," The Rust Programming Language Book.
//     [Online]. Available:
//     https://doc.rust-lang.org/book/ch05-00-structs.html.
//     [Accessed: May 2026].
// [5] The Rust Foundation, "std::fmt – Formatting," The Rust Standard Library.
//     [Online]. Available: https://doc.rust-lang.org/std/fmt/index.html.
//     [Accessed: May 2026].
// [6] git. (n.d.). "2.6 Git Basics – Tagging." git-scm.com. [Online].
//     Available: https://git-scm.com/book/en/v2/Git-Basics-Tagging.
//     [Accessed: May 2026].
// =============================================================================

//! # CST8002 Practical Project Part 1 – Entry Point
//!
//! Reads the Mercury Concentrations dataset (rows 31–102 per the course
//! specification), parses each row into a [`models::MercuryRecord`] struct,
//! stores the records in a `Vec`, and outputs them to the console via the
//! [`display`] module.
//!
//! Programming concepts demonstrated:
//! - **Variables**         – `records`, `raw_bytes`, `row_count`
//! - **Methods/Functions** – `load_records()`, `display::print_all_records()`
//! - **Loop structure**    – `for` loop over CSV rows; `for` loop in display
//! - **File-IO**           – `std::fs::read()` + `csv::Reader`
//! - **Exception handling**– `match` on `Result`, graceful error messages
//! - **Library use**       – `csv` crate (external library)
//! - **Array / Vec**       – `Vec<MercuryRecord>` stores parsed records

mod display;
mod models;

use models::MercuryRecord;
use std::io::Cursor;

/// Name of the dataset CSV file. The program reads from the current directory.
const DATASET_FILE: &str = "NCP_ArcticMarineEcosystems_Mercury_Concentrations_EN_FR.csv";

/// The dataset has 30 header/metadata rows before data rows start at row 31.
const SKIP_ROWS: usize = 30;

/// Maximum number of data records to load (rows 31–102 = 72 records).
const MAX_RECORDS: usize = 72;

/// Column index constants matching the dataset layout.
const COL_SITE_NAME:           usize = 0;
const COL_SITE_NUMBER:         usize = 1;
const COL_YEAR:                usize = 2;
const COL_WATER_COLUMN_DEPTH:  usize = 3;
const COL_THG:                 usize = 4;
const COL_DMHG:                usize = 5;

/// Converts a byte slice to a String, replacing any non-UTF-8 bytes with the
/// Unicode replacement character (U+FFFD). This handles ANSI / Windows-1252
/// encoded CSV files such as the Government of Canada dataset.
///
/// # Arguments
///
/// * `bytes` – Raw byte slice read from the file.
///
/// # Returns
///
/// A `String` with invalid UTF-8 sequences replaced by the replacement char.
fn bytes_to_string_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Reads and parses the dataset CSV file into a `Vec<MercuryRecord>`.
///
/// Reads the raw bytes of the file and converts them lossily to UTF-8, which
/// handles both UTF-8 BOM and ANSI (Windows-1252) encoded files. Skips the
/// first [`SKIP_ROWS`] rows (metadata and bilingual headers), then reads up
/// to [`MAX_RECORDS`] data rows using the csv crate.
///
/// # Returns
///
/// * `Ok(Vec<MercuryRecord>)` – Vector of parsed records on success.
/// * `Err(Box<dyn std::error::Error>)` – Boxed error if the file cannot be
///   opened or a row cannot be parsed.
fn load_records() -> Result<Vec<MercuryRecord>, Box<dyn std::error::Error>> {
    // Vec (array-like data structure) to store parsed record objects
    let mut records: Vec<MercuryRecord> = Vec::new();

    // Read raw bytes using File-IO (std::fs::read)
    let raw_bytes = std::fs::read(DATASET_FILE)?;

    // Strip UTF-8 BOM (EF BB BF) if present
    let data_bytes: &[u8] = if raw_bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &raw_bytes[3..]
    } else {
        &raw_bytes
    };

    // Convert bytes to String lossily — handles ANSI / Windows-1252 encoding
    // by replacing unrecognised characters with the Unicode replacement char
    let csv_string = bytes_to_string_lossy(data_bytes);

    // Open the CSV reader from the in-memory String using the csv library
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)   // manage headers/skipping manually
        .flexible(true)       // allow rows with varying column counts
        .from_reader(Cursor::new(csv_string.as_bytes()));

    let mut row_count: usize = 0;

    // Loop structure: iterate over every row in the CSV file
    for result in reader.records() {
        let record = result?;
        row_count += 1;

        // Skip the first SKIP_ROWS rows (metadata + bilingual column headers)
        if row_count <= SKIP_ROWS {
            continue;
        }

        // Stop after loading MAX_RECORDS data records
        if records.len() >= MAX_RECORDS {
            break;
        }

        // Helper closure: safely retrieve a field by column index
        let field = |idx: usize| -> String {
            record.get(idx).unwrap_or("").trim().to_string()
        };

        // Build a MercuryRecord using dataset column names as field names
        let mercury_record = MercuryRecord::new(
            field(COL_SITE_NAME),
            field(COL_SITE_NUMBER),
            field(COL_YEAR),
            field(COL_WATER_COLUMN_DEPTH),
            field(COL_THG),
            field(COL_DMHG),
        );

        records.push(mercury_record);
    }

    Ok(records)
}

/// Application entry point.
///
/// Calls [`load_records`] to read the dataset, then delegates display to
/// the [`display`] module. Uses exception handling (`match`) to catch
/// file-not-found and parse errors and exit gracefully.
fn main() {
    display::print_header();

    // Exception handling: match on the Result returned by load_records()
    match load_records() {
        Ok(records) => {
            // Loop over the Vec and output each record via the display module
            display::print_all_records(&records);
            display::print_footer();
        }
        Err(e) => {
            eprintln!("Error: Could not load dataset.");
            eprintln!("  Details : {}", e);
            eprintln!("  Expected: '{}' in the current directory.", DATASET_FILE);
            eprintln!("  Please ensure the CSV file is present and try again.");
            std::process::exit(1);
        }
    }
}
