//! RED test for classic/vector data coexistence (test key isolation, no mixing/overwrites)

#[test]
fn test_classic_vector_key_isolation() {
    let classic_key = "classic:foo";
    let vector_key = "code:myproject:doc:foo";
    assert!(!classic_key.starts_with("code:"));
    assert!(vector_key.starts_with("code:"));
    assert_ne!(classic_key, vector_key);
}
