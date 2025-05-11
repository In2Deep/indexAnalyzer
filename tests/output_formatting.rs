//! RED test for output formatting (human-readable, JSON flag)

use indexer::output_format::{format_human_readable, format_json};

#[test]
fn test_output_format_human_readable() {
    let results = vec![("foo", 0.9), ("bar", 0.8)];
    let output = format_human_readable(&results);
    assert!(output.contains("foo: 0.9"));
    assert!(output.contains("bar: 0.8"));
}

#[test]
fn test_output_format_json() {
    let results = vec![("foo", 0.9), ("bar", 0.8)];
    let output = format_json(&results);
    assert!(output.contains("\"foo\""));
    assert!(output.contains("0.9"));
}
