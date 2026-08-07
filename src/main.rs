// =============================================================================
// File:       src/main.rs
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
// [1] git. (2026, Jul. 28). 3.1 Git Branching – Branches in a Nutshell.
//     git-scm.com. [Online]. Available:
//     https://git-scm.com/book/en/v2/Git-Branching-Branches-in-a-Nutshell
//     [Accessed: Jul. 28, 2026].
// [2] git. (2026, Jul. 28). 2.6 Git Basics – Tagging. git-scm.com. [Online].
//     Available: https://git-scm.com/book/en/v2/Git-Basics-Tagging
//     [Accessed: Jul. 28, 2026].
// [3] The Rust Foundation. (2026, Jul. 28). Iterator::filter – Standard
//     library iterator adaptor. The Rust Standard Library. [Online].
//     Available: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
//     [Accessed: Jul. 28, 2026].
// [4] The Rust Foundation. (2026, Jul. 28). str::contains – Returns true if
//     the given pattern matches a sub-slice. The Rust Standard Library.
//     [Online]. Available:
//     https://doc.rust-lang.org/std/primitive.str.html#method.contains
//     [Accessed: Jul. 28, 2026].
// [5] The Rust Foundation. (2026, Jul. 28). Processing a Series of Items
//     with Iterators. The Rust Programming Language Book, Ch. 13. [Online].
//     Available: https://doc.rust-lang.org/book/ch13-02-iterators.html
//     [Accessed: Jul. 28, 2026].
// =============================================================================

//! # CST8002 Practical Project Part 4 – Entry Point
//!
//! Builds on Practical Project Parts 1, 2, and 3. Adds the PP4 novel
//! feature: **Search / Filter records based on multiple columns** using
//! Rust's `Iterator::filter()` API.
//!
//! N-Layered Architecture — four packages:
//! - `model/`        – MercuryRecord + SortColumn + SearchCriteria (NEW in PP4)
//!                     + sort_records (PP3) + filter_records (NEW in PP4)
//! - `persistence/`  – CSV File-IO (unchanged from PP3)
//! - `business/`     – Vec CRUD + sort (PP3) + search() method (NEW in PP4)
//! - `presentation/` – menu + option 9 Sort (PP3) + option 10 Search (NEW in PP4)

mod business;
mod model;
mod persistence;
mod presentation;

/// Application entry point — delegates to [`presentation::run`].
fn main() {
    presentation::run();
}
