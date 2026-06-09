//! Regression coverage for the `aria-current` idiom: inactive steps,
//! pages, and timeline nodes must render `aria-current="false"` (a literal
//! WAI-ARIA token) rather than `aria-current=""`, which assistive tech
//! maps to the default token `"true"`.

use dioxus::prelude::*;
use ui_dioxus::{AgentStep, AgentStepState, AgentTimeline, Pagination, Stepper, StepperStep};

#[test]
fn inactive_stepper_step_renders_aria_current_false() {
    let html = dioxus_ssr::render_element(rsx! {
        Stepper {
            current: "two",
            steps: vec![
                StepperStep::new("one", "One"),
                StepperStep::new("two", "Two"),
                StepperStep::new("three", "Three"),
            ],
        }
    });

    // The active step is "current"; the inactive steps must be explicit.
    assert!(html.contains("aria-current=\"false\""));
    assert!(html.contains("aria-current=\"step\""));
    assert!(!html.contains("aria-current=\"\""));
}

#[test]
fn inactive_pagination_button_renders_aria_current_false() {
    let html = dioxus_ssr::render_element(rsx! {
        Pagination {
            page: 1,
            total_pages: 5,
        }
    });

    assert!(html.contains("aria-current=\"false\""));
    assert!(html.contains("aria-current=\"page\""));
    assert!(!html.contains("aria-current=\"\""));
}

#[test]
fn inactive_agent_timeline_step_renders_aria_current_false() {
    let html = dioxus_ssr::render_element(rsx! {
        AgentTimeline {
            steps: vec![
                AgentStep::new("Plan", AgentStepState::Done),
                AgentStep::new("Execute", AgentStepState::Active),
                AgentStep::new("Verify", AgentStepState::Pending),
            ],
        }
    });

    assert!(html.contains("aria-current=\"false\""));
    assert!(html.contains("aria-current=\"step\""));
    assert!(!html.contains("aria-current=\"\""));
}
