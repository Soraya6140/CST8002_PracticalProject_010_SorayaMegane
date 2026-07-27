// =============================================================================
// File:       src/main.rs
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
// [1] git. (2026, Jul. 8). 3.1 Git Branching – Branches in a Nutshell.
//     git-scm.com. [Online]. Available:
//     https://git-scm.com/book/en/v2/Git-Branching-Branches-in-a-Nutshell
//     [Accessed: Jul. 8, 2026].
// [2] git. (2026, Jul. 8). 2.6 Git Basics – Tagging. git-scm.com. [Online].
//     Available: https://git-scm.com/book/en/v2/Git-Basics-Tagging
//     [Accessed: Jul. 8, 2026].
// [3] Geeks For Geeks. (2026, Jul. 8). Types of Linked List.
//     geeksforgeeks.org. [Online]. Available:
//     https://www.geeksforgeeks.org/dsa/types-of-linked-list/
//     [Accessed: Jul. 8, 2026].
// [4] The Rust Foundation. (2026, Jul. 8). std::vec::Vec::sort_by. The Rust
//     Standard Library. [Online]. Available:
//     https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
//     [Accessed: Jul. 8, 2026].
// [5] The Rust Foundation. (2026, Jul. 8). How to Write Tests. The Rust
//     Programming Language Book. [Online]. Available:
//     https://doc.rust-lang.org/book/ch11-01-writing-tests.html
//     [Accessed: Jul. 8, 2026].
// =============================================================================

//! # CST8002 Practical Project Part 3 – Entry Point
//!
//! Builds on Practical Project Part 2 by adding the **Algorithms (sorting)**
//! advanced topic. The program retains the full N-Layered architecture from
//! Part 2:
//!
//! - `model/`        – [`MercuryRecord`] entity + [`SortColumn`] enum +
//!                     [`sort_records`] function (NEW in PP3)
//! - `persistence/`  – CSV File-IO (unchanged from PP2)
//! - `business/`     – Vec<MercuryRecord> + CRUD + `sort()` method (NEW) +
//!                     4 unit tests (expanded from PP2)
//! - `presentation/` – interactive menu + new option 9 Sort (NEW)
//!
//! Execution is delegated immediately to [`presentation::run`].

mod business;
mod model;
mod persistence;
mod presentation;

/// Application entry point — delegates to [`presentation::run`].
fn main() {
    presentation::run();
}
