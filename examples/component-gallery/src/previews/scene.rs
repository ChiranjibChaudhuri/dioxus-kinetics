use dioxus::prelude::*;

use crate::previews::scenes::caption_demo::CaptionDemoScene;
use crate::previews::scenes::curved_trajectory::CurvedTrajectoryScene;
use crate::previews::scenes::lower_third_demo::LowerThirdDemoScene;
use crate::previews::scenes::metric_counter_demo::MetricCounterDemoScene;
use crate::previews::scenes::product_intro::ProductIntroScene;
use crate::previews::scenes::scroll_story::ScrollPinnedStoryScene;
use crate::previews::scenes::split_headline::SplitHeadlineScene;
use crate::previews::scenes::wipe_demo::WipeDemoScene;

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

pub fn lower_third_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            LowerThirdDemoScene {}
        }
    }
}

pub fn caption_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            CaptionDemoScene {}
        }
    }
}

pub fn wipe_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            WipeDemoScene {}
        }
    }
}

pub fn metric_counter_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            MetricCounterDemoScene {}
        }
    }
}
