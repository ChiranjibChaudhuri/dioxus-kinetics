use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_composition::ClipFill;

use super::cta_pulse::CtaPulseScene;
use super::flip_card_deck::FlipCardDeckScene;
use super::metric_counter::MetricCounterScene;

#[component]
pub fn ProductIntroScene(
    /// Show the Scene's transport (play/pause + scrubber + time readout).
    /// Defaults to `true` so the gallery preview tile keeps its scrub UI;
    /// the flagship hero passes `false` to suppress the chrome.
    #[props(default = true)]
    controls: bool,
) -> Element {
    rsx! {
        Scene {
            id: "product-intro",
            width: 1920,
            height: 1080,
            duration_ms: 10_000.0,
            fps: Some(60),
            autoplay: Some(true),
            controls: Some(controls),

            Clip { start_ms: 0.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "intro-title",
                    text: "Kinetics moves like light.".to_string(),
                    cue: "rise-in",
                    class: "scene-hero-title".to_string(),
                }
            }
            Clip { start_ms: 800.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "intro-body",
                    text: "Composable motion for downstream SaaS.".to_string(),
                    cue: "fade-in",
                    class: "scene-hero-subtitle".to_string(),
                }
            }
            Clip { start_ms: 3_000.0, duration_ms: 4_000.0,
                FlipCardDeckScene {}
            }
            Clip { start_ms: 4_800.0, duration_ms: 2_200.0,
                MetricCounterScene {}
            }
            Clip { start_ms: 6_800.0, duration_ms: 3_200.0, fill: ClipFill::HoldEnd,
                CtaPulseScene {}
            }
        }
    }
}
