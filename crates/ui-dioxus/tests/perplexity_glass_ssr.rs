use dioxus::prelude::*;
use ui_dioxus::{AnswerPanel, AnswerSource, LiquidGlass, RelatedQuestions};

#[test]
fn liquid_glass_renders_apple_surface() {
    let html = dioxus_ssr::render_element(rsx! {
        LiquidGlass { "Search the web" }
    });
    assert!(html.contains("ui-liquid-glass "), "{html}");
    assert!(html.contains("ui-liquid-glass--neutral"), "{html}");
}

#[test]
fn liquid_glass_tone_variant_applies_class() {
    let html = dioxus_ssr::render_element(rsx! {
        LiquidGlass { tone: ui_glass::GlassTone::Primary, "Go" }
    });
    assert!(html.contains("ui-liquid-glass--primary"), "{html}");
    assert!(html.contains(r#"data-glass-tone="primary""#), "{html}");
}

#[test]
fn answer_panel_renders_full_perplexity_layout() {
    let html = dioxus_ssr::render_element(rsx! {
        AnswerPanel {
            query: "What is Dioxus Kinetics?".to_string(),
            answer: "A Dioxus-first UI workspace with cinematic motion and liquid glass.".to_string(),
            sources: vec![
                AnswerSource::new("Dioxus Kinetics docs", "dioxus-kinetics.dev")
                    .snippet("The public facade crate.")
                    .href("https://example.com/dk"),
                AnswerSource::new("GitHub", "github.com"),
            ],
            related: vec!["How do I render a scene?".to_string(), "What is liquid glass?".to_string()],
        }
    });
    assert!(html.contains("ui-answer-panel"), "{html}");
    assert!(html.contains("What is Dioxus Kinetics?"), "{html}");
    // sources rail + cards
    assert!(html.contains("ui-source-rail"), "{html}");
    assert!(html.contains("Dioxus Kinetics docs"), "{html}");
    // streaming answer body
    assert!(html.contains("ui-stream"), "{html}");
    assert!(html.contains("cinematic motion"), "{html}");
    // citation chips for each source
    assert_eq!(html.matches("ui-citation-chip").count(), 2, "{html}");
    // related follow-ups
    assert!(html.contains("ui-related-questions"), "{html}");
    assert!(html.contains("How do I render a scene?"), "{html}");
}

#[test]
fn answer_panel_omits_sections_when_empty() {
    let html = dioxus_ssr::render_element(rsx! {
        AnswerPanel {
            query: "Hi".to_string(),
            answer: "Hello.".to_string(),
        }
    });
    assert!(!html.contains("ui-answer-sources"), "{html}");
    assert!(!html.contains("ui-answer-related"), "{html}");
    assert!(!html.contains("ui-answer-citations"), "{html}");
}

#[test]
fn answer_panel_marks_streaming_label() {
    let html = dioxus_ssr::render_element(rsx! {
        AnswerPanel {
            query: "Q".to_string(),
            answer: "so far".to_string(),
            streaming: true,
        }
    });
    assert!(html.contains("Answering"), "{html}");
    assert!(html.contains("ui-stream-caret"), "{html}");
}

#[test]
fn related_questions_renders_chips() {
    let html = dioxus_ssr::render_element(rsx! {
        RelatedQuestions { questions: vec!["A?".to_string(), "B?".to_string()] }
    });
    assert_eq!(html.matches("ui-related-question\"").count(), 2, "{html}");
    assert!(html.contains("A?"), "{html}");
}
