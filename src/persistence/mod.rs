// =============================================================================
// File:       src/persistence/mod.rs
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
// [1] The Rust Foundation, "std::fs," The Rust Standard Library. [Online].
//     Available: https://doc.rust-lang.org/std/fs/index.html.
//     [Accessed: May 2026].
// [2] BurntSushi, "csv crate," crates.io. [Online]. Available:
//     https://crates.io/crates/csv. [Accessed: May 2026].
// [3] R. Oliveira. (Jul 31, 2024). "GUID vs UUID vs ULID," medium.com.
//     [Online]. Available:
//     https://medium.com/@ronaldo.oliver7/guid-vs-uuid-vs-ulid-understanding-unique-identifiers-565c88cdca13.
//     [Accessed: May 2026].
// [4] The Rust Foundation, "uuid crate," crates.io. [Online]. Available:
//     https://crates.io/crates/uuid. [Accessed: May 2026].
// =============================================================================

//! # Persistence Layer
//!
//! All File-IO for the CST8002 PP2 application. Contains no console
//! interaction — all data flows through function arguments and return values.
//!
//! - [`load_records`]  – reads the dataset CSV into a `Vec<MercuryRecord>`
//! - [`save_records`]  – writes the in-memory Vec to a new UUID-named CSV file

use crate::model::MercuryRecord;
use std::io::Cursor;
use uuid::Uuid;

/// Number of metadata/header rows to skip at the top of the dataset.
const SKIP_ROWS: usize = 30;

/// Maximum number of records to load on startup.
const MAX_RECORDS: usize = 100;

/// Column indices in the dataset CSV.
const COL_SITE_NAME: usize = 0;
const COL_SITE_NUMBER: usize = 1;
const COL_YEAR: usize = 2;
const COL_WATER_COLUMN_DEPTH: usize = 3;
const COL_THG: usize = 4;
const COL_DMHG: usize = 5;

/// Converts a raw byte slice to a String, replacing invalid UTF-8 bytes
/// (such as those found in ANSI/Windows-1252 encoded files) with the
/// Unicode replacement character so the CSV parser does not crash.
///
/// # Arguments
///
/// * `bytes` – Raw bytes read from the file.
///
/// # Returns
///
/// A `String` with any invalid UTF-8 sequences replaced.
fn bytes_to_utf8_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Reads the dataset CSV file and returns up to [`MAX_RECORDS`] parsed
/// [`MercuryRecord`] objects.
///
/// Strips a UTF-8 BOM if present, converts ANSI encoding lossily, skips
/// the first [`SKIP_ROWS`] metadata rows, then parses data rows using the
/// `csv` crate.
///
/// # Arguments
///
/// * `path` – Path to the dataset CSV file.
///
/// # Returns
///
/// * `Ok(Vec<MercuryRecord>)` on success.
/// * `Err(String)` with a descriptive message if the file is missing or
///   cannot be parsed (exception handling).
pub fn load_records(path: &str) -> Result<Vec<MercuryRecord>, String> {
    let raw = std::fs::read(path)
        .map_err(|e| format!("Could not open '{}': {}", path, e))?;

    let data: &[u8] = if raw.starts_with(&[0xEF, 0xBB, 0xBF]) { &raw[3..] } else { &raw };
    let csv_str = bytes_to_utf8_lossy(data);

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(Cursor::new(csv_str.as_bytes()));

    let mut records: Vec<MercuryRecord> = Vec::new();
    let mut row_count: usize = 0;

    for result in reader.records() {
        let row = result.map_err(|e| format!("CSV parse error: {}", e))?;
        row_count += 1;
        if row_count <= SKIP_ROWS { continue; }
        if records.len() >= MAX_RECORDS { break; }

        let f = |i: usize| row.get(i).unwrap_or("").trim().to_string();
        records.push(MercuryRecord::new(
            f(COL_SITE_NAME), f(COL_SITE_NUMBER), f(COL_YEAR),
            f(COL_WATER_COLUMN_DEPTH), f(COL_THG), f(COL_DMHG),
        ));
    }
    Ok(records)
}

/// Writes all in-memory [`MercuryRecord`] objects to a new CSV file whose
/// name is generated using a UUID v4 (universally unique identifier).
///
/// The UUID is generated with the `uuid` crate using `Uuid::new_v4()`,
/// producing a randomly generated 128-bit identifier that guarantees the
/// output filename is unique each time the function is called.
///
/// # Arguments
///
/// * `records` – Slice of [`MercuryRecord`] objects to persist.
///
/// # Returns
///
/// * `Ok(String)` containing the generated filename on success.
/// * `Err(String)` with a descriptive message if writing fails.
pub fn save_records(records: &[MercuryRecord]) -> Result<String, String> {
    let filename = format!("{}.csv", Uuid::new_v4());
    let mut wtr = csv::Writer::from_path(&filename)
        .map_err(|e| format!("Could not create '{}': {}", filename, e))?;

    wtr.write_record(&["SiteName","SiteNumber","Year","Water_Column_Depth","THg","DMHg"])
        .map_err(|e| e.to_string())?;

    for r in records {
        wtr.write_record(&[
            &r.SiteName, &r.SiteNumber, &r.Year,
            &r.Water_Column_Depth, &r.THg, &r.DMHg,
        ]).map_err(|e| e.to_string())?;
    }
    wtr.flush().map_err(|e| e.to_string())?;
    Ok(filename)
}
