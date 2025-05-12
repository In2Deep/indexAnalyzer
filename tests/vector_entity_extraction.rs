//! RED test for vector entity extraction functionality
//! This test verifies that we can extract entities from Python code for vectorization

use indexer::extract_entities;

#[test]
fn test_extract_python_function() {
    let code = "def hello_world():\n    print('Hello, World!')\n";
    let entities = extract_entities(code);
    assert!(entities.contains(&"fn hello_world".to_string()), 
            "Should extract function name with 'fn' prefix");
}

#[test]
fn test_extract_python_class() {
    let code = "class MyClass:\n    def __init__(self):\n        pass\n";
    let entities = extract_entities(code);
    assert!(entities.contains(&"class MyClass".to_string()), 
            "Should extract class name with 'class' prefix");
    assert!(entities.contains(&"fn __init__".to_string()), 
            "Should extract method name with 'fn' prefix");
}

#[test]
fn test_extract_python_variable() {
    let code = "x = 10\ny = 'hello'\n";
    let entities = extract_entities(code);
    assert!(entities.contains(&"var x".to_string()), 
            "Should extract variable name with 'var' prefix");
    assert!(entities.contains(&"var y".to_string()), 
            "Should extract variable name with 'var' prefix");
}

#[test]
fn test_extract_python_docstring() {
    let code = "def documented():\n    \"\"\"This is a docstring\"\"\"\n    pass\n";
    let entities = extract_entities(code);
    assert!(entities.contains(&"fn documented".to_string()), 
            "Should extract function name with 'fn' prefix");
    assert!(entities.contains(&"doc documented: This is a docstring".to_string()), 
            "Should extract docstring with 'doc' prefix");
}

#[test]
fn test_extract_python_empty() {
    let code = "# Just a comment\n";
    let entities = extract_entities(code);
    assert!(entities.is_empty(), "Should return empty vector for code with no entities");
}
