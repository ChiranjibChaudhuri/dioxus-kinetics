//! `SplitText` — emits per-character or per-word `<span>` children with a
//! sequential `data-stagger-index` so the TimelineScope stagger machinery
//! can drive them. The parent carries the full text as `aria-label`; the
//! glyph / word spans are `aria-hidden` so screen readers read the
//! unsplit text exactly once.

use dioxus::prelude::*;

use crate::cue_style::cue_inline_style;
use crate::scene_player::SceneContext;
use crate::stagger::StaggerCursor;

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
pub fn SplitText(
    text: String,
    split_by: Option<SplitMode>,
    #[props(default = "rise-in".to_string())] cue: String,
) -> Element {
    let mode = split_by.unwrap_or_default();
    let mode_str = mode.as_str();
    let aria = text.clone();

    // Look up the surrounding contexts. When no SceneContext is present
    // the per-glyph styles stay empty (graceful no-op).
    let cursor = try_consume_context::<StaggerCursor>();
    let scene = try_consume_context::<SceneContext>();

    let parent_elapsed = scene
        .as_ref()
        .map(|s| *s.clock.elapsed_ms.read())
        .unwrap_or(0.0);

    let step_ms = cursor.as_ref().map(|c| c.step_ms).unwrap_or(80.0);
    let base_offset_ms = cursor
        .as_ref()
        .map(|c| c.current_offset_ms())
        .unwrap_or(0.0);

    // Reserve this SplitText's slot in the surrounding stagger cursor so
    // siblings after this SplitText continue past it. Single-advance — we
    // don't claim N slots for N glyphs because the surrounding
    // TimelineScope is designed for per-component stagger, not per-glyph
    // fanout. Internal glyph stagger is handled by the local index `i`
    // offsetting below.
    if let Some(c) = cursor.as_ref() {
        c.next_index();
    }

    let has_scene = scene.is_some();
    let cue_for_style = cue.clone();
    let style_for_index = move |i: usize| -> String {
        if !has_scene {
            return String::new();
        }
        let local = (parent_elapsed - base_offset_ms - (i as f32) * step_ms).max(0.0);
        cue_inline_style(&cue_for_style, local)
    };

    match mode {
        SplitMode::Character => {
            let glyphs: Vec<(usize, String, String)> = text
                .chars()
                .enumerate()
                .map(|(idx, ch)| (idx, ch.to_string(), style_for_index(idx)))
                .collect();

            rsx! {
                span {
                    class: "ui-split-text",
                    "aria-label": "{aria}",
                    "data-split-mode": "{mode_str}",
                    for (idx, glyph, style) in glyphs.into_iter() {
                        span {
                            key: "{idx}",
                            class: "ui-split-text__glyph",
                            "data-stagger-index": "{idx}",
                            "aria-hidden": "true",
                            style: "{style}",
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
                Word {
                    index: usize,
                    value: String,
                    style: String,
                },
                Gap(String),
            }

            let mut runs: Vec<Run> = Vec::new();
            let mut current = String::new();
            let mut current_is_ws = false;
            let mut word_index: usize = 0;

            let push_word = |runs: &mut Vec<Run>, word_index: &mut usize, value: String| {
                let style = style_for_index(*word_index);
                runs.push(Run::Word {
                    index: *word_index,
                    value,
                    style,
                });
                *word_index += 1;
            };

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
                        push_word(&mut runs, &mut word_index, std::mem::take(&mut current));
                    }
                    current.push(ch);
                    current_is_ws = is_ws;
                }
            }
            if !current.is_empty() {
                if current_is_ws {
                    runs.push(Run::Gap(current));
                } else {
                    push_word(&mut runs, &mut word_index, current);
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
                                Run::Word { index, value, style } => rsx! {
                                    span {
                                        key: "w-{slot}",
                                        class: "ui-split-text__word",
                                        "data-stagger-index": "{index}",
                                        "aria-hidden": "true",
                                        style: "{style}",
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
