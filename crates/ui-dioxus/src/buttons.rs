use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonTone {
    #[default]
    Neutral,
    Primary,
    Danger,
}

impl IconButtonTone {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Primary => "primary",
            Self::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonSize {
    Compact,
    #[default]
    Default,
    Spacious,
}

impl IconButtonSize {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Default => "default",
            Self::Spacious => "spacious",
        }
    }
}

#[component]
pub fn IconButton(
    label: String,
    #[props(default)] tone: IconButtonTone,
    #[props(default)] size: IconButtonSize,
    #[props(default = false)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let tone_class = tone.class_suffix();
    let size_class = size.class_suffix();
    let class = format!("ui-icon-button ui-icon-button--{tone_class} ui-icon-button--{size_class}");

    rsx! {
        button {
            r#type: "button",
            class: "{class}",
            "aria-label": "{label}",
            disabled: disabled,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            span { class: "ui-icon-button-glyph",
                {children}
            }
        }
    }
}
