//! Perplexity-style answer surface: a labelled "Sources" rail, a streaming
//! answer body (with citation chips), and a "Related" follow-up list.
//! Composes the existing `SourceCard`/`SourceRail`/`StreamingText`/
//! `CitationChip` primitives into the answer-experience layout that
//! Perplexity-style AI products ship.

use dioxus::prelude::*;

use crate::ai::{CitationChip, SourceCard, SourceRail, StreamingText};

/// One source cited by an [`AnswerPanel`]. Mirrors `SourceCard`'s prop set
/// without coupling callers to render-time details.
#[derive(Clone, Debug, PartialEq)]
pub struct AnswerSource {
    pub title: String,
    pub domain: String,
    pub snippet: String,
    pub href: String,
    pub favicon: String,
}

impl AnswerSource {
    pub fn new(title: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            domain: domain.into(),
            snippet: String::new(),
            href: String::new(),
            favicon: String::new(),
        }
    }

    pub fn snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = snippet.into();
        self
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = href.into();
        self
    }

    pub fn favicon(mut self, favicon: impl Into<String>) -> Self {
        self.favicon = favicon.into();
        self
    }
}

/// A Perplexity-style answer: the question, a numbered sources rail, the
/// streaming answer body, and an optional "Related" follow-up list. Pass
/// `streaming: true` while tokens arrive; the body live-regions the tail.
#[component]
pub fn AnswerPanel(
    query: String,
    answer: String,
    #[props(default)] streaming: bool,
    #[props(default)] sources: Vec<AnswerSource>,
    #[props(default)] related: Vec<String>,
    on_select_related: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        article { class: "ui-answer-panel",
            h2 { class: "ui-answer-query", "{query}" }
            if !sources.is_empty() {
                section { class: "ui-answer-sources",
                    h3 { class: "ui-answer-section-label", "Sources" }
                    SourceRail {
                        for (index, source) in sources.iter().enumerate() {
                            SourceCard {
                                key: "{source.domain}-{index}",
                                index: (index as u32) + 1,
                                title: source.title.clone(),
                                domain: source.domain.clone(),
                                snippet: source.snippet.clone(),
                                href: source.href.clone(),
                                favicon: source.favicon.clone(),
                            }
                        }
                    }
                }
            }
            section { class: "ui-answer-body",
                h3 { class: "ui-answer-section-label",
                    if streaming { "Answering" } else { "Answer" }
                }
                StreamingText { text: answer, streaming: streaming }
                if !sources.is_empty() {
                    div { class: "ui-answer-citations", "aria-label": "Citations",
                        for (index, source) in sources.iter().enumerate() {
                            CitationChip {
                                key: "cite-{index}",
                                index: (index as u32) + 1,
                                title: source.title.clone(),
                                href: source.href.clone(),
                            }
                        }
                    }
                }
            }
            if !related.is_empty() {
                section { class: "ui-answer-related",
                    h3 { class: "ui-answer-section-label", "Related" }
                    RelatedQuestions { questions: related, on_select: on_select_related }
                }
            }
        }
    }
}

/// A vertical list of follow-up question chips (the Perplexity "Related"
/// block). Emits the chosen question via `on_select`.
#[component]
pub fn RelatedQuestions(
    questions: Vec<String>,
    on_select: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        ul { class: "ui-related-questions",
            for (index, question) in questions.iter().enumerate() {
                li { key: "{index}",
                    button {
                        class: "ui-related-question",
                        r#type: "button",
                        onclick: {
                            let handler = on_select;
                            let value = question.clone();
                            move |_| {
                                if let Some(h) = handler {
                                    h.call(value.clone());
                                }
                            }
                        },
                        span { class: "ui-related-question-plus", "aria-hidden": "true", "+" }
                        span { class: "ui-related-question-text", "{question}" }
                    }
                }
            }
        }
    }
}
