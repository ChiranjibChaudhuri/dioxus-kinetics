//! Modal dialog with focus trap and Escape-to-dismiss.

use dioxus::prelude::*;

use super::focus_trap;

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
        Self {
            tone: DialogActionTone::Primary,
            ..Self::new(id, label)
        }
    }

    pub fn danger(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            tone: DialogActionTone::Danger,
            ..Self::new(id, label)
        }
    }

    pub fn ghost(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            tone: DialogActionTone::Ghost,
            ..Self::new(id, label)
        }
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

/// Derives the element id for one part (`title`, `description`, `panel`)
/// of a dialog from its `id` base, so concurrent dialogs get distinct ids.
fn dialog_part_id(id: &str, part: &str) -> String {
    format!("{id}-{part}")
}

#[component]
pub fn Dialog(
    title: String,
    /// Stable id base for this dialog. The title, description, and panel
    /// elements derive `{id}-title`, `{id}-description`, and `{id}-panel`
    /// ids from it so concurrently mounted dialogs never collide on the
    /// global `.ui-dialog-panel`/`ui-dialog-title` ids. Defaults to
    /// `"ui-dialog"` so single-dialog call sites keep their old ids.
    #[props(default = "ui-dialog".to_string())]
    id: String,
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
    let title_id = dialog_part_id(&id, "title");
    let description_id = dialog_part_id(&id, "description");
    let panel_id = dialog_part_id(&id, "panel");
    let described_by = if has_description {
        description_id.clone()
    } else {
        String::new()
    };

    // Restore focus to the opener (FocusPolicy::RestoreOnClose) on every
    // built-in dismissal path — Escape and backdrop click. Each handler
    // owns its own clone of the panel id.
    let panel_for_key = panel_id.clone();
    let panel_for_backdrop = panel_id.clone();
    let panel_for_mount = panel_id.clone();

    rsx! {
        div {
            class: "ui-dialog",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": "{title_id}",
            "aria-describedby": "{described_by}",
            onkeydown: move |evt| {
                if dismissible && evt.key() == Key::Escape {
                    evt.stop_propagation();
                    focus_trap::restore_opener(&panel_for_key);
                    if let Some(handler) = &on_dismiss {
                        handler.call(());
                    }
                }
            },
            div {
                class: "ui-dialog-backdrop",
                onclick: move |_evt| {
                    if dismissible {
                        focus_trap::restore_opener(&panel_for_backdrop);
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    }
                },
            }
            div {
                id: "{panel_id}",
                class: "ui-dialog-panel",
                "data-state": "open",
                tabindex: "-1",
                onmounted: move |evt| {
                    let panel_id = panel_for_mount.clone();
                    focus_trap::capture_opener(&panel_id);
                    spawn(async move {
                        let _ = evt.set_focus(true).await;
                    });
                    focus_trap::install_trap(&panel_id);
                },
                h2 { id: "{title_id}", class: "ui-dialog-title", "{title}" }
                if has_description {
                    p { id: "{description_id}", class: "ui-dialog-description", "{description}" }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_part_ids_default_to_legacy_names() {
        assert_eq!(dialog_part_id("ui-dialog", "title"), "ui-dialog-title");
        assert_eq!(
            dialog_part_id("ui-dialog", "description"),
            "ui-dialog-description"
        );
        assert_eq!(dialog_part_id("ui-dialog", "panel"), "ui-dialog-panel");
    }

    #[test]
    fn concurrent_dialogs_get_distinct_panel_ids() {
        assert_ne!(
            dialog_part_id("confirm", "panel"),
            dialog_part_id("settings", "panel")
        );
    }

    #[test]
    fn string_label_maps_to_neutral_action() {
        let action: DialogAction = "Cancel".into();
        assert_eq!(action.id, "Cancel");
        assert_eq!(action.label, "Cancel");
        assert_eq!(action.tone, DialogActionTone::Neutral);
    }

    #[test]
    fn tone_class_names_are_stable() {
        assert_eq!(
            DialogActionTone::Primary.class_name(),
            "ui-button ui-button--primary"
        );
        assert_eq!(
            DialogActionTone::Danger.class_name(),
            "ui-button ui-button--danger"
        );
    }
}
