// See vi spec: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/vi.html
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Buffer {
    pub text: String,
    // TODO: Use string slices to safely access substrings, iterating over _graphemes_
    // pub byte_offset: usize,
}

pub fn is_whitespace_str(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}

// (0, "h") (1, "i")
// (1, "i") (2, " ")
// (2, " ") (3, "t")

impl Buffer {
    pub fn big_word(&self) -> (usize, &str) {
        let iter = self.text.grapheme_indices(true);
        let (_current, next): (Vec<_>, Vec<_>) = iter
            .tuple_windows()
            .take_while_inclusive(|((_, curr_grapheme), (_, next_grapheme))| {
                !is_whitespace_str(curr_grapheme) || is_whitespace_str(next_grapheme)
            })
            .unzip();
        let (indices, _graphemes): (Vec<usize>, Vec<&str>) = next.into_iter().unzip();
        let index = indices.into_iter().max().unwrap_or_default();
        (index, &self.text[0..index])
    }
}
