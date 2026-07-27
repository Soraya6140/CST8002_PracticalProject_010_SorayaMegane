// =============================================================================
// File:       src/main.rs
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
// [1] git. (n.d.). "3.1 Git Branching – Branches in a Nutshell."
//     git-scm.com. [Online]. Available:
//     https://git-scm.com/book/en/v2/Git-Branching-Branches-in-a-Nutshell.
//     [Accessed: May 2026].
// [2] git. (n.d.). "2.6 Git Basics – Tagging." git-scm.com. [Online].
//     Available: https://git-scm.com/book/en/v2/Git-Basics-Tagging.
//     [Accessed: May 2026].
// [3] R. Oliveira. (Jul 31, 2024). "GUID vs UUID vs ULID." medium.com.
//     [Online]. Available:
//     https://medium.com/@ronaldo.oliver7/guid-vs-uuid-vs-ulid-understanding-unique-identifiers-565c88cdca13.
//     [Accessed: May 2026].
// [4] The Rust Foundation, "uuid crate." crates.io. [Online]. Available:
//     https://crates.io/crates/uuid. [Accessed: May 2026].
// [5] The Rust Foundation, "How to Write Tests." The Rust Programming
//     Language Book. [Online]. Available:
//     https://doc.rust-lang.org/book/ch11-01-writing-tests.html.
//     [Accessed: May 2026].
// [6] K. Fakhroutdinov. (2009-2024). "Multi-Layered Application: UML Model
//     Diagram Example." uml-diagrams.org. [Online]. Available:
//     https://www.uml-diagrams.org/multi-layered-application-uml-model-diagram-example.html.
//     [Accessed: May 2026].
// =============================================================================

//! # CST8002 Practical Project Part 2 – Entry Point
//!
//! N-Layered architecture with four packages:
//! - `model/`        – [`MercuryRecord`] entity / record-object
//! - `persistence/`  – CSV File-IO (read dataset, write UUID-named CSV)
//! - `business/`     – in-memory `Vec<MercuryRecord>` + CRUD + unit tests
//! - `presentation/` – all console interaction and menu loop

mod business;
mod model;
mod persistence;
mod presentation;

/// Application entry point — delegates immediately to [`presentation::run`].
fn main() {
    presentation::run();
}
