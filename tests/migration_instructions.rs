//! RED test for migration instructions if schema changes are required

#[test]
fn test_migration_instructions_present() {
    let docs = include_str!("../docs/roadmap.md");
    assert!(docs.contains("migration"), "Migration instructions should be documented in roadmap.md");
}
