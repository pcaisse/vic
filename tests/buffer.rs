use state::buffer::{is_whitespace_str, Buffer};

#[test]
fn test_is_whitespace_str() {
    assert!(is_whitespace_str(""));
    assert!(is_whitespace_str(" "));
    assert!(is_whitespace_str(" "));
    assert!(is_whitespace_str("   "));
    assert!(!is_whitespace_str("a"));
}

#[test]
fn test_big_word() {
    let buffer = Buffer {
        text: String::from("hi   there"),
    };
    assert_eq!(buffer.big_word(), (5, "hi   "))
}

#[test]
fn test_big_word_empty() {
    let buffer = Buffer {
        text: String::new(),
    };
    assert_eq!(buffer.big_word(), (0, ""))
}

#[test]
fn test_big_word_unicode() {
    let buffer = Buffer {
        text: String::from("नमस्ते foo"),
    };
    assert_eq!(buffer.big_word(), (19, "नमस्ते "))
}
