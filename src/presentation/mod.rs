// =============================================================================
// File:       src/presentation/mod.rs
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
// [1] The Rust Foundation. (2026, Jul. 8). std::io – Traits, helpers, and
//     type definitions for core I/O. The Rust Standard Library. [Online].
//     Available: https://doc.rust-lang.org/std/io/index.html
//     [Accessed: Jul. 8, 2026].
// [2] The Rust Foundation. (2026, Jul. 8). std::vec::Vec::sort_by. The Rust
//     Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
//     [Accessed: Jul. 8, 2026].
// =============================================================================

//! # Presentation Layer
//!
//! All user-facing console interaction. Calls the Business layer for data
//! operations and the Persistence layer for File-IO. No business logic here.
//!
//! **New in Part 3:** menu option 9 — Sort records — which calls
//! [`BusinessLayer::sort`] via the Business layer to implement the Algorithms
//! advanced topic. The user selects the sort column; the sorted result is
//! then visible when they choose Display All (option 4) [2].

use crate::business::BusinessLayer;
use crate::model::{MercuryRecord, SortColumn};
use crate::persistence;
use std::io::{self, BufRead, Write};

/// Dataset CSV filename — read from the current working directory.
const DATASET_FILE: &str =
    "NCP_ArcticMarineEcosystems_Mercury_Concentrations_EN_FR.csv";

/// Student name displayed on every menu iteration.
const STUDENT_NAME: &str = "Soraya Megane Kaji";

/// Reads one trimmed line from standard input, flushing stdout first.
///
/// # Returns
///
/// A `String` containing the user's trimmed input, or empty on error [1].
fn read_line() -> String {
    io::stdout().flush().ok();
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).ok();
    line.trim().to_string()
}

/// Prompts for a numeric index and returns it as `usize`.
///
/// # Arguments
///
/// * `prompt` – The text displayed before reading input.
///
/// # Returns
///
/// The parsed index, or `0` if the input was not a valid number.
fn read_index(prompt: &str) -> usize {
    print!("{}", prompt);
    read_line().parse::<usize>().unwrap_or(0)
}

/// Prints the main application menu including the student name and record
/// count, so the student name is visible on every menu iteration.
///
/// # Arguments
///
/// * `count` – The current number of records held in memory.
fn print_menu(count: usize) {
    println!();
    println!("+============================================================+");
    println!("|  CST8002 Practical Project 3 – Mercury Concentrations      |");
    println!("|  Student: {:51}|", STUDENT_NAME);
    println!("|  Records in memory: {:41}|", count);
    println!("|-----------------------------------------------------------|");
    println!("|  1) Reload data from dataset                               |");
    println!("|  2) Save data to file (UUID filename)                      |");
    println!("|  3) Display one record                                     |");
    println!("|  4) Display all records                                    |");
    println!("|  5) Create a new record                                    |");
    println!("|  6) Edit a record                                          |");
    println!("|  7) Delete a record                                        |");
    println!("|  8) Exit                                                   |");
    println!("|  9) Sort records  [NEW – Algorithms / PP3 feature]        |");
    println!("+============================================================+");
    print!("  Choose an option (1-9): ");
}

/// Handles option 1 – reload data from the dataset CSV file.
///
/// # Arguments
///
/// * `bl` – Mutable reference to the [`BusinessLayer`].
fn handle_reload(bl: &mut BusinessLayer) {
    println!("\n--- Reload Data ---");
    match persistence::load_records(DATASET_FILE) {
        Ok(records) => {
            let n = records.len();
            bl.load(records);
            println!("  Loaded {} record(s) from '{}'.", n, DATASET_FILE);
        }
        Err(e) => println!("  Error: {}", e),
    }
}

/// Handles option 2 – save in-memory data to a UUID-named CSV file.
///
/// # Arguments
///
/// * `bl` – Reference to the [`BusinessLayer`].
fn handle_save(bl: &BusinessLayer) {
    println!("\n--- Save Data (UUID filename) ---");
    match persistence::save_records(bl.all()) {
        Ok(f) => println!("  Saved {} record(s) to '{}'.", bl.count(), f),
        Err(e) => println!("  Error: {}", e),
    }
}

/// Handles option 3 – display one record selected by index.
///
/// # Arguments
///
/// * `bl` – Reference to the [`BusinessLayer`].
fn handle_display_one(bl: &BusinessLayer) {
    println!("\n--- Display One Record ---");
    let idx = read_index(&format!("  Enter record number (1–{}): ", bl.count()));
    match bl.get(idx) {
        Some(r) => { println!(); println!("{}", r); }
        None    => println!("  Invalid index."),
    }
}

/// Handles option 4 – display all records, printing the student name every
/// 10 records so it remains visible throughout long output.
///
/// # Arguments
///
/// * `bl` – Reference to the [`BusinessLayer`].
fn handle_display_all(bl: &BusinessLayer) {
    println!("\n--- Display All {} Record(s) ---", bl.count());
    for (i, r) in bl.all().iter().enumerate() {
        if i % 10 == 0 {
            println!("  [Program by {}]", STUDENT_NAME);
        }
        println!("─── Record {} ──────────────────────────────────────", i + 1);
        println!("{}", r);
        println!();
    }
}

