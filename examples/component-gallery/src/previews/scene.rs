use dioxus::prelude::*;

use crate::previews::scenes::curved_trajectory::CurvedTrajectoryScene;
use crate::previews::scenes::product_intro::ProductIntroScene;
use crate::previews::scenes::scroll_story::ScrollPinnedStoryScene;
use crate::previews::scenes::split_headline::SplitHeadlineScene;

pub fn product_intro_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            ProductIntroScene {}
        }
    }
}

pub fn scroll_pinned_story_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            ScrollPinnedStoryScene {}
        }
    }
}

pub fn split_headline_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            SplitHeadlineScene {}
        }
    }
}

pub fn curved_trajectory_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            CurvedTrajectoryScene {}
        }
    }
}
