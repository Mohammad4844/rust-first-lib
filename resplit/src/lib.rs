//! This is a library that provides utilities for command line tools
//! # Examples:
//! ```
//! use resplit::read_stdin;
//! let input = read_stdin();
//! ```

use std::io::{BufRead, BufReader};

/// This function reads a line from the standard input and returns it as a String.
/// It will panic if it fails to read a line.
/// # Examples:
/// ```
/// let input = resplit::read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}
