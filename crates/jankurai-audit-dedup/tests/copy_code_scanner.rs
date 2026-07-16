//! Integration and property tests for the copy-code scanner crate.
//!
//! These exercise the public, dependency-light surface of
//! `jankurai-audit-dedup` (`normalize_token_line` and `rust_unit_name`) with
//! both concrete `#[test]` cases and `proptest`-driven invariants, so the
//! scanner's tokenizer and unit-name extraction cannot silently regress.

use jankurai_audit_dedup::{normalize_token_line, rust_unit_name};
use proptest::prelude::*;

#[test]
fn normalize_collapses_internal_whitespace() {
    let a = normalize_token_line("let   x   =   1;");
    let b = normalize_token_line("let x = 1;");
    assert_eq!(a, b, "runs of whitespace must normalize identically");
}

#[test]
fn normalize_trims_leading_and_trailing_whitespace() {
    assert_eq!(
        normalize_token_line("   foo()  "),
        normalize_token_line("foo()")
    );
}

#[test]
fn rust_unit_name_reads_function_signatures() {
    assert_eq!(
        rust_unit_name("pub fn scan_repo(repo: &Path) -> Result<()> {"),
        Some("scan_repo".to_string()),
    );
    assert_eq!(
        rust_unit_name("    fn helper() {"),
        Some("helper".to_string())
    );
}

#[test]
fn rust_unit_name_ignores_plain_statements() {
    assert_eq!(rust_unit_name("let total = 0;"), None);
}

proptest! {
    /// Normalization is idempotent: normalizing an already-normalized line is a
    /// no-op. This is the invariant the token-block fingerprint relies on.
    #[test]
    fn normalize_is_idempotent(line in "[ -~]{0,120}") {
        let once = normalize_token_line(&line);
        let twice = normalize_token_line(&once);
        prop_assert_eq!(once, twice);
    }

    /// Extracted unit names never contain whitespace; they are bare identifiers.
    #[test]
    fn unit_names_have_no_whitespace(line in "[ -~]{0,120}") {
        if let Some(name) = rust_unit_name(&line) {
            prop_assert!(!name.chars().any(|c| c.is_whitespace()));
        }
    }
}
