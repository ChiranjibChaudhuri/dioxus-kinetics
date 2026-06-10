use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Waveform
// ---------------------------------------------------------------------------

/// Geometry of one waveform bar in the `0 0 100 32` viewBox.
#[derive(Clone, Copy, Debug, PartialEq)]
struct WaveBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

/// Clamps a level sample into [0, 1]; NaN / infinite samples render as
/// silence rather than poisoning the layout.
fn normalize_level(level: f32) -> f32 {
    if level.is_finite() {
        level.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// Lays out bar `index` of `count`, vertically centered with a 2-unit
/// minimum height so silent samples still read as a track.
fn wave_bar(index: usize, count: usize, level: f32) -> WaveBar {
    const VIEW_W: f32 = 100.0;
    const VIEW_H: f32 = 32.0;
    const GAP_RATIO: f32 = 0.4;

    let count = count.max(1);
    let slot = VIEW_W / count as f32;
    let width = (slot * (1.0 - GAP_RATIO)).max(0.5);
    let height = (normalize_level(level) * (VIEW_H - 4.0)).max(2.0);
    WaveBar {
        x: slot * index as f32 + (slot - width) / 2.0,
        y: (VIEW_H - height) / 2.0,
        width,
        height,
    }
}

/// An audio level trace rendered as centered bars — the visual idiom of
/// voice recording UIs. Levels are plain props (0.0–1.0 per bar), so the
/// component stays deterministic: hosts stream real microphone levels in
/// `wasm`, captures replay recorded ones, and SSR renders a static trace.
///
/// While `active`, bars pulse with a CSS animation staggered per bar (and
/// stilled under `prefers-reduced-motion`). Decorative when `label` is
/// empty; `role="img"` named by the label otherwise.
#[component]
pub fn Waveform(
    levels: Vec<f32>,
    #[props(default)] label: String,
    #[props(default)] active: bool,
) -> Element {
    let count = levels.len();
    let class = format!(
        "ui-waveform{}",
        if active { " ui-waveform--active" } else { "" }
    );
    let decorative = label.is_empty();

    rsx! {
        span {
            class: "{class}",
            role: if decorative { "presentation" } else { "img" },
            "aria-hidden": if decorative { "true" } else { "false" },
            "aria-label": if decorative { "" } else { "{label}" },
            svg {
                view_box: "0 0 100 32",
                preserve_aspect_ratio: "none",
                "aria-hidden": "true",
                for (index, level) in levels.iter().enumerate() {
                    {
                        let bar = wave_bar(index, count, *level);
                        rsx! {
                            rect {
                                class: "ui-waveform-bar",
                                x: "{bar.x:.2}",
                                y: "{bar.y:.2}",
                                width: "{bar.width:.2}",
                                height: "{bar.height:.2}",
                                rx: "1",
                                style: "animation-delay:{index * 70}ms",
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// VoiceInput
// ---------------------------------------------------------------------------

/// Lifecycle of a [`VoiceInput`] control. The host owns the actual audio
/// pipeline; this component renders the state it is told.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VoiceInputState {
    #[default]
    Idle,
    Recording,
    Processing,
    Error,
}

impl VoiceInputState {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Recording => "recording",
            Self::Processing => "processing",
            Self::Error => "error",
        }
    }

    /// Errors announce assertively; everything else is a polite status.
    pub const fn live_role(self) -> &'static str {
        match self {
            Self::Error => "alert",
            _ => "status",
        }
    }

    pub const fn is_recording(self) -> bool {
        matches!(self, Self::Recording)
    }
}

/// The status line announced for each state; the error state prefers the
/// host-supplied message when one exists.
fn status_text(state: VoiceInputState, error_message: &str) -> String {
    match state {
        VoiceInputState::Idle => "Ready to record".to_string(),
        VoiceInputState::Recording => "Recording…".to_string(),
        VoiceInputState::Processing => "Transcribing…".to_string(),
        VoiceInputState::Error => {
            if error_message.is_empty() {
                "Voice input failed".to_string()
            } else {
                error_message.to_string()
            }
        }
    }
}

/// A push-to-talk voice composer control for AI surfaces: a mic toggle, a
/// live [`Waveform`], an elapsed-time readout, and a state line announced
/// through a live region (`role="status"`, escalating to `role="alert"` for
/// errors).
///
/// Controlled like the rest of the AI family: the host drives `state`,
/// `levels`, and `elapsed` (a preformatted string, so SSR and capture stay
/// deterministic), and reacts to `on_start` / `on_stop`.
#[component]
pub fn VoiceInput(
    state: VoiceInputState,
    #[props(default)] levels: Vec<f32>,
    on_start: EventHandler<()>,
    on_stop: EventHandler<()>,
    #[props(default)] elapsed: String,
    #[props(default)] error_message: String,
    #[props(default = "Start voice input".to_string())] start_label: String,
    #[props(default = "Stop recording".to_string())] stop_label: String,
    #[props(default)] disabled: bool,
) -> Element {
    let recording = state.is_recording();
    let class = format!("ui-voice-input ui-voice-input--{}", state.class_suffix());
    let toggle_label = if recording {
        stop_label.clone()
    } else {
        start_label.clone()
    };
    let status = status_text(state, &error_message);
    let busy = state == VoiceInputState::Processing;

    rsx! {
        div { class: "{class}",
            button {
                class: "ui-voice-input-toggle",
                r#type: "button",
                disabled: disabled || busy,
                "aria-label": "{toggle_label}",
                "aria-pressed": if recording { "true" } else { "false" },
                onclick: move |_| {
                    if recording {
                        on_stop.call(());
                    } else {
                        on_start.call(());
                    }
                },
                if recording {
                    // Stop square.
                    svg {
                        class: "ui-voice-input-icon",
                        view_box: "0 0 16 16",
                        "aria-hidden": "true",
                        rect { x: "3.5", y: "3.5", width: "9", height: "9", rx: "1.5" }
                    }
                } else {
                    // Microphone.
                    svg {
                        class: "ui-voice-input-icon",
                        view_box: "0 0 16 16",
                        "aria-hidden": "true",
                        rect { x: "6", y: "1.5", width: "4", height: "8", rx: "2" }
                        path {
                            d: "M3.5 7.5a4.5 4.5 0 0 0 9 0M8 12v2.5M5.5 14.5h5",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.4",
                            stroke_linecap: "round",
                        }
                    }
                }
            }
            div { class: "ui-voice-input-trace",
                Waveform { levels, active: recording }
            }
            div { class: "ui-voice-input-meta",
                if !elapsed.is_empty() {
                    span { class: "ui-voice-input-elapsed ui-tabular", "{elapsed}" }
                }
                span {
                    class: "ui-voice-input-status",
                    role: "{state.live_role()}",
                    "{status}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_level_clamps_and_silences_garbage() {
        assert_eq!(normalize_level(0.5), 0.5);
        assert_eq!(normalize_level(-1.0), 0.0);
        assert_eq!(normalize_level(7.0), 1.0);
        assert_eq!(normalize_level(f32::NAN), 0.0);
    }

    #[test]
    fn wave_bar_centers_vertically_with_min_height() {
        let silent = wave_bar(0, 10, 0.0);
        assert_eq!(silent.height, 2.0);
        assert_eq!(silent.y, 15.0);

        let full = wave_bar(0, 10, 1.0);
        assert_eq!(full.height, 28.0);
        assert_eq!(full.y, 2.0);
    }

    #[test]
    fn wave_bar_slots_span_the_viewbox() {
        let count = 4;
        let last = wave_bar(count - 1, count, 1.0);
        assert!(last.x + last.width <= 100.0);
        let first = wave_bar(0, count, 1.0);
        assert!(first.x >= 0.0);
    }

    #[test]
    fn voice_state_class_suffixes() {
        assert_eq!(VoiceInputState::Idle.class_suffix(), "idle");
        assert_eq!(VoiceInputState::Recording.class_suffix(), "recording");
        assert_eq!(VoiceInputState::Processing.class_suffix(), "processing");
        assert_eq!(VoiceInputState::Error.class_suffix(), "error");
        assert_eq!(VoiceInputState::default(), VoiceInputState::Idle);
    }

    #[test]
    fn voice_state_error_is_assertive() {
        assert_eq!(VoiceInputState::Error.live_role(), "alert");
        assert_eq!(VoiceInputState::Recording.live_role(), "status");
    }

    #[test]
    fn status_text_prefers_host_error_message() {
        assert_eq!(
            status_text(VoiceInputState::Error, "Mic permission denied"),
            "Mic permission denied"
        );
        assert_eq!(
            status_text(VoiceInputState::Error, ""),
            "Voice input failed"
        );
        assert_eq!(status_text(VoiceInputState::Recording, ""), "Recording…");
    }
}
