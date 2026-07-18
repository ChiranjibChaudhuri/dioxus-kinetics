//! Perplexity Comet-style landing page — a dark, teal, "agentic browser"
//! hero with a live browser-window mockup whose agent rail drives an
//! `AgentTimeline`. Composed entirely from kinetics primitives
//! (`LiquidGlass`, `KineticText`, `AgentTimeline`, `AiStatus`, `MetricCounter`,
//! `Button`).
//!
//! Run it:  dx serve --package comet --port 9176
//!
//! Note: the live Perplexity Comet site blocks programmatic fetches, so the
//! palette (warm dark + Perplexity teal #20B8CE) and structure ("the
//! agentic browser" hero + browser mockup) match the known Comet identity
//! rather than a pixel copy.

use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_styles::library_css;

const BG: &str = "#0e0f0c";
const TEAL: &str = "#20b8ce";

#[component]
pub fn App() -> Element {
    let css = library_css();
    rsx! {
        style { "{css}" }
        div {
            "data-ui-theme": "dark",
            "data-ui-density": "comfortable",
            style: "min-height:100vh; background:{BG}; color:#f7f7f5; \
                    font-family:Inter, ui-sans-serif, system-ui, sans-serif;",
            Hero {}
            BrowserMock {}
            FeatureStrip {}
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            style: "padding:88px 32px 48px; display:flex; flex-direction:column; \
                    align-items:center; text-align:center; gap:22px; max-width:1100px; \
                    margin:0 auto;",
            span {
                style: "font-size:14px; letter-spacing:0.2em; text-transform:uppercase; \
                        font-weight:600; color:{TEAL};",
                KineticText { id: "comet-eyebrow", text: "Introducing Comet".to_string(), cue: "fade-in" }
            }
            h1 {
                style: "margin:0; font-size:clamp(48px, 7vw, 92px); line-height:1.02; \
                        letter-spacing:-0.03em; font-weight:700; max-width:14ch;",
                KineticText { id: "comet-title", text: "The agentic browser.".to_string(), cue: "rise-in" }
            }
            p {
                style: "margin:0; font-size:22px; line-height:1.4; color:#b8b8b0; max-width:640px;",
                KineticText {
                    id: "comet-sub",
                    text: "Comet thinks, clicks, and completes across the web \u{2014} so you stop doing the busywork.".to_string(),
                    cue: "fade-in",
                }
            }
            div { style: "display:flex; gap:14px; margin-top:8px;",
                button {
                    style: "border:0; border-radius:999px; padding:14px 28px; font-size:17px; \
                            font-weight:600; background:{TEAL}; color:#04141a; cursor:pointer;",
                    "Get Comet \u{2014} free"
                }
                button {
                    style: "border:1px solid rgba(255,255,255,0.16); border-radius:999px; \
                            padding:14px 28px; font-size:17px; font-weight:600; background:transparent; \
                            color:#f7f7f5; cursor:pointer;",
                    "Watch it work"
                }
            }
        }
    }
}

#[component]
fn BrowserMock() -> Element {
    let steps = vec![
        AgentStep::new("Opened flight aggregator", AgentStepState::Done),
        AgentStep::new("Compared 12 options", AgentStepState::Done),
        AgentStep::new("Filtering for nonstop, under $400", AgentStepState::Active),
        AgentStep::new("Booking the best match", AgentStepState::Pending),
    ];
    rsx! {
        section {
            style: "padding:24px 32px 72px; max-width:1200px; margin:0 auto;",
            LiquidGlass {
                tone: GlassTone::Neutral,
                div {
                    style: "border-radius:22px; overflow:hidden;",
                    // Window chrome: tab strip + address bar
                    div {
                        style: "display:flex; align-items:center; gap:12px; padding:14px 18px; \
                                border-bottom:1px solid rgba(255,255,255,0.08);",
                        div { style: "display:flex; gap:7px;",
                            span { style: "width:12px; height:12px; border-radius:50%; background:#ff5f57;" }
                            span { style: "width:12px; height:12px; border-radius:50%; background:#febc2e;" }
                            span { style: "width:12px; height:12px; border-radius:50%; background:#28c840;" }
                        }
                        div { style: "display:flex; gap:8px;",
                            span { style: "padding:5px 14px; border-radius:9px 9px 0 0; \
                                          background:rgba(255,255,255,0.08); font-size:13px; color:#f7f7f5;",
                                "Find me a flight"
                            }
                            span { style: "padding:5px 14px; font-size:13px; color:#9c9c95;", "Perplexity" }
                        }
                        div {
                            style: "flex:1; margin-left:8px; padding:7px 14px; border-radius:8px; \
                                    background:rgba(255,255,255,0.05); font-size:13px; color:#9c9c95;",
                            "comet \u{00b7} searching the web"
                        }
                    }
                    // Split: agent rail + content skeleton
                    div { style: "display:flex; min-height:340px;",
                        aside {
                            style: "width:280px; padding:22px 20px; border-right:1px solid rgba(255,255,255,0.08); \
                                    display:flex; flex-direction:column; gap:18px;",
                            div { style: "display:flex; align-items:center; gap:10px;",
                                AiStatus { state: AiStatusState::Searching, label: "Working on it\u{2026}".to_string() }
                            }
                            AgentTimeline { steps }
                        }
                        main {
                            style: "flex:1; padding:28px; display:flex; flex-direction:column; gap:14px;",
                            div { style: "height:18px; width:55%; border-radius:5px; background:rgba(255,255,255,0.14);" }
                            div { style: "height:12px; width:90%; border-radius:5px; background:rgba(255,255,255,0.07);" }
                            div { style: "height:12px; width:82%; border-radius:5px; background:rgba(255,255,255,0.07);" }
                            div { style: "height:12px; width:68%; border-radius:5px; background:rgba(255,255,255,0.07);" }
                            div {
                                style: "margin-top:8px; padding:16px; border-radius:12px; \
                                        background:rgba(32,184,206,0.12); border:1px solid rgba(32,184,206,0.35); \
                                        font-size:15px; color:{TEAL};",
                                "Best match: nonstop SFO\u{2192}JFK, $318 \u{2014} want me to book it?"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureStrip() -> Element {
    rsx! {
        section {
            style: "padding:0 32px 96px; max-width:1100px; margin:0 auto; \
                    display:grid; grid-template-columns:repeat(auto-fit,minmax(240px,1fr)); gap:24px;",
            FeatureCard {
                title: "Acts on your behalf".to_string(),
                body: "Clicks, scrolls, fills, and books across sites \u{2014} with you in control.".to_string(),
            }
            FeatureCard {
                title: "Answers with sources".to_string(),
                body: "Every claim cites where it came from. No hallucinated footnotes.".to_string(),
            }
            FeatureCard {
                title: "Learns your intent".to_string(),
                body: "Remembers context across tabs and tasks, so you never re-explain.".to_string(),
            }
        }
    }
}

#[component]
fn FeatureCard(title: String, body: String) -> Element {
    rsx! {
        LiquidGlass {
            tone: GlassTone::Neutral,
            div {
                style: "padding:28px; border-radius:18px; display:flex; flex-direction:column; gap:10px;",
                h3 { style: "margin:0; font-size:20px; font-weight:600; color:#f7f7f5;", "{title}" }
                p { style: "margin:0; font-size:15px; line-height:1.5; color:#b8b8b0;", "{body}" }
            }
        }
    }
}
