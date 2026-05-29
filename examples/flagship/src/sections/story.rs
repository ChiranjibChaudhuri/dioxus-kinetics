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
            // controls:false suppresses the Scene transport — the marketing
            // narrative should read as a pinned story, not a debug scrubber.
            // Belt-and-suspenders with the `.flagship-story .ui-scene-transport`
            // display:none rule in styles.rs.
            ScrollPinnedStoryScene { controls: false }
        }
    }
}
