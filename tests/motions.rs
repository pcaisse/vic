use state::buffer::motions::{big_word_backwards, big_word_forwards};

#[test]
fn test_big_word_forwards() {
    assert_eq!(big_word_forwards("hi", 0), 1);
    assert_eq!(big_word_forwards("hi", 1), 1);
    assert_eq!(big_word_forwards("hi\n", 0), 2);
    assert_eq!(big_word_forwards("hi   there", 1), 5);
    assert_eq!(big_word_forwards("hey   there", 1), 6);
    assert_eq!(big_word_forwards("", 0), 0);
    assert_eq!(big_word_forwards("नमस्ते foo bar", 0), 19);
}

#[test]
fn test_big_word_backwards() {
    assert_eq!(big_word_backwards("hi", 0), 0);
    assert_eq!(big_word_backwards("a b c", 4), 2);
    assert_eq!(big_word_backwards("a b c", 2), 0);
    assert_eq!(big_word_backwards("hello ", 5), 0);
    assert_eq!(big_word_backwards("hi   there", 5), 1);
    assert_eq!(big_word_backwards("hi   there", 6), 5);
    assert_eq!(big_word_backwards("hi   there", 9), 5);
    assert_eq!(big_word_backwards("", 0), 0);
    assert_eq!(big_word_backwards("नमस्ते foo bar", 19), 0);
}
