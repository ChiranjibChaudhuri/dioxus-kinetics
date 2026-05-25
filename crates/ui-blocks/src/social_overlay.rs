use dioxus::prelude::*;

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
            div { class: "ui-block-social-overlay__handle", "{handle}" }
            div { class: "ui-block-social-overlay__message", "{message}" }
        }
    }
}
