use dioxus::prelude::*;
use ui_dioxus::{Button, ButtonVariant, Surface};
#[cfg(not(feature = "liquid-glass"))]
use ui_dioxus::GlassSurface;
#[cfg(not(feature = "liquid-glass"))]
use ui_glass::{GlassLevel, GlassTone};

#[test]
fn button_renders_semantic_button() {
    let html = dioxus_ssr::render_element(rsx! {
        Button {
            variant: ButtonVariant::Primary,
            "Save"
        }
    });

    assert!(html.contains("<button"));
    assert!(html.contains("ui-button--primary"));
    assert!(html.contains("Save"));
}

#[test]
fn surface_renders_section_with_surface_class() {
    let html = dioxus_ssr::render_element(rsx! {
        Surface {
            "Panel"
        }
    });

    assert!(html.contains("<section"));
    assert!(html.contains("ui-surface"));
}

#[cfg(not(feature = "liquid-glass"))]
#[test]
fn glass_surface_uses_semantic_glass_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        GlassSurface {
            level: GlassLevel::Chrome,
            tone: GlassTone::Neutral,
            "Toolbar"
        }
    });

    assert!(html.contains("data-glass-level=\"chrome\""));
    assert!(html.contains("data-glass-tone=\"neutral\""));
}
