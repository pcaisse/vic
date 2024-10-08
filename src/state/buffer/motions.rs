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

// fn next_bigword(iter: impl Iterator<Item = (usize, &str)>) {
//     iter.tuple_windows()
//         .take_while_inclusive(|((_, curr_grapheme), (_, next_grapheme))| {
//             // Stop when the current grapheme is blank and the next grapheme isn't blank
//             !is_whitespace_str(curr_grapheme) || is_whitespace_str(next_grapheme)
//         })
//         .unzip()
// }

// "hi  there"
// (0, "i") (2, " ")
// (1, "i") (2, " ")
// (2, " ") (3, " ")
// (3, " ") (4, "t")

pub fn big_word_forwards(s: &str, i: usize) -> usize {
    let iter = grapheme_iter(s, i, true);
    // Find next bigword
    let (_current, next): (Vec<_>, Vec<_>) = iter
        .tuple_windows()
        .take_while_inclusive(|((_, curr_grapheme), (_, next_grapheme))| {
            // Stop when the current grapheme is blank and the next grapheme isn't blank
            !is_whitespace_str(curr_grapheme) || is_whitespace_str(next_grapheme)
        })
        .unzip();
    // Extract grapheme indices
    let (indices, _graphemes): (Vec<usize>, Vec<&str>) = next.into_iter().unzip();
    // Return last grapheme index offset by cursor position
    let index = indices.into_iter().max().unwrap_or_default();
    i + index
}

// "hi  there"
// (0, "h") (1, "t")
// (1, "t") (2, " ")
// (2, " ") (3, " ")
// (3, " ") (4, "i")

pub fn big_word_backwards(s: &str, i: usize) -> usize {
    let s_end_trimmed = s.trim_end();
    let mut iter = grapheme_iter(s_end_trimmed, i, false).peekable();
    let p = iter.peek();
    let result = match (s != s_end_trimmed, p) {
        (true, _) => {
            // We started on whitespace, so go to beginning of next bigword
            iter.take_while(|(_, grapheme)| !is_whitespace_str(grapheme))
                .collect()
        }
        (_, Some((_, next_grapheme))) if !is_whitespace_str(next_grapheme) => {
            // We started *not* on whitespace and the next grapheme is also not whitespace, such as
            // in the middle of a bigword, so go to end of the next bigword
            let (current, _next): (Vec<_>, Vec<_>) = iter
                .tuple_windows()
                .take_while_inclusive(|(_, (_, next_grapheme))| {
                    // Stop when the next grapheme is blank
                    !is_whitespace_str(next_grapheme)
                })
                .unzip();
            current
        }
        _ => {
            // We're at the beginning of a bigword, so keep going until we get to the end of
            // another bigword
            let (_current, next): (Vec<_>, Vec<_>) = iter
                .tuple_windows()
                .take_while_inclusive(|((_, curr_grapheme), (_, next_grapheme))| {
                    // Stop when the current grapheme is blank and the next grapheme isn't blank
                    !is_whitespace_str(curr_grapheme) || is_whitespace_str(next_grapheme)
                })
                .unzip();
            next
        }
    };
    // Extract grapheme indices
    let (indices, _graphemes): (Vec<usize>, Vec<&str>) = result.into_iter().unzip();
    // Return first grapheme index offset by cursor position
    let index = indices.into_iter().min().unwrap_or_default();
    i - (i - index)
}
