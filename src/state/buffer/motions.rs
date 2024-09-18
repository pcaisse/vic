// See vi spec: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/vi.html
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

fn is_whitespace_str(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}

// (0, "h") (1, "i")
// (1, "i") (2, " ")
// (2, " ") (3, "t")

pub fn big_word(s: &str, i: usize) -> (usize, &str) {
    let iter = s[i..].grapheme_indices(true);
    let (_current, next): (Vec<_>, Vec<_>) = iter
        .tuple_windows()
        .take_while_inclusive(|((_, curr_grapheme), (_, next_grapheme))| {
            !is_whitespace_str(curr_grapheme) || is_whitespace_str(next_grapheme)
        })
        .unzip();
    let (indices, _graphemes): (Vec<usize>, Vec<&str>) = next.into_iter().unzip();
    let index = indices.into_iter().max().unwrap_or_default();
    (i + index, &s[i..index])
}
