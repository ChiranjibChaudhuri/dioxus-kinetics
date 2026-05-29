//! Non-modal notification toast with tonal variants.
//!
//! Two surfaces ship here. [`Toast`] is the single inline card consumers
//! place wherever they like. [`Toaster`] is the fixed-position stack: it
//! owns a list of [`ToastEntry`] values, renders each in the Wave-1
//! `.ui-toast-region`, and auto-dismisses each after `duration_ms`
//! (pausing the countdown while the pointer is over a toast).

use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

impl ToastTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Neutral => "ui-toast ui-toast--neutral",
            Self::Success => "ui-toast ui-toast--success",
            Self::Warning => "ui-toast ui-toast--warning",
            Self::Danger => "ui-toast ui-toast--danger",
            Self::Info => "ui-toast ui-toast--info",
        }
    }

    pub const fn role(self) -> &'static str {
        match self {
            Self::Danger | Self::Warning => "alert",
            _ => "status",
        }
    }
}

#[component]
pub fn Toast(
    title: String,
    #[props(default)] tone: ToastTone,
    #[props(default)] description: String,
    #[props(default)] action_label: String,
    #[props(default)] dismiss_label: String,
    on_action: Option<EventHandler<()>>,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div { class: "{tone.class_name()}", role: "{tone.role()}",
            div { class: "ui-toast-content",
                strong { class: "ui-toast-title", "{title}" }
                if !description.is_empty() {
                    p { class: "ui-toast-description", "{description}" }
                }
            }
            if !action_label.is_empty() || !dismiss_label.is_empty() {
                div { class: "ui-toast-actions",
                    if !action_label.is_empty() {
                        button {
                            class: "ui-button ui-button--secondary",
                            r#type: "button",
                            onclick: move |_evt| {
                                if let Some(handler) = &on_action {
                                    handler.call(());
                                }
                            },
                            "{action_label}"
                        }
                    }
                    if !dismiss_label.is_empty() {
                        button {
                            class: "ui-button ui-button--ghost",
                            r#type: "button",
                            onclick: move |_evt| {
                                if let Some(handler) = &on_dismiss {
                                    handler.call(());
                                }
                            },
                            "{dismiss_label}"
                        }
                    }
                }
            }
        }
    }
}

/// Default time, in milliseconds, a toast remains before auto-dismissing.
const DEFAULT_TOAST_DURATION_MS: u32 = 5000;

/// One toast in a [`Toaster`] stack.
///
/// `id` is the stable key passed back to `on_dismiss` so the host can drop
/// the matching entry from its own list. `tone` selects the visual variant
/// and ARIA role; `title` is the headline and `description` is optional
/// supporting copy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastEntry {
    pub id: String,
    pub tone: ToastTone,
    pub title: String,
    pub description: String,
}

