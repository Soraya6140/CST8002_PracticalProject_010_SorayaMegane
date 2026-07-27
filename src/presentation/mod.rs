// =============================================================================
// File:       src/presentation/mod.rs
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
// [1] The Rust Foundation, "std::io," The Rust Standard Library. [Online].
//     Available: https://doc.rust-lang.org/std/io/index.html.
//     [Accessed: May 2026].
// [2] K. Fakhroutdinov. (2009-2024). "Multi-Layered Application: UML Model
//     Diagram Example." uml-diagrams.org. [Online]. Available:
//     https://www.uml-diagrams.org/multi-layered-application-uml-model-diagram-example.html.
//     [Accessed: May 2026].
// =============================================================================

//! # Presentation Layer
//!
//! All user-facing console interaction for the CST8002 PP2 application.
//! Calls into the Business layer for data operations and the Persistence
//! layer for File-IO. Contains no business logic itself.

use crate::business::BusinessLayer;
use crate::model::MercuryRecord;
use crate::persistence;
use std::io::{self, BufRead, Write};

/// Dataset CSV filename — read from the current working directory.
const DATASET_FILE: &str =
    "NCP_ArcticMarineEcosystems_Mercury_Concentrations_EN_FR.csv";

/// Student name displayed on every menu iteration.
const STUDENT_NAME: &str = "Soraya Megane Kaji";

/// Reads one trimmed line from standard input, flushing stdout first so
/// any preceding prompt is always visible.
///
/// # Returns
///
/// A `String` containing the trimmed user input, or empty on error.
fn read_line() -> String {
    io::stdout().flush().ok();
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).ok();
    line.trim().to_string()
}

/// Prompts the user for a numeric index and returns it as `usize`.
///
/// # Arguments
///
/// * `prompt` – The text to display before reading input.
///
/// # Returns
///
/// The parsed index, or `0` if the input was not a valid number.
fn read_index(prompt: &str) -> usize {
    print!("{}", prompt);
    read_line().parse::<usize>().unwrap_or(0)
}

/// Prints the main application menu banner, including the student name,
/// so it is visible on every loop iteration.
fn print_menu(count: usize) {
    println!();
    println!("+===========================================================+");
    println!("|  CST8002 Practical Project 2 – Mercury Concentrations     |");
    println!("|  Student: {:47}|", STUDENT_NAME);
    println!("|  Records in memory: {:37}|", count);
    println!("|-----------------------------------------------------------|");
    println!("|  1) Reload data from dataset                              |");
    println!("|  2) Save data to file (UUID filename)                     |");
    println!("|  3) Display one record                                    |");
    println!("|  4) Display all records                                   |");
    println!("|  5) Create a new record                                   |");
    println!("|  6) Edit a record                                         |");
    println!("|  7) Delete a record                                       |");
    println!("|  8) Exit                                                  |");
    println!("+===========================================================+");
    print!("  Choose an option (1-8): ");
}

/// Handles menu option 1 – reload data from the dataset CSV file.
///
/// Calls [`persistence::load_records`] and replaces the in-memory data
/// via [`BusinessLayer::load`].
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

/// Handles menu option 2 – save in-memory data to a UUID-named CSV file.
///
/// Calls [`persistence::save_records`], which generates a UUID v4 filename
/// and writes all records to it.
///
/// # Arguments
///
/// * `bl` – Reference to the [`BusinessLayer`].
fn handle_save(bl: &BusinessLayer) {
    println!("\n--- Save Data (UUID filename) ---");
    match persistence::save_records(bl.all()) {
        Ok(filename) => println!("  Saved {} record(s) to '{}'.", bl.count(), filename),
        Err(e) => println!("  Error: {}", e),
    }
}

/// Handles menu option 3 – display one record selected by index.
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

/// Handles menu option 4 – display all records, printing the student name
/// every 10 records so it remains visible throughout long output.
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

/// Prompts the user to enter all six fields for a [`MercuryRecord`].
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
        read("SiteName"),
        read("SiteNumber"),
        read("Year"),
        read("Water_Column_Depth"),
        read("THg (ng/L)"),
        read("DMHg (pg/L)"),
    )
}

/// Handles menu option 5 – create a new record in memory.
///
/// # Arguments
///
/// * `bl` – Mutable reference to the [`BusinessLayer`].
fn handle_create(bl: &mut BusinessLayer) {
    println!("\n--- Create New Record ---");
    let record = prompt_record_fields();
    bl.create(record);
    println!("  Record added. Total records: {}.", bl.count());
}

/// Handles menu option 6 – edit an existing record in memory.
///
/// # Arguments
///
/// * `bl` – Mutable reference to the [`BusinessLayer`].
fn handle_edit(bl: &mut BusinessLayer) {
    println!("\n--- Edit Record ---");
    let idx = read_index(&format!("  Enter record number to edit (1–{}): ", bl.count()));
    match bl.get(idx) {
        None => { println!("  Invalid index."); return; }
        Some(r) => {
            println!("  Current values:");
            println!("{}", r);
            println!("  Enter new values (press Enter to keep blank):");
        }
    }
    let record = prompt_record_fields();
    if bl.update(idx, record) {
        println!("  Record {} updated.", idx);
    } else {
        println!("  Update failed.");
    }
}

/// Handles menu option 7 – delete a record from memory.
///
/// # Arguments
///
/// * `bl` – Mutable reference to the [`BusinessLayer`].
fn handle_delete(bl: &mut BusinessLayer) {
    println!("\n--- Delete Record ---");
    let idx = read_index(&format!("  Enter record number to delete (1–{}): ", bl.count()));
    if bl.delete(idx) {
        println!("  Record {} deleted. Total records: {}.", idx, bl.count());
    } else {
        println!("  Invalid index.");
    }
}

/// Runs the main application loop.
///
/// Loads the dataset on startup, then presents the menu repeatedly until
/// the user selects option 8 (Exit). Each menu iteration displays the
/// student name and the current record count.
pub fn run() {
    println!("CST8002 Practical Project 2 – Mercury Concentrations");
    println!("Student: {}", STUDENT_NAME);
    println!("Loading dataset...");

    let mut bl = BusinessLayer::new();

    match persistence::load_records(DATASET_FILE) {
        Ok(records) => {
            println!("  Loaded {} record(s).", records.len());
            bl.load(records);
        }
        Err(e) => {
            eprintln!("  Warning: Could not load dataset on startup: {}", e);
            eprintln!("  Use menu option 1 to reload once the file is available.");
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
            _ => println!("  Invalid option. Please enter 1–8."),
        }
    }
}
