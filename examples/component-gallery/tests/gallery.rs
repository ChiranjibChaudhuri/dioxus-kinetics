use component_gallery::{categories, component_docs, ComponentCategory, ComponentStatus};

#[test]
fn registry_groups_components_by_product_category() {
    let categories = categories();

    assert_eq!(
        categories,
        &[
            ComponentCategory::Actions,
            ComponentCategory::Inputs,
            ComponentCategory::Layout,
            ComponentCategory::Surfaces,
            ComponentCategory::Feedback,
            ComponentCategory::Motion,
        ]
    );
}

#[test]
fn registry_contains_ready_and_coming_soon_components() {
    let docs = component_docs();

    assert!(docs
        .iter()
        .any(|doc| doc.name == "Button" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "Surface" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "GlassSurface" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "Stack" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "TextField" && doc.status == ComponentStatus::ComingSoon));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "SharedElement" && doc.status == ComponentStatus::ComingSoon));
}

#[test]
fn registry_status_matches_live_renderer_availability() {
    for doc in component_docs() {
        match doc.status {
            ComponentStatus::Ready => {
                assert!(
                    doc.render.is_some(),
                    "{} should render a live example",
                    doc.name
                );
            }
            ComponentStatus::ComingSoon => {
                assert!(
                    doc.render.is_none(),
                    "{} should not render unavailable components",
                    doc.name
                );
            }
        }
    }
}

use dioxus::prelude::*;

#[test]
fn gallery_renders_ready_examples_and_coming_soon_entries() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Unified UI Component Gallery"));
    assert!(html.contains("Actions"));
    assert!(html.contains("Button"));
    assert!(html.contains("Save changes"));
    assert!(html.contains("GlassSurface"));
    assert!(html.contains("Coming soon"));
    assert!(html.contains("TextField"));
    assert!(html.contains("SharedElement"));
}

#[test]
fn gallery_renders_snippets_as_rust_code_blocks() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("language-rust"));
    assert!(html.contains("ButtonVariant::Primary"));
    assert!(html.contains("GlassLevel::Floating"));
}
