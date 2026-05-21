use dioxus::prelude::*;
use ui_composition::{
    ClipFill, Composition, FrameClip as CompositionFrameClip, FrameLayer as CompositionFrameLayer,
};

#[component]
pub fn FrameStage(composition: Composition, frame: u32, children: Element) -> Element {
    rsx! {
        section {
            class: "ui-frame-stage",
            "data-composition-id": "{composition.id}",
            "data-width": "{composition.width}",
            "data-height": "{composition.height}",
            "data-fps": "{composition.fps}",
            "data-frame": "{frame}",
            {children}
        }
    }
}

#[component]
pub fn FrameClip(start: u32, duration: u32, children: Element) -> Element {
    let clip = CompositionFrameClip::new(start, duration, ClipFill::None);

    rsx! {
        div {
            class: "ui-frame-clip",
            "data-start": "{clip.start}",
            "data-duration": "{clip.duration}",
            {children}
        }
    }
}

#[component]
pub fn FrameLayer(id: String, depth: i32, children: Element) -> Element {
    let layer = CompositionFrameLayer::new(id, depth);

    rsx! {
        div {
            class: "ui-frame-layer",
            "data-layer-id": "{layer.id}",
            "data-depth": "{layer.depth}",
            {children}
        }
    }
}
