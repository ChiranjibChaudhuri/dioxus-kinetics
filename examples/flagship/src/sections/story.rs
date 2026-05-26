use component_gallery::previews::scenes::scroll_story::ScrollPinnedStoryScene;
use dioxus::prelude::*;

#[component]
pub fn Story() -> Element {
    rsx! {
        section { class: "flagship-story", aria_labelledby: "flagship-story-heading",
            // ScrollPinnedStoryScene renders its narrative copy via
            // KineticText (visible-but-unsemantic spans). Add a real h2
            // so the document outline has a Story landmark between the
            // Hero's h1 and the Features section's h2.
            h2 {
                id: "flagship-story-heading",
                class: "flagship-sr-only",
                "Scroll-driven storytelling"
            }
            ScrollPinnedStoryScene {}
        }
    }
}
