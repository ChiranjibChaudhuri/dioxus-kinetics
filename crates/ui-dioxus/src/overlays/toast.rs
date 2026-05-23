//! Non-modal notification toast with tonal variants.

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
