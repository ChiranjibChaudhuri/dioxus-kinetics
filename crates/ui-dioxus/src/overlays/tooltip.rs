//! Hover/focus tooltip with ARIA association via aria-describedby.

use dioxus::prelude::*;

#[component]
pub fn Tooltip(id: String, visible: bool, trigger_label: String, content: String) -> Element {
    let described_by = if visible { id.clone() } else { String::new() };

    rsx! {
        span { class: "ui-tooltip",
            span {
                class: "ui-tooltip-trigger",
                "aria-describedby": "{described_by}",
                "{trigger_label}"
            }
            if visible {
                span { id: "{id}", class: "ui-tooltip-content", role: "tooltip", "{content}" }
            }
        }
    }
}
