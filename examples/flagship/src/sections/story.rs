use dioxus::prelude::*;
use component_gallery::previews::scenes::scroll_story::ScrollPinnedStoryScene;

#[component]
pub fn Story() -> Element {
    rsx! {
        section { class: "flagship-story", aria_label: "Scroll-driven product story",
            ScrollPinnedStoryScene {}
        }
    }
}
