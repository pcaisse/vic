pub mod motions;

use self::motions::big_word;

#[derive(Default)]
pub struct Buffer {
    pub text: String,
    pub grapheme_index: usize,
}

impl Buffer {
    pub fn append(&mut self, c: char) {
        self.text.push(c);
        self.grapheme_index += 1;
    }

    pub fn move_big_word_forwards(&mut self) {
        let new_index = big_word(&self.text, self.grapheme_index);
        self.grapheme_index = new_index;
    }
}