/// Prompts the user to enter all six dataset field values for a record.
///
/// # Returns
///
/// A fully populated [`MercuryRecord`] built from user input.
#[allow(non_snake_case)]
fn prompt_record_fields() -> MercuryRecord {
    let read = |label: &str| -> String {
        print!("  {}: ", label);
        read_line()
    };
    MercuryRecord::new(
        read("SiteName"), read("SiteNumber"), read("Year"),
        read("Water_Column_Depth"), read("THg (ng/L)"), read("DMHg (pg/L)"),
    )
}

/// Handles option 5 – create a new record in memory.
///
/// # Arguments
///
/// * `bl` – Mutable reference to the [`BusinessLayer`].
fn handle_create(bl: &mut BusinessLayer) {
    println!("\n--- Create New Record ---");
    let record = prompt_record_fields();
    bl.create(record);
    println!("  Record added. Total: {}.", bl.count());
}

/// Handles option 6 – edit an existing record in memory.
///
/// # Arguments
///
/// * `bl` – Mutable reference to the [`BusinessLayer`].
fn handle_edit(bl: &mut BusinessLayer) {
    println!("\n--- Edit Record ---");
    let idx = read_index(&format!("  Enter record number to edit (1–{}): ", bl.count()));
    match bl.get(idx) {
        None => { println!("  Invalid index."); return; }
        Some(r) => { println!("  Current:\n{}", r); }
    }
    let record = prompt_record_fields();
    if bl.update(idx, record) {
        println!("  Record {} updated.", idx);
    } else {
        println!("  Update failed.");
    }
}

/// Handles option 7 – delete a record from memory.
///
/// # Arguments
///
/// * `bl` – Mutable reference to the [`BusinessLayer`].
fn handle_delete(bl: &mut BusinessLayer) {
    println!("\n--- Delete Record ---");
    let idx = read_index(&format!("  Enter record number to delete (1–{}): ", bl.count()));
    if bl.delete(idx) {
        println!("  Record {} deleted. Total: {}.", idx, bl.count());
    } else {
        println!("  Invalid index.");
    }
}

/// Handles option 9 – sort the in-memory records by a user-selected column.
///
/// This function implements the **Algorithms (sorting)** advanced topic for
/// Practical Project Part 3. It presents the user with a sub-menu to choose
/// the sort column, then delegates to [`BusinessLayer::sort`], which calls
/// [`model::sort_records`] using Rust's `Vec::sort_by` API. The sorted result
/// is visible immediately by choosing Display All (option 4) [2].
///
/// # Arguments
///
/// * `bl` – Mutable reference to the [`BusinessLayer`].
fn handle_sort(bl: &mut BusinessLayer) {
    println!("\n--- Sort Records [PP3 Feature: Algorithms] ---");
    println!("  Sort by which column?");
    println!("  1) SiteName");
    println!("  2) SiteNumber");
    println!("  3) Year");
    println!("  4) THg (total mercury)");
    print!("  Choose (1-4): ");

    let column = match read_line().as_str() {
        "1" => SortColumn::SiteName,
        "2" => SortColumn::SiteNumber,
        "3" => SortColumn::Year,
        "4" => SortColumn::THg,
        _   => { println!("  Invalid choice. Returning to menu."); return; }
    };

    bl.sort(&column);
    println!("  {} record(s) sorted by {} (ascending).", bl.count(), column);
    println!("  Choose option 4 (Display All) to view the sorted records.");
}

/// Runs the main application loop.
///
/// Loads the dataset on startup, then presents the menu repeatedly until
/// the user selects option 8 (Exit). The student name and record count are
/// displayed in the menu header on every iteration.
pub fn run() {
    println!("CST8002 Practical Project 3 – Mercury Concentrations");
    println!("Student: {}", STUDENT_NAME);
    println!("Advanced topic: Algorithms (Sorting via Vec::sort_by)");
    println!("Loading dataset...");

    let mut bl = BusinessLayer::new();
    match persistence::load_records(DATASET_FILE) {
        Ok(records) => {
            println!("  Loaded {} record(s).", records.len());
            bl.load(records);
        }
        Err(e) => {
            eprintln!("  Warning: Could not load dataset on startup: {}", e);
            eprintln!("  Use option 1 to reload once the file is available.");
        }
    }

    loop {
        print_menu(bl.count());
        let choice = read_line();
        match choice.as_str() {
            "1" => handle_reload(&mut bl),
            "2" => handle_save(&bl),
            "3" => handle_display_one(&bl),
            "4" => handle_display_all(&bl),
            "5" => handle_create(&mut bl),
            "6" => handle_edit(&mut bl),
            "7" => handle_delete(&mut bl),
            "8" => {
                println!("\nExiting. Goodbye, {}!", STUDENT_NAME);
                break;
            }
            "9" => handle_sort(&mut bl),
            _   => println!("  Invalid option. Please enter 1–9."),
        }
    }
}
