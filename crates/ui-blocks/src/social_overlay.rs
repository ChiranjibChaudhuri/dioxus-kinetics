use dioxus::prelude::*;
use ui_dioxus::{KineticText, TimelineScope};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SocialPlatform {
    Instagram,
    Twitter,
    YouTube,
    TikTok,
}

impl SocialPlatform {
    fn modifier(self) -> &'static str {
        match self {
            Self::Instagram => "instagram",
            Self::Twitter => "twitter",
            Self::YouTube => "youtube",
            Self::TikTok => "tiktok",
        }
    }
}

#[component]
pub fn SocialOverlay(platform: SocialPlatform, handle: String, message: String) -> Element {
    let modifier_class = format!("ui-block-social-overlay--{}", platform.modifier());
    rsx! {
        div {
            class: "ui-block-social-overlay {modifier_class}",
            "data-block": "social-overlay",
            "data-platform": "{platform.modifier()}",
            TimelineScope {
                id: "social-overlay-stagger".to_string(),
                autoplay: false,
                stagger_step_ms: 150.0,
                div { class: "ui-block-social-overlay__handle",
                    KineticText {
                        id: "social-overlay-handle".to_string(),
                        text: handle.clone(),
                        cue: "slide-up".to_string(),
                    }
                }
                div { class: "ui-block-social-overlay__message",
                    KineticText {
                        id: "social-overlay-message".to_string(),
                        text: message.clone(),
                        cue: "fade-in".to_string(),
                    }
                }
            }
        }
    }
}
