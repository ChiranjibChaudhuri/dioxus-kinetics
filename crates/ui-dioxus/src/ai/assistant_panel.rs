//! `AssistantPanel` — a non-modal docked side panel for an assistant
//! conversation, dismissible via a close button or the Escape key.

use dioxus::prelude::*;

/// Which edge the assistant panel docks against.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AssistantSide {
    #[default]
    End,
    Start,
}

impl AssistantSide {
    /// Slug used for the `--docked-{slug}` modifier class.
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::End => "end",
            Self::Start => "start",
        }
    }
}

/// A complementary, non-modal docked panel. Because it is non-modal it
/// does NOT trap focus or render a backdrop — the rest of the page stays
/// interactive. Escape and the header close button both fire
/// `on_dismiss`. When `open` is false the component renders nothing.
#[component]
pub fn AssistantPanel(
    #[props(default = true)] open: bool,
    #[props(default)] side: AssistantSide,
    title: String,
    on_dismiss: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    let class = format!(
        "ui-assistant-panel ui-assistant-panel--docked-{}",
        side.class_suffix()
    );

    rsx! {
        aside {
            class: "{class}",
            role: "complementary",
            "aria-label": "{title}",
            "data-state": "open",
            onkeydown: move |evt| {
                if evt.key() == Key::Escape {
                    evt.stop_propagation();
                    if let Some(handler) = &on_dismiss {
                        handler.call(());
                    }
                }
            },
            header { class: "ui-assistant-panel-header",
                span { class: "ui-assistant-panel-title", "{title}" }
                button {
                    class: "ui-assistant-panel-close",
                    r#type: "button",
                    "aria-label": "Close",
                    onclick: move |_evt| {
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    },
                    svg {
                        "viewBox": "0 0 24 24",
                        width: "16",
                        height: "16",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        "aria-hidden": "true",
                        path { d: "M6 6l12 12M18 6L6 18" }
                    }
                }
            }
            div { class: "ui-assistant-panel-body",
                {children}
            }
            footer { class: "ui-assistant-panel-footer" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_suffix_maps_each_side() {
        assert_eq!(AssistantSide::End.class_suffix(), "end");
        assert_eq!(AssistantSide::Start.class_suffix(), "start");
    }

    #[test]
    fn default_side_is_end() {
        assert_eq!(AssistantSide::default(), AssistantSide::End);
    }
}