impl ToastEntry {
    /// Constructs a neutral-tone entry with no description.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tone: ToastTone::Neutral,
            title: title.into(),
            description: String::new(),
        }
    }

    pub fn with_tone(mut self, tone: ToastTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

/// Fixed-position toast stack.
///
/// Renders `items` into the Wave-1 `.ui-toast-region`, each animating in
/// via the `ui-toast-in` keyframe. Every entry starts a `duration_ms`
/// auto-dismiss countdown that fires `on_dismiss(id)`; hovering a toast
/// pauses its countdown (so a user reading it is never interrupted) and
/// the timer re-arms on pointer-leave.
#[component]
pub fn Toaster(
    #[props(default)] items: Vec<ToastEntry>,
    /// Auto-dismiss delay per entry. Defaults to 5000ms.
    duration_ms: Option<u32>,
    on_dismiss: Option<EventHandler<String>>,
) -> Element {
    let duration = duration_ms.unwrap_or(DEFAULT_TOAST_DURATION_MS);

    // Ids whose countdown is currently paused because the pointer is over
    // the toast. Shared across the region so a timer can re-check on wake.
    let hovered = use_signal(HashSet::<String>::new);

    // Per-id generation token. Captured at mount; bumped on every
    // (re)mount. The auto-dismiss task only fires `on_dismiss` while its
    // captured generation still matches the live value, so a task that
    // outlives its element (id removed or the key reused for a new entry)
    // can never dismiss the wrong toast.
    let generations = use_signal(HashMap::<String, u64>::new);

    rsx! {
        div { class: "ui-toast-region",
            for entry in items {
                {
                    let id_for_key = entry.id.clone();
                    let id_for_timer = entry.id.clone();
                    let id_for_enter = entry.id.clone();
                    let id_for_leave = entry.id.clone();
                    let tone = entry.tone;
                    let title = entry.title.clone();
                    let description = entry.description.clone();
                    rsx! {
                        div {
                            key: "{id_for_key}",
                            class: "{tone.class_name()}",
                            role: "{tone.role()}",
                            "data-state": "open",
                            onmounted: move |_evt| {
                                let id = id_for_timer.clone();
                                // Bump and capture this mount's generation
                                // so a stale task from a prior mount of the
                                // same id stands down.
                                let mut generations = generations;
                                let my_gen = {
                                    let mut map = generations.write();
                                    let next = map.get(&id).copied().unwrap_or(0) + 1;
                                    map.insert(id.clone(), next);
                                    next
                                };
                                spawn(async move {
                                    // Accumulate only un-hovered elapsed
                                    // time in short ticks: a hovered toast
                                    // never advances its countdown, which
                                    // pauses (and effectively re-arms) it
                                    // for free. Dismiss once the un-hovered
                                    // total reaches the full delay.
                                    let mut elapsed: u32 = 0;
                                    while elapsed < duration {
                                        let step = TOAST_TICK_MS.min(duration - elapsed);
                                        let mut eval = dioxus::document::eval(
                                            &toast_delay_script(step),
                                        );
                                        let _ = eval.recv::<bool>().await;
                                        // The element (or a re-mount of this
                                        // id) superseded us — stand down
                                        // without dismissing.
                                        if generations.read().get(&id).copied() != Some(my_gen) {
                                            return;
                                        }
                                        if !hovered.read().contains(&id) {
                                            elapsed += step;
                                        }
                                    }
                                    if generations.read().get(&id).copied() == Some(my_gen) {
                                        if let Some(handler) = &on_dismiss {
                                            handler.call(id.clone());
                                        }
                                    }
                                });
                            },
                            onmouseenter: move |_evt| {
                                let mut hovered = hovered;
                                hovered.write().insert(id_for_enter.clone());
                            },
                            onmouseleave: move |_evt| {
                                let mut hovered = hovered;
                                hovered.write().remove(&id_for_leave);
                            },
                            div { class: "ui-toast-content",
                                strong { class: "ui-toast-title", "{title}" }
                                if !description.is_empty() {
                                    p { class: "ui-toast-description", "{description}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Length of one auto-dismiss tick, in milliseconds. The per-entry task
/// wakes this often to accumulate un-hovered elapsed time, giving hover
/// pause/resume a sub-`TOAST_TICK_MS` granularity without a JS-side timer
/// the Rust task can no longer cancel.
const TOAST_TICK_MS: u32 = 100;

/// Builds the per-entry auto-dismiss `setTimeout` script. Mirrors the
/// popover close-delay round-trip: JS waits `ms` then notifies Rust.
fn toast_delay_script(ms: u32) -> String {
    format!(
        r#"
        setTimeout(() => {{ dioxus.send(true); }}, {ms});
        "#,
        ms = ms,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_role_maps_danger_and_warning_to_alert() {
        assert_eq!(ToastTone::Danger.role(), "alert");
        assert_eq!(ToastTone::Warning.role(), "alert");
        assert_eq!(ToastTone::Success.role(), "status");
        assert_eq!(ToastTone::Neutral.role(), "status");
    }

    #[test]
    fn toast_entry_builders_set_tone_and_description() {
        let entry = ToastEntry::new("saved", "Saved")
            .with_tone(ToastTone::Success)
            .with_description("Your changes are live.");
        assert_eq!(entry.id, "saved");
        assert_eq!(entry.tone, ToastTone::Success);
        assert_eq!(entry.description, "Your changes are live.");
    }

    #[test]
    fn toast_entry_defaults_to_neutral_no_description() {
        let entry = ToastEntry::new("x", "X");
        assert_eq!(entry.tone, ToastTone::Neutral);
        assert!(entry.description.is_empty());
    }

    #[test]
    fn delay_script_embeds_duration() {
        let script = toast_delay_script(DEFAULT_TOAST_DURATION_MS);
        assert!(script.contains("setTimeout"));
        assert!(script.contains(&format!(", {DEFAULT_TOAST_DURATION_MS})")));
    }

    #[test]
    fn tick_is_a_positive_fraction_of_the_default_duration() {
        // The auto-dismiss task advances `elapsed` in `TOAST_TICK_MS`
        // chunks (clamped to the remaining time), so the tick must be
        // non-zero and no larger than a default-duration countdown.
        assert!(TOAST_TICK_MS > 0);
        assert!(TOAST_TICK_MS <= DEFAULT_TOAST_DURATION_MS);
    }
}
