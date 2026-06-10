use dioxus::prelude::*;
use kinetics::prelude::*;

/// Deterministic level trace shared by the voice previews: a synthetic
/// speech envelope, so static snapshots are stable across runs.
fn demo_levels() -> Vec<f32> {
    vec![
        0.18, 0.42, 0.65, 0.5, 0.8, 0.95, 0.7, 0.4, 0.55, 0.85, 0.6, 0.3, 0.45, 0.75, 0.5, 0.25,
        0.35, 0.6, 0.4, 0.2,
    ]
}

// ---------------------------------------------------------------------------
// Waveform
// ---------------------------------------------------------------------------

pub fn waveform_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Static trace · levels as props" }
                Waveform {
                    levels: demo_levels(),
                    label: "Recorded clip levels".to_string(),
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Active · staggered pulse while recording" }
                Waveform { levels: demo_levels(), active: true }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// VoiceInput
// ---------------------------------------------------------------------------

pub fn voice_input_preview() -> Element {
    rsx! { VoiceInputPreviewBody {} }
}

#[component]
fn VoiceInputPreviewBody() -> Element {
    let mut state = use_signal(|| VoiceInputState::Idle);

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Interactive · toggle starts and stops" }
                VoiceInput {
                    state: *state.read(),
                    levels: demo_levels(),
                    elapsed: if state.read().is_recording() { "0:07".to_string() } else { String::new() },
                    on_start: move |_| state.set(VoiceInputState::Recording),
                    on_stop: move |_| state.set(VoiceInputState::Idle),
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Error · assertive announcement" }
                VoiceInput {
                    state: VoiceInputState::Error,
                    error_message: "Microphone permission denied".to_string(),
                    on_start: move |_| {},
                    on_stop: move |_| {},
                }
            }
        }
    }
}
