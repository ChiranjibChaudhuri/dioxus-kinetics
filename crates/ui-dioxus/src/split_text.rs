//! `SplitText` — emits per-character or per-word `<span>` children with a
//! sequential `data-stagger-index` so the TimelineScope stagger machinery
//! can drive them. The parent carries the full text as `aria-label`; the
//! glyph / word spans are `aria-hidden` so screen readers read the
//! unsplit text exactly once.

use dioxus::prelude::*;

/// How `SplitText` chops the input string. `Character` splits on Unicode
/// grapheme-ish boundaries (per-`char`); `Word` splits on whitespace runs
/// while preserving the whitespace itself as literal text nodes between
/// word spans (so visual spacing survives the split).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SplitMode {
    /// One span per Unicode `char` in the source string.
    #[default]
    Character,
    /// One span per whitespace-delimited word. Whitespace runs are
    /// emitted as literal text nodes between word spans.
    Word,
}

impl SplitMode {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Character => "character",
            Self::Word => "word",
        }
    }
}

#[component]
pub fn SplitText(text: String, split_by: Option<SplitMode>) -> Element {
    let mode = split_by.unwrap_or_default();
    let mode_str = mode.as_str();
    let aria = text.clone();

    match mode {
        SplitMode::Character => {
            let glyphs: Vec<(usize, String)> = text
                .chars()
                .enumerate()
                .map(|(idx, ch)| (idx, ch.to_string()))
                .collect();

            rsx! {
                span {
                    class: "ui-split-text",
                    "aria-label": "{aria}",
                    "data-split-mode": "{mode_str}",
                    for (idx, glyph) in glyphs.into_iter() {
                        span {
                            key: "{idx}",
                            class: "ui-split-text__glyph",
                            "data-stagger-index": "{idx}",
                            "aria-hidden": "true",
                            "{glyph}"
                        }
                    }
                }
            }
        }
        SplitMode::Word => {
            // Tokenize the source into a sequence of runs that are either
            // a word (non-whitespace) or a whitespace gap. Word runs get a
            // sequential `data-stagger-index`; whitespace runs are emitted
            // verbatim as text nodes between word spans.
            #[derive(Clone)]
            enum Run {
                Word { index: usize, value: String },
                Gap(String),
            }

            let mut runs: Vec<Run> = Vec::new();
            let mut current = String::new();
            let mut current_is_ws = false;
            let mut word_index: usize = 0;

            for ch in text.chars() {
                let is_ws = ch.is_whitespace();
                if current.is_empty() {
                    current.push(ch);
                    current_is_ws = is_ws;
                    continue;
                }
                if is_ws == current_is_ws {
                    current.push(ch);
                } else {
                    if current_is_ws {
                        runs.push(Run::Gap(std::mem::take(&mut current)));
                    } else {
                        runs.push(Run::Word {
                            index: word_index,
                            value: std::mem::take(&mut current),
                        });
                        word_index += 1;
                    }
                    current.push(ch);
                    current_is_ws = is_ws;
                }
            }
            if !current.is_empty() {
                if current_is_ws {
                    runs.push(Run::Gap(current));
                } else {
                    runs.push(Run::Word {
                        index: word_index,
                        value: current,
                    });
                }
            }

            rsx! {
                span {
                    class: "ui-split-text",
                    "aria-label": "{aria}",
                    "data-split-mode": "{mode_str}",
                    for (slot, run) in runs.into_iter().enumerate() {
                        {
                            match run {
                                Run::Word { index, value } => rsx! {
                                    span {
                                        key: "w-{slot}",
                                        class: "ui-split-text__word",
                                        "data-stagger-index": "{index}",
                                        "aria-hidden": "true",
                                        "{value}"
                                    }
                                },
                                Run::Gap(value) => rsx! {
                                    {value}
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
