//! `AiStatus` — a compact, animated indicator of the assistant's current
//! phase (thinking / searching / generating), announced politely.

use dioxus::prelude::*;

/// Phase of an assistant turn. Drives the `data-ai-state` attribute and
/// the default human-readable label.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AiStatusState {
    #[default]
    Idle,
    Thinking,
    Searching,
    Generating,
    Done,
}

impl AiStatusState {
    /// Stable slug used for the `data-ai-state` attribute and CSS hooks.
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Thinking => "thinking",
            Self::Searching => "searching",
            Self::Generating => "generating",
            Self::Done => "done",
        }
    }

    /// Default label shown when the caller does not supply one.
    pub const fn default_label(self) -> &'static str {
        match self {
            Self::Idle => "Ready",
            Self::Thinking => "Thinking…",
            Self::Searching => "Searching…",
            Self::Generating => "Generating…",
            Self::Done => "Done",
        }
    }
}

/// Inline status pill. Renders three pulsing dots for active phases; the
/// `Done` phase swaps the dots for a check glyph. The whole element is a
/// polite live region so phase changes are announced once.
#[component]
pub fn AiStatus(state: AiStatusState, #[props(default)] label: String) -> Element {
    let suffix = state.class_suffix();
    let text = if label.is_empty() {
        state.default_label().to_string()
    } else {
        label
    };
    let is_done = state == AiStatusState::Done;

    rsx! {
        div {
            class: "ui-ai-status",
            role: "status",
            "aria-live": "polite",
            "data-ai-state": "{suffix}",
            if is_done {
                svg {
                    class: "ui-ai-status-check",
                    "viewBox": "0 0 24 24",
                    width: "16",
                    height: "16",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    "aria-hidden": "true",
                    path { d: "M5 12l4 4 10-10" }
                }
            } else {
                span { class: "ui-ai-status-dot", "aria-hidden": "true" }
                span { class: "ui-ai-status-dot", "aria-hidden": "true" }
                span { class: "ui-ai-status-dot", "aria-hidden": "true" }
            }
            span { class: "ui-ai-status-label", "{text}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_suffix_maps_each_state() {
        assert_eq!(AiStatusState::Idle.class_suffix(), "idle");
        assert_eq!(AiStatusState::Thinking.class_suffix(), "thinking");
        assert_eq!(AiStatusState::Searching.class_suffix(), "searching");
        assert_eq!(AiStatusState::Generating.class_suffix(), "generating");
        assert_eq!(AiStatusState::Done.class_suffix(), "done");
    }

    #[test]
    fn default_label_is_nonempty() {
        for state in [
            AiStatusState::Idle,
            AiStatusState::Thinking,
            AiStatusState::Searching,
            AiStatusState::Generating,
            AiStatusState::Done,
        ] {
            assert!(!state.default_label().is_empty());
        }
    }
}
