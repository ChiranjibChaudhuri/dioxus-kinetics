//! Popover — stateful overlay anchored to a trigger element.
//!
//! This is the lightest viable popover primitive. It is **controlled**
//! (the consumer owns `open`) and renders an inline `position: absolute`
//! panel positioned relative to its trigger via CSS — no Floating-UI-style
//! viewport-flip math yet. That arrives when `Select` / `DatePicker` need
//! anchor-aware positioning under viewport edges (a future spec).
//!
//! For now, Popover is enough to host menus, color swatches, filter
//! pickers, and any other simple anchored overlay. The anchor positions
//! the panel via `PopoverSide` and the host stylesheet handles spacing.

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverSide {
    Top,
    #[default]
    Bottom,
    Start,
    End,
}

impl PopoverSide {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Top => "ui-popover ui-popover--top",
            Self::Bottom => "ui-popover ui-popover--bottom",
            Self::Start => "ui-popover ui-popover--start",
            Self::End => "ui-popover ui-popover--end",
        }
    }
}

#[component]
pub fn Popover(
    /// Stable id for the popover panel. The trigger receives
    /// `aria-controls={id}` and the panel receives `id={id}`.
    id: String,
    /// Whether the panel is visible. Consumer-controlled.
    #[props(default)]
    open: bool,
    /// Side of the trigger to anchor the panel against.
    #[props(default)]
    side: PopoverSide,
    /// Click-outside / Escape dismisses the panel when true (default).
    /// When false, the consumer is responsible for closing the panel.
    #[props(default = true)]
    dismissible: bool,
    /// The interactive element that opens the popover (typically a `Button`).
    trigger: Element,
    /// The panel contents.
    children: Element,
    on_open_change: Option<EventHandler<bool>>,
) -> Element {
    let panel_class = side.class_name();
    let aria_expanded = if open { "true" } else { "false" };

    rsx! {
        div { class: "ui-popover-root", "data-state": if open { "open" } else { "closed" },
            div {
                class: "ui-popover-trigger",
                "aria-haspopup": "dialog",
                "aria-expanded": "{aria_expanded}",
                "aria-controls": "{id}",
                onclick: move |_| {
                    if let Some(handler) = &on_open_change {
                        handler.call(!open);
                    }
                },
                {trigger}
            }
            if open {
                div {
                    id: "{id}",
                    class: "{panel_class}",
                    role: "dialog",
                    "data-side": "{side_attr(side)}",
                    onkeydown: move |evt| {
                        if dismissible && evt.key() == Key::Escape {
                            evt.stop_propagation();
                            if let Some(handler) = &on_open_change {
                                handler.call(false);
                            }
                        }
                    },
                    {children}
                }
            }
        }
    }
}

const fn side_attr(side: PopoverSide) -> &'static str {
    match side {
        PopoverSide::Top => "top",
        PopoverSide::Bottom => "bottom",
        PopoverSide::Start => "start",
        PopoverSide::End => "end",
    }
}
