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

#[test]
fn advanced_wave_components_are_ready_with_accessibility_notes() {
    let docs = component_docs();

    for name in [
        "TextField",
        "Checkbox",
        "Switch",
        "Tabs",
        "Dialog",
        "Toast",
        "CommandMenu",
        "Tooltip",
        "Toolbar",
        "Sidebar",
        "MetricCard",
        "EmptyState",
    ] {
        let doc = docs
            .iter()
            .find(|doc| doc.name == name)
            .expect("component doc exists");
        assert_eq!(doc.status, ComponentStatus::Ready, "{name} should be ready");
        assert!(doc.render.is_some(), "{name} should render a live example");
        assert!(
            !doc.accessibility.is_empty(),
            "{name} needs accessibility notes"
        );
        assert!(!doc.snippet.is_empty(), "{name} needs a snippet");
    }
}

use dioxus::prelude::*;

#[test]
fn gallery_renders_ready_examples_and_coming_soon_entries() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Unified UI Component Gallery"));
    for category in component_gallery::categories() {
        assert!(html.contains(category.label()));
    }
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

#[test]
fn gallery_embeds_styles_for_gallery_and_component_classes() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains(".gallery-shell"));
    assert!(html.contains(".ui-button--primary"));
    assert!(html.contains(".ui-glass-surface"));
    assert!(html.contains("backdrop-filter"));
}

#[test]
fn root_readme_mentions_component_gallery() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
    let readme = std::fs::read_to_string(readme_path).expect("README.md should be readable");

    assert!(readme.contains("Component Gallery"));
    assert!(readme.contains("cargo check -p component-gallery"));
    assert!(readme.contains("dx serve --package component-gallery"));
}
