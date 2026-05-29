use dioxus::prelude::*;
use kinetics::prelude::*;

// ---------------------------------------------------------------------------
// StreamingText
// ---------------------------------------------------------------------------

pub fn streaming_text_preview() -> Element {
    // The settled prefix is everything up to and including the largest in-range
    // byte boundary; the trailing chunk is wrapped in `.ui-stream-token`. We
    // place the boundary just before the final word so the static snapshot
    // shows a settled paragraph plus a freshly-faded tail token and the caret.
    const STREAMING_BODY: &str =
        "Revenue grew 18% quarter over quarter, driven mostly by enterprise renewals";
    // Byte offset of the last space, so the tail token is the final word.
    let boundary = STREAMING_BODY.rfind(' ').map(|i| i + 1).unwrap_or(0);

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Streaming · live tail + caret" }
                StreamingText {
                    text: STREAMING_BODY.to_string(),
                    streaming: true,
                    chunk_boundaries: vec![boundary],
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Settled · stream complete" }
                StreamingText {
                    text: "Revenue grew 18% quarter over quarter, driven mostly by enterprise renewals."
                        .to_string(),
                    streaming: false,
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AiStatus
// ---------------------------------------------------------------------------

pub fn ai_status_preview() -> Element {
    let states = [
        (AiStatusState::Idle, "Idle", "Ready"),
        (
            AiStatusState::Thinking,
            "Thinking",
            "Reasoning over your request…",
        ),
        (
            AiStatusState::Searching,
            "Searching",
            "Searching 4 sources…",
        ),
        (
            AiStatusState::Generating,
            "Generating",
            "Generating answer…",
        ),
        (AiStatusState::Done, "Done", "Done"),
    ];
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            for (state, variant, label) in states {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{variant}" }
                    AiStatus { state, label: label.to_string() }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CitationChip
// ---------------------------------------------------------------------------

pub fn citation_chip_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Linked · inline references" }
                p { style: "margin: 0; line-height: 1.6;",
                    "The Rust ownership model prevents data races at compile time"
                    CitationChip {
                        index: 1,
                        title: "The Rust Reference",
                        href: "https://doc.rust-lang.org/reference/",
                    }
                    " and is enforced by the borrow checker"
                    CitationChip {
                        index: 2,
                        title: "Rust Book · Ownership",
                        href: "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html",
                    }
                    "."
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Button · popover-driven (no href)" }
                p { style: "margin: 0; line-height: 1.6;",
                    "Async tasks are scheduled by the runtime"
                    CitationChip { index: 3, title: "Tokio · Internal scheduler" }
                    " rather than the OS."
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SourceCard / SourceRail
// ---------------------------------------------------------------------------

pub fn source_card_preview() -> Element {
    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Source rail · Perplexity-style" }
            }
            SourceRail {
                SourceCard {
                    index: 1,
                    title: "Understanding Ownership",
                    domain: "doc.rust-lang.org",
                    snippet: "Ownership is Rust's most unique feature and enables memory safety without a garbage collector.",
                    href: "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html",
                }
                SourceCard {
                    index: 2,
                    title: "Fearless Concurrency",
                    domain: "blog.rust-lang.org",
                    snippet: "The type system and ownership rules catch concurrency bugs at compile time.",
                    href: "https://blog.rust-lang.org/",
                }
                SourceCard {
                    index: 3,
                    title: "Async in depth",
                    domain: "tokio.rs",
                    snippet: "Futures are inert in Rust; they make progress only when polled by an executor.",
                    href: "https://tokio.rs/tokio/tutorial",
                }
                SourceCard {
                    index: 4,
                    title: "The Rustonomicon",
                    domain: "doc.rust-lang.org",
                    snippet: "The dark arts of unsafe Rust, for when the safe subset is not enough.",
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// PromptInput
// ---------------------------------------------------------------------------

pub fn prompt_input_preview() -> Element {
    rsx! { PromptInputPreviewBody {} }
}

#[component]
fn PromptInputPreviewBody() -> Element {
    // Seed each composer with real text so the static snapshot is not empty.
    // The idle tile renders the send affordance; the streaming tile flips the
    // action button to the square Stop control via `streaming: true`.
    let mut idle_value = use_signal(|| "Summarise this quarter's revenue drivers".to_string());
    let mut streaming_value = use_signal(|| "Drafting the executive summary".to_string());
    let mut streaming = use_signal(|| true);

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Idle composer · Enter to send" }
                PromptInput {
                    value: idle_value.read().clone(),
                    streaming: false,
                    placeholder: "Ask anything…",
                    on_input: move |next: String| idle_value.set(next),
                    on_submit: move |_submitted: String| idle_value.set(String::new()),
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Streaming · send toggles to Stop" }
                PromptInput {
                    value: streaming_value.read().clone(),
                    streaming: *streaming.read(),
                    placeholder: "Ask anything…",
                    on_input: move |next: String| streaming_value.set(next),
                    on_submit: move |_submitted: String| {},
                    on_stop: move |_| streaming.set(false),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AssistantPanel
// ---------------------------------------------------------------------------

pub fn assistant_panel_preview() -> Element {
    rsx! { AssistantPanelPreviewBody {} }
}

#[component]
fn AssistantPanelPreviewBody() -> Element {
    // Keep the panel open by default so the snapshot captures the full
    // Comet-style assistant; the close button / Escape re-hide it at runtime.
    let mut open = use_signal(|| true);
    let mut composer = use_signal(|| "What changed in the latest release?".to_string());

    const ANSWER: &str =
        "The 0.7 release adds AI-native surfaces: streaming text, source rails, and an agent timeline";
    let boundary = ANSWER.rfind(' ').map(|i| i + 1).unwrap_or(0);

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Assistant panel · open by default" }
                button {
                    class: "ui-button ui-button--secondary",
                    r#type: "button",
                    onclick: move |_| open.set(true),
                    "Reopen"
                }
            }
            AssistantPanel {
                open: *open.read(),
                side: AssistantSide::End,
                title: "Workspace assistant",
                on_dismiss: move |_| open.set(false),
                AiStatus { state: AiStatusState::Generating, label: "Generating answer…".to_string() }
                StreamingText {
                    text: ANSWER.to_string(),
                    streaming: true,
                    chunk_boundaries: vec![boundary],
                }
                SourceRail {
                    SourceCard {
                        index: 1,
                        title: "Release notes · 0.7",
                        domain: "github.com",
                        snippet: "New AI-native component category lands in the prelude.",
                        href: "https://github.com/",
                    }
                    SourceCard {
                        index: 2,
                        title: "Migration guide",
                        domain: "docs.rs",
                        snippet: "How to adopt the new surfaces in an existing app.",
                        href: "https://docs.rs/",
                    }
                }
                PromptInput {
                    value: composer.read().clone(),
                    streaming: false,
                    placeholder: "Reply to the assistant…",
                    on_input: move |next: String| composer.set(next),
                    on_submit: move |_submitted: String| composer.set(String::new()),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AgentTimeline
// ---------------------------------------------------------------------------

pub fn agent_timeline_preview() -> Element {
    let steps = vec![
        AgentStep::new("Parse the request", AgentStepState::Done),
        AgentStep::new("Search the knowledge base", AgentStepState::Done),
        AgentStep::new("Synthesise an answer", AgentStepState::Active),
        AgentStep::new("Cite sources", AgentStepState::Pending),
        AgentStep::new("Deliver response", AgentStepState::Pending),
    ];
    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Agent run · done / active / pending" }
            }
            AgentTimeline { steps }
        }
    }
}
