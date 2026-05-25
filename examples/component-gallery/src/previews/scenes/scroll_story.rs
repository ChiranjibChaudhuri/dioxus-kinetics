use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_composition::ClipFill;

#[component]
pub fn ScrollPinnedStoryScene() -> Element {
    let driver = SceneDriver::Scroll(ScrollObserverConfig::new("#scroll-story-trigger"));
    rsx! {
        div { class: "scene-scroll-shell",
            div {
                id: "scroll-story-trigger",
                class: "scene-scroll-trigger",
                style: "height: 200vh; position: relative;",
                div { class: "scene-scroll-sticky",
                    style: "position: sticky; top: 0; height: 100vh;",
                    Scene {
                        id: "scroll-story",
                        width: 1280,
                        height: 720,
                        duration_ms: 10_000.0,
                        driver: Some(driver),
                        controls: Some(true),

                        Clip { start_ms: 0.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                            KineticText {
                                id: "scroll-headline",
                                text: "Scroll-driven storytelling.".to_string(),
                                cue: "rise-in",
                            }
                        }
                        Clip { start_ms: 2_500.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                            KineticText {
                                id: "scroll-body",
                                text: "Same Scene API. Scroll instead of autoplay.".to_string(),
                                cue: "fade-in",
                            }
                        }
                        Clip { start_ms: 5_000.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                            KineticText {
                                id: "scroll-feature",
                                text: "Built on IntersectionObserver + window scroll.".to_string(),
                                cue: "slide-up",
                            }
                        }
                        Clip { start_ms: 7_500.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                            KineticText {
                                id: "scroll-cta",
                                text: "Pin a story to the page.".to_string(),
                                cue: "rise-in",
                            }
                        }
                    }
                }
            }
        }
    }
}
