// =============================================================================
// File:       src/presentation/mod.rs
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
// [1] The Rust Foundation. (2026, Jul. 28). std::io – Traits, helpers, and
//     type definitions for core I/O. The Rust Standard Library. [Online].
//     Available: https://doc.rust-lang.org/std/io/index.html
//     [Accessed: Jul. 28, 2026].
// [2] The Rust Foundation. (2026, Jul. 28). Iterator::filter – Standard
//     library iterator adaptor. The Rust Standard Library. [Online].
//     Available: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
//     [Accessed: Jul. 28, 2026].
// =============================================================================

//! # Presentation Layer
//!
//! All user-facing console interaction. Calls the Business layer for data
//! and the Persistence layer for File-IO. No business logic here.
//!
//! **New in PP4:** menu option 10 — Search/Filter records — which calls
//! [`BusinessLayer::search`] with a [`SearchCriteria`] built from the
//! user's optional input for SiteName, Year, and THg columns [2].

use crate::business::BusinessLayer;
use crate::model::{MercuryRecord, SearchCriteria, SortColumn};
use crate::persistence;
use std::io::{self, BufRead, Write};

const DATASET_FILE: &str =
    "NCP_ArcticMarineEcosystems_Mercury_Concentrations_EN_FR.csv";
const STUDENT_NAME: &str = "Soraya Megane Kaji";

/// Reads one trimmed line from stdin, flushing stdout first [1].
fn read_line() -> String {
    io::stdout().flush().ok();
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).ok();
    line.trim().to_string()
}

/// Reads a numeric index from stdin with the given prompt.
fn read_index(prompt: &str) -> usize {
    print!("{}", prompt);
    read_line().parse::<usize>().unwrap_or(0)
}

/// Prints the main application menu with student name and record count.
fn print_menu(count: usize) {
    println!();
    println!("+============================================================+");
    println!("|  CST8002 Practical Project 4 – Mercury Concentrations      |");
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
    println!("|  9) Sort records          [PP3 – Algorithms]               |");
    println!("|  10) Search/Filter records [PP4 – Novel Feature]           |");
    println!("+============================================================+");
    print!("  Choose an option (1-10): ");
}

/// Handles option 1 — reload data from the dataset CSV.
fn handle_reload(bl: &mut BusinessLayer) {
    println!("\n--- Reload Data ---");
    match persistence::load_records(DATASET_FILE) {
        Ok(records) => { let n = records.len(); bl.load(records);
                         println!("  Loaded {} record(s).", n); }
        Err(e)      => println!("  Error: {}", e),
    }
}

/// Handles option 2 — save in-memory data to a UUID-named CSV file.
fn handle_save(bl: &BusinessLayer) {
    println!("\n--- Save Data (UUID filename) ---");
    match persistence::save_records(bl.all()) {
        Ok(f) => println!("  Saved {} record(s) to '{}'.", bl.count(), f),
        Err(e) => println!("  Error: {}", e),
    }
}

/// Handles option 3 — display one record by 1-based index.
fn handle_display_one(bl: &BusinessLayer) {
    println!("\n--- Display One Record ---");
    let idx = read_index(&format!("  Enter record number (1–{}): ", bl.count()));
    match bl.get(idx) {
        Some(r) => { println!(); println!("{}", r); }
        None    => println!("  Invalid index."),
    }
}

/// Handles option 4 — display all records, student name every 10 rows.
fn handle_display_all(bl: &BusinessLayer) {
    println!("\n--- Display All {} Record(s) ---", bl.count());
    for (i, r) in bl.all().iter().enumerate() {
        if i % 10 == 0 { println!("  [Program by {}]", STUDENT_NAME); }
        println!("─── Record {} ──────────────────────────────────────", i + 1);
        println!("{}", r);
        println!();
    }
}

/// Prompts user to enter all six dataset field values for a record.
#[allow(non_snake_case)]
fn prompt_record() -> MercuryRecord {
    let read = |label: &str| -> String { print!("  {}: ", label); read_line() };
    MercuryRecord::new(
        read("SiteName"), read("SiteNumber"), read("Year"),
        read("Water_Column_Depth"), read("THg (ng/L)"), read("DMHg (pg/L)"),
    )
}

/// Handles option 5 — create a new record in memory.
fn handle_create(bl: &mut BusinessLayer) {
    println!("\n--- Create New Record ---");
    bl.create(prompt_record());
    println!("  Record added. Total: {}.", bl.count());
}

