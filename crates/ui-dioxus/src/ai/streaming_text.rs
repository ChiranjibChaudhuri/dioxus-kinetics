//! `StreamingText` — incremental assistant output with a settled prefix,
//! a highlighted latest chunk, and a blinking caret while streaming.

use dioxus::prelude::*;

/// Split `text` into the settled prefix (everything up to and including
/// the last chunk boundary) and the trailing latest chunk.
///
/// `boundaries` are byte offsets into `text` marking the end of each
/// settled chunk. The largest in-range boundary wins; out-of-range or
/// non-char-boundary offsets are ignored. When no usable boundary
/// exists the entire string is treated as the tail (latest chunk), so a
/// brand-new stream highlights all of its text.
pub fn split_settled_and_tail(text: &str, boundaries: &[usize]) -> (String, String) {
    let cut = boundaries
        .iter()
        .copied()
        .filter(|&b| b <= text.len() && text.is_char_boundary(b))
        .max()
        .unwrap_or(0);
    let (settled, tail) = text.split_at(cut);
    (settled.to_string(), tail.to_string())
}

/// Streaming assistant text. The settled prefix renders as plain text;
/// the latest chunk is wrapped in `span.ui-stream-token` so CSS can
/// fade it in. While `streaming`, a `span.ui-stream-caret` blinks after
/// the tail. The whole block is a polite, non-atomic live region so
/// assistive tech announces only the newly appended text.
#[component]
pub fn StreamingText(
    text: String,
    #[props(default)] streaming: bool,
    #[props(default)] chunk_boundaries: Vec<usize>,
) -> Element {
    let (settled, tail) = split_settled_and_tail(&text, &chunk_boundaries);

    rsx! {
        div {
            class: "ui-stream",
            role: "status",
            "aria-live": "polite",
            "aria-atomic": "false",
            if !settled.is_empty() {
                "{settled}"
            }
            if !tail.is_empty() {
                span { class: "ui-stream-token", "{tail}" }
            }
            if streaming {
                span { class: "ui-stream-caret", "aria-hidden": "true" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_boundaries_treats_all_as_tail() {
        let (settled, tail) = split_settled_and_tail("hello world", &[]);
        assert_eq!(settled, "");
        assert_eq!(tail, "hello world");
    }

    #[test]
    fn largest_in_range_boundary_wins() {
        let (settled, tail) = split_settled_and_tail("hello world", &[5, 2]);
        assert_eq!(settled, "hello");
        assert_eq!(tail, " world");
    }

    #[test]
    fn out_of_range_and_non_char_boundaries_are_ignored() {
        // "é" is 2 bytes; offset 1 is not a char boundary, 99 is past end.
        let (settled, tail) = split_settled_and_tail("é!", &[1, 99]);
        assert_eq!(settled, "");
        assert_eq!(tail, "é!");
    }
}
