use dioxus::prelude::*;
use kinetics::prelude::*;

use crate::demo_frame::{ReplayFrame, ScrubElapsedMs, ScrubFrame};

pub fn presence_preview() -> Element {
    let spring_enter = Transition::Spring(Spring::snappy());
    let spring_exit = Transition::Spring(Spring::snappy());

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            div { class: "gallery-variant-tile",
                ReplayFrame {
                    label: "Tween enter",
                    children: rsx! {
                        Presence { present: true, cue: PresenceCue::Rise,
                            p { "Visible state" }
                        }
                    },
                }
            }
            div { class: "gallery-variant-tile",
                ReplayFrame {
                    label: "Tween exit",
                    children: rsx! {
                        Presence { present: false, cue: PresenceCue::Rise,
                            p { "Hidden state" }
                        }
                    },
                }
            }
            div { class: "gallery-variant-tile",
                ReplayFrame {
                    label: "Spring enter",
                    children: rsx! {
                        Presence {
                            present: true,
                            enter: spring_enter,
                            exit: spring_exit,
                            cue: PresenceCue::Scale,
                            p { "Spring-driven entry" }
                        }
                    },
                }
            }
            div { class: "gallery-variant-tile",
                ReplayFrame {
                    label: "Spring exit",
                    children: rsx! {
                        Presence {
                            present: false,
                            enter: spring_enter,
                            exit: spring_exit,
                            cue: PresenceCue::Scale,
                            p { "Spring-driven exit" }
                        }
                    },
                }
            }
        }
    }
}

pub fn sequence_preview() -> Element {
    rsx! {
        ScrubFrame {
            duration_ms: 560.0,
            fps: None,
            label: "Sequence",
            children: rsx! { SequenceBody {} },
        }
    }
}

#[component]
fn SequenceBody() -> Element {
    let elapsed = use_context::<ScrubElapsedMs>().0;
    let elapsed_ms = *elapsed.read();
    let tween_short = Transition::Tween {
        duration_ms: 220,
        ease: Ease::Standard,
    };
    let tween_med = Transition::Tween {
        duration_ms: 200,
        ease: Ease::Standard,
    };
    let tween_long = Transition::Tween {
        duration_ms: 240,
        ease: Ease::Standard,
    };
    let cues = vec![
        Cue::new(
            "title",
            0.0,
            MotionCue::Opacity {
                from: 0.0,
                to: 1.0,
                transition: tween_short,
            },
        ),
        Cue::new(
            "body",
            120.0,
            MotionCue::Translate {
                axis: Axis::Y,
                from: 12.0,
                to: 0.0,
                transition: tween_med,
            },
        ),
        Cue::new(
            "cta",
            320.0,
            MotionCue::Scale {
                from: 0.94,
                to: 1.0,
                transition: tween_long,
            },
        ),
    ];
    rsx! {
        Sequence {
            cues: Some(cues),
            clock: TimelineClock::Manual { elapsed_ms },
            KineticBox { id: "title", h4 { "Welcome" } }
            KineticBox { id: "body", p { "Subtle entry choreography" } }
            KineticBox { id: "cta", Button { "Get started" } }
        }
    }
}

pub fn timeline_scope_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                ScrubFrame {
                    duration_ms: 1200.0,
                    fps: None,
                    label: "Stagger",
                    children: rsx! {
                        TimelineScope { id: "stagger-demo", autoplay: false,
                            for index in 0u32..4 {
                                div { "data-stagger-index": "{index}",
                                    KineticBox { id: "stagger-{index}", cue: "rise-in",
                                        "Tile {index}"
                                    }
                                }
                            }
                        }
                    },
                }
            }
            div { class: "gallery-variant-tile",
                ScrubFrame {
                    duration_ms: 1000.0,
                    fps: None,
                    label: "Sequence",
                    children: rsx! {
                        TimelineScope { id: "sequence-demo", autoplay: false,
                            KineticBox { id: "sequence-enter", cue: "enter", "Enter" }
                            KineticBox { id: "sequence-settle", cue: "settle", "Settle" }
                            KineticBox { id: "sequence-pulse", cue: "pulse", "Pulse" }
                        }
                    },
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Reduced motion" }
                div { "data-ui-transparency": "reduced",
                    TimelineScope { id: "reduced-demo", autoplay: true,
                        for index in 0u32..4 {
                            div { "data-stagger-index": "{index}",
                                KineticBox { id: "reduced-{index}", cue: "rise-in",
                                    "Tile {index}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn kinetic_box_preview() -> Element {
    let cues = ["rise-in", "fade-in", "slide-up"];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3col",
            for cue in cues {
                div { class: "gallery-variant-tile",
                    ReplayFrame {
                        label: cue,
                        children: rsx! {
                            KineticBox { id: "cue-{cue}", cue: cue.to_string(),
                                p { "Cue preview" }
                            }
                        },
                    }
                }
            }
        }
    }
}

pub fn presence_gate_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            div { class: "gallery-variant-tile",
                ReplayFrame {
                    label: "Present",
                    children: rsx! {
                        PresenceGate { present: true,
                            p { "Visible state" }
                        }
                    },
                }
            }
            div { class: "gallery-variant-tile",
                ReplayFrame {
                    label: "Hidden",
                    children: rsx! {
                        PresenceGate { present: false,
                            p { "Visible state" }
                        }
                        span { class: "gallery-variant-label", "(gate suppresses children)" }
                    },
                }
            }
        }
    }
}
