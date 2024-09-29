// See vi spec: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/vi.html
use auto_enums::auto_enum;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

fn is_whitespace_str(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}

#[auto_enum(Iterator)]
fn grapheme_iter(s: &str, i: usize, is_forwards: bool) -> impl Iterator<Item = (usize, &str)> {
    let text = if is_forwards { &s[i..] } else { &s[..i] };
    let iter = text.grapheme_indices(true);
    match is_forwards {
        true => iter,
        false => iter.rev(),
    }
}

fn big_word(s: &str, i: usize, is_forwards: bool) -> usize {
    let iter = grapheme_iter(s, i, is_forwards);
    // Find next bigword
    let (_current, next): (Vec<_>, Vec<_>) = iter
        .tuple_windows()
        .take_while_inclusive(|((_, curr_grapheme), (_, next_grapheme))| {
            !is_whitespace_str(curr_grapheme) || is_whitespace_str(next_grapheme)
        })
        .unzip();
    // Extract grapheme indices
    let (indices, _graphemes): (Vec<usize>, Vec<&str>) = next.into_iter().unzip();
    if is_forwards {
        // Return last grapheme index offset by cursor position
        let index = indices.into_iter().max().unwrap_or_default();
        i + index
    } else {
        // Return first grapheme index offset by cursor position
        let index = indices.into_iter().min().unwrap_or_default();
        i - (i - index)
    }
}

pub fn big_word_forwards(s: &str, i: usize) -> usize {
    big_word(s, i, true)
}

pub fn big_word_backwards(s: &str, i: usize) -> usize {
    big_word(s, i, false)
}
