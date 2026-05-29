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
use ui_runtime::reduced_motion::use_reduced_motion;

/// How long the panel stays mounted with `data-state="closed"` after
/// `open` flips false, so the Wave-1 exit animation can play before the
/// node is removed. Kept in lockstep with the `ui-overlay-in` timing
/// (200ms enter); the exit reuses the same easing window.
const CLOSE_ANIMATION_MS: u32 = 160;

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
    let reduced = use_reduced_motion();

    // The panel stays mounted while `open` is true and for a short
    // closing phase afterwards so the Wave-1 exit animation can play. We
    // track that phase locally: `mounted` is the source of truth for
    // whether the panel node exists in the DOM.
    let mut mounted = use_signal(|| open);

    // `open`/`reduced` are plain props, not signals, so the effect is wrapped
    // in `use_reactive` to re-run whenever either value changes between
    // renders (a bare `use_effect` only re-runs on signal reads).
    use_effect(use_reactive((&open, &reduced), move |(open, reduced)| {
        if open {
            // Opening (or re-opening before a pending close fires):
            // ensure the panel is mounted immediately.
            mounted.set(true);
        } else if *mounted.peek() {
            // Closing: keep the panel mounted with data-state="closed" so
            // the exit animation runs, then unmount. Reduced-motion users
            // skip the delay and unmount immediately.
            if reduced {
                mounted.set(false);
            } else {
                spawn(async move {
                    let mut eval = dioxus::document::eval(&close_delay_script(CLOSE_ANIMATION_MS));
                    let _ = eval.recv::<bool>().await;
                    // Only unmount if we are still meant to be closed; a
                    // re-open during the delay flips `open` back to true.
                    if !open {
                        mounted.set(false);
                    }
                });
            }
        }
    }));

    let render_panel = open || mounted();
    let panel_state = if open { "open" } else { "closed" };

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
            if render_panel {
                div {
                    id: "{id}",
                    class: "{panel_class}",
                    role: "dialog",
                    "data-side": "{side_attr(side)}",
                    "data-state": "{panel_state}",
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

/// Builds a script that resolves back to Rust after `ms` milliseconds via
/// `setTimeout`, used to drive the popover's closing-phase unmount. On
/// non-web targets the eval is a no-op and the awaiting task simply never
/// resolves, which is harmless (the panel is already detached at render).
fn close_delay_script(ms: u32) -> String {
    format!(
        r#"
        setTimeout(() => {{ dioxus.send(true); }}, {ms});
        "#,
        ms = ms,
    )
}

const fn side_attr(side: PopoverSide) -> &'static str {
    match side {
        PopoverSide::Top => "top",
        PopoverSide::Bottom => "bottom",
        PopoverSide::Start => "start",
        PopoverSide::End => "end",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side_attr_maps_every_variant() {
        assert_eq!(side_attr(PopoverSide::Top), "top");
        assert_eq!(side_attr(PopoverSide::Bottom), "bottom");
        assert_eq!(side_attr(PopoverSide::Start), "start");
        assert_eq!(side_attr(PopoverSide::End), "end");
    }

    #[test]
    fn default_side_is_bottom() {
        assert_eq!(PopoverSide::default(), PopoverSide::Bottom);
    }

    #[test]
    fn close_delay_script_uses_the_configured_duration() {
        let script = close_delay_script(CLOSE_ANIMATION_MS);
        assert!(script.contains("setTimeout"));
        assert!(script.contains("dioxus.send(true)"));
        assert!(script.contains(&format!(", {CLOSE_ANIMATION_MS})")));
    }
}
