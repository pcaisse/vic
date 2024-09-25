use state::buffer::motions::big_word;

#[test]
fn test_big_word() {
    assert_eq!(big_word("hi", 0), 1);
    assert_eq!(big_word("hi", 1), 1);
    assert_eq!(big_word("hi\n", 0), 2);
    assert_eq!(big_word("hi   there", 1), 5);
    assert_eq!(big_word("", 0), 0);
    assert_eq!(big_word("नमस्ते foo bar", 0), 19);
}
