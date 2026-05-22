use std::rc::Rc;

use dioxus::prelude::*;
use ui_runtime::{
    use_element_computed_style, use_element_rect, use_shared_element_registry, ElementSnapshot,
    SharedElementRegistry, SharedTransition,
};

const TRACKED_PROPERTIES: &[&str] = &["border-radius", "background-color", "color", "opacity"];

#[component]
pub fn SharedLayout(children: Element) -> Element {
    use_context_provider(|| Signal::new(Rc::new(SharedElementRegistry::default())));

    rsx! {
        div {
            class: "ui-shared-layout",
            {children}
        }
    }
}

#[component]
pub fn SharedElement(
    id: String,
    #[props(default)] transition: SharedTransition,
    children: Element,
) -> Element {
    let (rect_callback, rect) = use_element_rect();
    let (style_callback, computed) = use_element_computed_style(TRACKED_PROPERTIES);
    let registry = use_shared_element_registry();

    let id_cloned = id.clone();

    use_effect(move || {
        if let Some(current_rect) = rect() {
            let computed_snapshot = computed.read().clone();
            let reg = registry.read().clone();
            reg.record(
                id_cloned.clone(),
                ElementSnapshot {
                    rect: current_rect,
                    computed: computed_snapshot,
                    timestamp_ms: 0.0,
                },
            );
        }
    });

    let _ = transition;

    let id_attr = id.clone();
    rsx! {
        div {
            class: "ui-shared-element",
            "data-shared-id": "{id_attr}",
            onmounted: move |evt| {
                rect_callback.0.call(evt.clone());
                style_callback.0.call(evt);
            },
            {children}
        }
    }
}
