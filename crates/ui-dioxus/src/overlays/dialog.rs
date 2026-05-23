//! Modal dialog with focus trap and Escape-to-dismiss.

use dioxus::prelude::*;

/// Tone of a single dialog action, mapped to a button variant class.
///
/// `Primary` is the affirmative call-to-action; `Danger` flags an
/// irreversible destructive action and the consumer should pair it
/// with a confirmation step. `Neutral` (default) is a secondary
/// alternative; `Ghost` is for low-emphasis cancel-style actions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogActionTone {
    #[default]
    Neutral,
    Primary,
    Danger,
    Ghost,
}

impl DialogActionTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Neutral => "ui-button ui-button--secondary",
            Self::Primary => "ui-button ui-button--primary",
            Self::Danger => "ui-button ui-button--danger",
            Self::Ghost => "ui-button ui-button--ghost",
        }
    }
}

/// One action button in a dialog's `actions` slot.
///
/// `id` is the value passed back to `on_action` when the button is
/// clicked, allowing the consumer to disambiguate. `label` is the
/// visible button text. `tone` chooses the button variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogAction {
    pub id: String,
    pub label: String,
    pub tone: DialogActionTone,
}

impl DialogAction {
    /// Convenience constructor for the most common neutral-tone case.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tone: DialogActionTone::Neutral,
        }
    }

    pub fn primary(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { tone: DialogActionTone::Primary, ..Self::new(id, label) }
    }

    pub fn danger(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { tone: DialogActionTone::Danger, ..Self::new(id, label) }
    }

    pub fn ghost(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { tone: DialogActionTone::Ghost, ..Self::new(id, label) }
    }
}

/// Legacy compat: accept a plain `String` label as a Neutral action.
/// Lets existing call sites continue to pass `vec!["Cancel".into(), "OK".into()]`
/// without breaking. New consumers should use `DialogAction::*` constructors
/// for explicit tone selection.
impl From<String> for DialogAction {
    fn from(label: String) -> Self {
        Self {
            id: label.clone(),
            label,
            tone: DialogActionTone::Neutral,
        }
    }
}

impl From<&str> for DialogAction {
    fn from(label: &str) -> Self {
        Self::from(label.to_string())
    }
}

/// Installs a Tab-cycling focus trap on the most recently mounted
/// `.ui-dialog-panel`. The handler is registered on the panel element so when
/// the panel is removed from the DOM, the listener is garbage-collected
/// together with it — no Rust-side teardown needed.
fn install_dialog_focus_trap() {
    const FOCUSABLE_SELECTOR: &str =
        "button:not([disabled]),[href],input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex=\"-1\"])";
    let script = format!(
        r#"
        (function() {{
            const panel = document.querySelector('.ui-dialog-panel');
            if (!panel || panel.__kineticsTrap) return;
            panel.__kineticsTrap = true;
            panel.addEventListener('keydown', (e) => {{
                if (e.key !== 'Tab') return;
                const f = panel.querySelectorAll('{selector}');
                if (f.length === 0) {{ e.preventDefault(); panel.focus(); return; }}
                const first = f[0];
                const last = f[f.length - 1];
                const active = document.activeElement;
                if (e.shiftKey && (active === first || active === panel)) {{
                    e.preventDefault();
                    last.focus();
                }} else if (!e.shiftKey && active === last) {{
                    e.preventDefault();
                    first.focus();
                }}
            }});
        }})();
        "#,
        selector = FOCUSABLE_SELECTOR,
    );
    let _ = dioxus::document::eval(&script);
}

#[component]
pub fn Dialog(
    title: String,
    #[props(default)] open: bool,
    #[props(default)] description: String,
    #[props(default)] body: String,
    #[props(default, into)] actions: Vec<DialogAction>,
    #[props(default = true)] dismissible: bool,
    on_dismiss: Option<EventHandler<()>>,
    on_action: Option<EventHandler<String>>,
) -> Element {
    if !open {
        return rsx! {};
    }

    let has_description = !description.is_empty();
    let described_by = if has_description {
        "ui-dialog-description"
    } else {
        ""
    };

    rsx! {
        div {
            class: "ui-dialog",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": "ui-dialog-title",
            "aria-describedby": "{described_by}",
            onkeydown: move |evt| {
                if dismissible && evt.key() == Key::Escape {
                    evt.stop_propagation();
                    if let Some(handler) = &on_dismiss {
                        handler.call(());
                    }
                }
            },
            div {
                class: "ui-dialog-backdrop",
                onclick: move |_evt| {
                    if dismissible {
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    }
                },
            }
            div {
                class: "ui-dialog-panel",
                tabindex: "-1",
                onmounted: move |evt| {
                    spawn(async move {
                        let _ = evt.set_focus(true).await;
                    });
                    install_dialog_focus_trap();
                },
                h2 { id: "ui-dialog-title", class: "ui-dialog-title", "{title}" }
                if has_description {
                    p { id: "ui-dialog-description", class: "ui-dialog-description", "{description}" }
                }
                if !body.is_empty() {
                    div { class: "ui-dialog-body", "{body}" }
                }
                if !actions.is_empty() {
                    div { class: "ui-dialog-actions",
                        for action in actions {
                            {
                                let action_id = action.id.clone();
                                let action_label = action.label.clone();
                                let action_class = action.tone.class_name();
                                rsx! {
                                    button {
                                        class: "{action_class}",
                                        r#type: "button",
                                        onclick: move |_evt| {
                                            if let Some(handler) = &on_action {
                                                handler.call(action_id.clone());
                                            }
                                        },
                                        "{action_label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
