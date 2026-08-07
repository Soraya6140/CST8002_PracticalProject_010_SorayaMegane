// =============================================================================
// File:       src/persistence/mod.rs
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
// [1] The Rust Foundation. (2026, Jul. 28). std::fs – Filesystem manipulation
//     operations. The Rust Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/fs/index.html [Accessed: Jul. 28, 2026].
// [2] BurntSushi. (2026, Jul. 28). csv – CSV reading and writing for Rust.
//     crates.io. [Online]. Available: https://crates.io/crates/csv
//     [Accessed: Jul. 28, 2026].
// [3] The Rust Foundation. (2026, Jul. 28). uuid crate. crates.io. [Online].
//     Available: https://crates.io/crates/uuid [Accessed: Jul. 28, 2026].
// =============================================================================

//! # Persistence Layer
//!
//! All File-IO for the CST8002 PP4 application. Unchanged from PP3 — the
//! N-Layered Architecture meant adding the search/filter feature in PP4
//! required zero changes to this layer.

use crate::model::MercuryRecord;
use std::io::Cursor;
use uuid::Uuid;

const SKIP_ROWS: usize = 30;
const MAX_RECORDS: usize = 100;
const COL_SITE_NAME:          usize = 0;
const COL_SITE_NUMBER:        usize = 1;
const COL_YEAR:               usize = 2;
const COL_WATER_COLUMN_DEPTH: usize = 3;
const COL_THG:                usize = 4;
const COL_DMHG:               usize = 5;

/// Converts raw bytes to String, replacing invalid UTF-8 (ANSI/Windows-1252)
/// with the replacement character [1].
fn bytes_to_utf8_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Reads the dataset CSV and returns up to 100 [`MercuryRecord`] objects.
///
/// Strips UTF-8 BOM if present, handles ANSI encoding lossily, skips the
/// first 30 metadata rows, then parses data rows using the csv crate [1][2].
///
/// # Arguments
///
/// * `path` – Path to the dataset CSV file.
///
/// # Returns
///
/// * `Ok(Vec<MercuryRecord>)` on success.
/// * `Err(String)` with a descriptive message on failure.
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

/// Writes all in-memory records to a new CSV file with a UUID v4 filename [3].
///
/// # Arguments
///
/// * `records` – Slice of records to write.
///
/// # Returns
///
/// * `Ok(String)` with the generated filename.
/// * `Err(String)` with a descriptive message on failure.
pub fn save_records(records: &[MercuryRecord]) -> Result<String, String> {
    let filename = format!("{}.csv", Uuid::new_v4());
    let mut wtr = csv::Writer::from_path(&filename)
        .map_err(|e| format!("Could not create '{}': {}", filename, e))?;
    wtr.write_record(&["SiteName","SiteNumber","Year",
                        "Water_Column_Depth","THg","DMHg"])
        .map_err(|e| e.to_string())?;
    for r in records {
        wtr.write_record(&[&r.SiteName, &r.SiteNumber, &r.Year,
                            &r.Water_Column_Depth, &r.THg, &r.DMHg])
            .map_err(|e| e.to_string())?;
    }
    wtr.flush().map_err(|e| e.to_string())?;
    Ok(filename)
}
