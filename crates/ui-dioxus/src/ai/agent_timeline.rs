//! `AgentTimeline` — an ordered list visualising an agent run's steps,
//! each marked pending / active / done with a node glyph and connector.

use dioxus::prelude::*;

/// Lifecycle state of a single agent step.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentStepState {
    #[default]
    Pending,
    Active,
    Done,
}

impl AgentStepState {
    /// Slug used for the step's `data-state` attribute and CSS hooks.
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Done => "done",
        }
    }
}

/// One step in an `AgentTimeline`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentStep {
    pub label: String,
    pub state: AgentStepState,
}

impl AgentStep {
    pub fn new(label: impl Into<String>, state: AgentStepState) -> Self {
        Self {
            label: label.into(),
            state,
        }
    }
}

/// Vertical run timeline. Each step shows a node (check when done, a
/// filled ring when active, a hollow ring when pending), the label, and
/// a connector to the next step (omitted on the last step, and hidden
/// from assistive tech).
#[component]
pub fn AgentTimeline(steps: Vec<AgentStep>) -> Element {
    let last = steps.len().saturating_sub(1);

    rsx! {
        ol { class: "ui-agent-timeline",
            for (i, step) in steps.iter().enumerate() {
                {
                    let suffix = step.state.class_suffix();
                    let label = step.label.clone();
                    let is_done = step.state == AgentStepState::Done;
                    let is_active = step.state == AgentStepState::Active;
                    let is_last = i == last;
                    let state_text = match step.state {
                        AgentStepState::Pending => "pending",
                        AgentStepState::Active => "in progress",
                        AgentStepState::Done => "completed",
                    };
                    rsx! {
                        li {
                            key: "{i}",
                            class: "ui-agent-timeline-step",
                            "data-state": "{suffix}",
                            "aria-current": if is_active { "step" } else { "" },
                            span { class: "ui-agent-timeline-node", "aria-hidden": "true",
                                if is_done {
                                    svg {
                                        "viewBox": "0 0 24 24",
                                        width: "14",
                                        height: "14",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "stroke-width": "2.5",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        path { d: "M5 12l4 4 10-10" }
                                    }
                                } else if is_active {
                                    span { class: "ui-agent-timeline-ring ui-agent-timeline-ring--filled" }
                                } else {
                                    span { class: "ui-agent-timeline-ring ui-agent-timeline-ring--hollow" }
                                }
                            }
                            span { class: "ui-agent-timeline-label", "{label}" }
                            span { class: "visually-hidden", " — {state_text}" }
                            if !is_last {
                                span { class: "ui-agent-timeline-connector", "aria-hidden": "true" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_suffix_maps_each_state() {
        assert_eq!(AgentStepState::Pending.class_suffix(), "pending");
        assert_eq!(AgentStepState::Active.class_suffix(), "active");
        assert_eq!(AgentStepState::Done.class_suffix(), "done");
    }
}
