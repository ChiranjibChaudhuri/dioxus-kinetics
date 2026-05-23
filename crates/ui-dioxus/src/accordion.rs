//! Accordion — collapsible content sections with WAI-ARIA disclosure semantics.

use dioxus::prelude::*;

/// One section of an `Accordion`. `id` round-trips through `expanded` /
/// `on_toggle`; `header` is the visible trigger text; `body` is the
/// content rendered when expanded. `disabled` greys out the section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccordionSection {
    pub id: String,
    pub header: String,
    pub body: String,
    pub disabled: bool,
}

impl AccordionSection {
    pub fn new(id: impl Into<String>, header: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            body: body.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Multi-section accordion with single- or multi-expand behaviour.
///
/// Pass `expanded` as a slice of currently-open section ids; emit
/// `on_toggle(id)` and let the consumer decide whether to enforce
/// single-expand (replace the set with `[id]`) or multi-expand
/// (toggle the id in/out of the set).
///
/// Uses the WAI-ARIA disclosure pattern: each header is a `<button>`
/// with `aria-expanded` reflecting the open state and `aria-controls`
/// pointing at the body region's id.
#[component]
pub fn Accordion(
    sections: Vec<AccordionSection>,
    #[props(default)] expanded: Vec<String>,
    on_toggle: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        div { class: "ui-accordion",
            for section in sections {
                {
                    let id = section.id.clone();
                    let region_id = format!("ui-accordion-region-{}", section.id);
                    let trigger_id = format!("ui-accordion-trigger-{}", section.id);
                    let is_open = expanded.iter().any(|e| e == &section.id);
                    let class = if is_open {
                        "ui-accordion-section ui-accordion-section--open"
                    } else {
                        "ui-accordion-section"
                    };
                    rsx! {
                        section { class: "{class}",
                            h3 { class: "ui-accordion-heading",
                                button {
                                    id: "{trigger_id}",
                                    class: "ui-accordion-trigger",
                                    r#type: "button",
                                    "aria-expanded": if is_open { "true" } else { "false" },
                                    "aria-controls": "{region_id}",
                                    disabled: section.disabled,
                                    onclick: move |_| {
                                        if !section.disabled {
                                            if let Some(handler) = &on_toggle {
                                                handler.call(id.clone());
                                            }
                                        }
                                    },
                                    span { class: "ui-accordion-marker", "aria-hidden": "true",
                                        if is_open { "−" } else { "+" }
                                    }
                                    span { class: "ui-accordion-label", "{section.header}" }
                                }
                            }
                            if is_open {
                                div {
                                    id: "{region_id}",
                                    class: "ui-accordion-region",
                                    role: "region",
                                    "aria-labelledby": "{trigger_id}",
                                    "{section.body}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
