use state::buffer::Buffer;

#[test]
fn test_move_big_word_forwards() {
    let mut buffer = Buffer {
        text: String::from("hi   there"),
        grapheme_index: 1,
    };
    buffer.move_big_word_forwards();
    assert_eq!(buffer.grapheme_index, 5)
}