/// Handles option 6 — edit an existing record in memory.
fn handle_edit(bl: &mut BusinessLayer) {
    println!("\n--- Edit Record ---");
    let idx = read_index(&format!("  Enter record number to edit (1–{}): ", bl.count()));
    if let Some(r) = bl.get(idx) { println!("  Current:\n{}", r); } else {
        println!("  Invalid index."); return; }
    if bl.update(idx, prompt_record()) { println!("  Record {} updated.", idx); }
    else { println!("  Update failed."); }
}

/// Handles option 7 — delete a record from memory.
fn handle_delete(bl: &mut BusinessLayer) {
    println!("\n--- Delete Record ---");
    let idx = read_index(&format!("  Enter record number to delete (1–{}): ", bl.count()));
    if bl.delete(idx) { println!("  Record {} deleted. Total: {}.", idx, bl.count()); }
    else { println!("  Invalid index."); }
}

/// Handles option 9 — sort records by a user-selected column (PP3 feature).
fn handle_sort(bl: &mut BusinessLayer) {
    println!("\n--- Sort Records [PP3 – Algorithms] ---");
    println!("  1) SiteName  2) SiteNumber  3) Year  4) THg");
    print!("  Choose (1-4): ");
    let column = match read_line().as_str() {
        "1" => SortColumn::SiteName,
        "2" => SortColumn::SiteNumber,
        "3" => SortColumn::Year,
        "4" => SortColumn::THg,
        _   => { println!("  Invalid choice."); return; }
    };
    bl.sort(&column);
    println!("  {} record(s) sorted by {} (ascending).", bl.count(), column);
    println!("  Use option 4 (Display All) to view sorted records.");
}

/// Handles option 10 — multi-column search/filter (PP4 novel feature).
///
/// Prompts the user for optional search terms for SiteName, Year, and THg.
/// Pressing Enter on any field treats it as a wildcard. Calls
/// [`BusinessLayer::search`] which delegates to `model::filter_records`
/// using `Iterator::filter()` with chained AND conditions [2].
fn handle_search(bl: &BusinessLayer) {
    println!("\n--- Search / Filter Records [PP4 – Novel Feature] ---");
    println!("  Press Enter to skip a field (skip = match all for that column).");
    println!();

    print!("  SiteName contains: ");
    let sn = read_line();
    print!("  Year contains:     ");
    let yr = read_line();
    print!("  THg contains:      ");
    let th = read_line();

    let to_opt = |s: String| if s.is_empty() { None } else { Some(s) };
    let criteria = SearchCriteria::new(to_opt(sn), to_opt(yr), to_opt(th));

    if criteria.is_empty() {
        println!("  No criteria entered — showing all {} records.", bl.count());
    }

    let results = bl.search(&criteria);

    println!();
    println!("  ┌─ Search Results ─────────────────────────────────────────┐");
    println!("  │  Student: {:52}│", STUDENT_NAME);
    println!("  │  Matched {} of {} record(s)                              │",
             results.len(), bl.count());
    println!("  └──────────────────────────────────────────────────────────┘");
    println!();

    if results.is_empty() {
        println!("  No records matched the search criteria.");
    } else {
        for (i, r) in results.iter().enumerate() {
            println!("  ─── Match {} ──────────────────────────────────────", i + 1);
            println!("{}", r);
            println!();
        }
    }
}

/// Runs the main application loop until the user selects option 8 (Exit).
pub fn run() {
    println!("CST8002 Practical Project 4 – Mercury Concentrations");
    println!("Student: {}", STUDENT_NAME);
    println!("Novel feature: Search / Filter on multiple columns (PP4)");
    println!("Loading dataset...");

    let mut bl = BusinessLayer::new();
    match persistence::load_records(DATASET_FILE) {
        Ok(records) => { println!("  Loaded {} record(s).", records.len()); bl.load(records); }
        Err(e)      => { eprintln!("  Warning: {}", e);
                         eprintln!("  Use option 1 to reload."); }
    }

    loop {
        print_menu(bl.count());
        let choice = read_line();
        match choice.as_str() {
            "1"  => handle_reload(&mut bl),
            "2"  => handle_save(&bl),
            "3"  => handle_display_one(&bl),
            "4"  => handle_display_all(&bl),
            "5"  => handle_create(&mut bl),
            "6"  => handle_edit(&mut bl),
            "7"  => handle_delete(&mut bl),
            "8"  => { println!("\nExiting. Goodbye, {}!", STUDENT_NAME); break; }
            "9"  => handle_sort(&mut bl),
            "10" => handle_search(&bl),
            _    => println!("  Invalid option. Please enter 1–10."),
        }
    }
}
