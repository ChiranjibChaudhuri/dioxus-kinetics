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
    /// Whether the scene starts playing automatically. Defaults to
    /// `true` for the gallery preview; the flagship hero passes
    /// `false` so the scene renders as a static still at
    /// `initial_elapsed_ms`.
    #[props(default = true)]
    autoplay: bool,
    /// Seek the scene clock to this elapsed time once at mount.
    /// Combined with `autoplay: false`, the scene renders the curated
    /// frame indefinitely — used by the flagship hero to freeze at
    /// the title + body cinematic peak.
    initial_elapsed_ms: Option<f32>,
) -> Element {
    rsx! {
        Scene {
            id: "product-intro",
            width: 1920,
            height: 1080,
            duration_ms: 10_000.0,
            fps: Some(60),
            autoplay: Some(autoplay),
            controls: Some(controls),
            initial_elapsed_ms: initial_elapsed_ms,

            // Title and body intentionally do NOT use HoldEnd — once the
            // hero film plays through, the settled-state composition is
            // just the CTA pulse (`Start building`) overlaid on the
            // ambient backdrop. Holding the title and body too would
            // stack three centred elements in the same grid cell when
            // the scene is hosted full-bleed (e.g., the flagship hero).
            Clip { start_ms: 0.0, duration_ms: 2_400.0,
                KineticText {
                    id: "intro-title",
                    text: "Kinetics moves like light.".to_string(),
                    cue: "rise-in",
                    class: "scene-hero-title".to_string(),
                }
            }
            Clip { start_ms: 800.0, duration_ms: 2_400.0,
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
