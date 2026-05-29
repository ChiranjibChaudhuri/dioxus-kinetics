//! Side sheet (drawer) overlay with focus trap and Escape-to-dismiss.
//!
//! A `Sheet` slides in from the inline-start or inline-end edge and
//! behaves as a modal surface: it renders a dismissible backdrop, traps
//! Tab focus inside the panel, and pulls focus to itself on mount. Like
//! `Dialog`, it returns an empty tree when `open` is `false`, so the
//! consumer controls visibility purely by toggling the prop.

use dioxus::prelude::*;

use super::focus_trap;

/// Which inline edge the sheet docks to. `End` (the default) slides in
/// from the inline-end edge (right in LTR); `Start` from inline-start.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    Start,
    #[default]
    End,
}

impl SheetSide {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// Inline close (X) glyph path, kept self-contained so the sheet does
/// not depend on the icon crate or sibling components.
const CLOSE_PATH_D: &str = "M6 6l12 12M18 6L6 18";

/// A modal side sheet / drawer.
///
/// `open` toggles visibility (returns an empty tree when `false`).
/// `side` chooses the docking edge. `title` labels the dialog and is
/// shown in the header. When `dismissible` (default `true`), the
/// backdrop click, the Escape key, and the close button all fire
/// `on_dismiss`.
#[component]
pub fn Sheet(
    #[props(default)] open: bool,
    #[props(default)] side: SheetSide,
    title: String,
    /// Stable element id for the panel `<aside>`. The focus trap and
    /// opener-restoration helpers key off this id so concurrently mounted
    /// sheets never collide. Defaults to `"ui-sheet"` so single-sheet call
    /// sites keep their old id.
    #[props(default = "ui-sheet".to_string())]
    id: String,
    #[props(default = true)] dismissible: bool,
    on_dismiss: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    let panel_class = format!("ui-sheet ui-sheet--{}", side.class_suffix());

    // Restore focus to the opener on every built-in dismissal path —
    // Escape, backdrop click, and the close button. Each handler owns its
    // own clone of the panel id.
    let id_for_key = id.clone();
    let id_for_backdrop = id.clone();
    let id_for_close = id.clone();
    let id_for_mount = id.clone();

    rsx! {
        div { class: "ui-sheet-root",
            div {
                class: "ui-sheet-backdrop",
                "data-state": "open",
                onclick: move |_evt| {
                    if dismissible {
                        focus_trap::restore_opener(&id_for_backdrop);
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    }
                },
            }
            aside {
                id: "{id}",
                class: "{panel_class}",
                "data-state": "open",
                role: "dialog",
                "aria-modal": "true",
                "aria-label": "{title}",
                tabindex: "-1",
                onkeydown: move |evt| {
                    if dismissible && evt.key() == Key::Escape {
                        evt.stop_propagation();
                        focus_trap::restore_opener(&id_for_key);
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    }
                },
                onmounted: move |evt| {
                    let panel_id = id_for_mount.clone();
                    focus_trap::capture_opener(&panel_id);
                    spawn(async move {
                        let _ = evt.set_focus(true).await;
                    });
                    focus_trap::install_trap(&panel_id);
                },
                div { class: "ui-sheet-header",
                    h2 { class: "ui-sheet-title", "{title}" }
                    if dismissible {
                        button {
                            r#type: "button",
                            class: "ui-sheet-close",
                            "aria-label": "Close",
                            onclick: move |_evt| {
                                focus_trap::restore_opener(&id_for_close);
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
                                path { d: "{CLOSE_PATH_D}" }
                            }
                        }
                    }
                }
                div { class: "ui-sheet-body",
                    {children}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheet_side_maps_to_suffix() {
        assert_eq!(SheetSide::Start.class_suffix(), "start");
        assert_eq!(SheetSide::End.class_suffix(), "end");
    }

    #[test]
    fn sheet_side_default_is_end() {
        assert_eq!(SheetSide::default(), SheetSide::End);
    }
}
