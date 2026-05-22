use component_gallery::{categories, component_docs, ComponentCategory, ComponentStatus};

#[test]
fn registry_groups_components_by_product_category() {
    let categories = categories();

    assert_eq!(
        categories,
        &[
            ComponentCategory::Foundations,
            ComponentCategory::Actions,
            ComponentCategory::Inputs,
            ComponentCategory::Navigation,
            ComponentCategory::Layout,
            ComponentCategory::Surfaces,
            ComponentCategory::Feedback,
            ComponentCategory::DataWorkflows,
            ComponentCategory::Motion,
            ComponentCategory::Composition,
            ComponentCategory::Capture,
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
        .any(|doc| doc.name == "TextField" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "SharedElement" && doc.status == ComponentStatus::Ready));
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

    assert!(html.contains("Kinetics Component Gallery"));
    for category in component_gallery::categories() {
        assert!(html.contains(category.label()));
    }
    assert!(html.contains("Button"));
    assert!(html.contains("Save changes"));
    assert!(html.contains("GlassSurface"));
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
    assert!(html.contains(".gallery-preview .ui-dialog"));
    assert!(html.contains("backdrop-filter"));
}

#[test]
fn gallery_renders_advanced_workbench_controls_and_notes() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Theme"));
    assert!(html.contains("Density"));
    assert!(html.contains("Light"));
    assert!(html.contains("Dark"));
    assert!(html.contains("Compact"));
    assert!(html.contains("Spacious"));
    assert!(html.contains("Accessibility"));
    assert!(html.contains(".ui-command-menu"));
    assert!(
        html.contains("[data-ui-theme=&quot;dark&quot;]")
            || html.contains("[data-ui-theme=\"dark\"]")
    );
}

#[test]
fn gallery_renders_native_kinetics_examples_without_bridge_copy() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for expected in ["TimelineScope", "FrameStage", "CaptureStage", "GlassLayer"] {
        assert!(html.contains(expected), "missing gallery entry {expected}");
    }

    for rejected in ["GSAP", "Remotion", "HyperFrames"] {
        assert!(
            !html.contains(rejected),
            "gallery must not show bridge copy {rejected}"
        );
    }
}

#[test]
fn root_readme_mentions_component_gallery() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
    let readme = std::fs::read_to_string(readme_path).expect("README.md should be readable");

    assert!(readme.contains("Component Gallery"));
    assert!(readme.contains("cargo check -p component-gallery"));
    assert!(readme.contains("dx serve --package component-gallery"));
}

#[test]
fn gallery_glass_layer_preview_renders_tone_level_matrix() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("gallery-variant-grid--3x3"));
    for level in ["Subtle", "Floating", "Overlay"] {
        for tone in ["Neutral", "Info", "Warning"] {
            assert!(
                html.contains(&format!("{level} · {tone}")),
                "missing GlassLayer tile {level} · {tone}",
            );
        }
    }
}

#[test]
fn root_readme_describes_native_systems_without_bridge_language() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
    let readme = std::fs::read_to_string(readme_path).expect("README.md should be readable");

    for expected in ["ui-timeline", "ui-composition", "ui-capture"] {
        assert!(readme.contains(expected), "README missing {expected}");
    }

    for rejected in ["GSAP", "Remotion", "HyperFrames"] {
        assert!(
            !readme.contains(rejected),
            "README still contains bridge term {rejected}"
        );
    }
}

#[test]
fn gallery_brand_uses_kinetics_logo_and_name() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Kinetics"));
    assert!(!html.contains("Unified UI"));
    assert!(html.contains("<svg"));
    assert!(html.contains("dioxus-kinetics logo"));
}

#[test]
fn gallery_css_includes_logo_and_variant_grid_styles() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for selector in [
        ".gallery-logo",
        ".visually-hidden",
        ".gallery-variant-grid",
        ".gallery-variant-grid--3x3",
        ".gallery-variant-grid--3col",
        ".gallery-variant-grid--2col",
        ".gallery-variant-grid--stack",
        ".gallery-variant-tile",
        ".gallery-variant-label",
    ] {
        assert!(html.contains(selector), "missing CSS selector {selector}");
    }
}

#[test]
fn gallery_timeline_scope_preview_renders_three_variants() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("gallery-variant-grid--stack"));
    assert!(
        html.contains("\"data-stagger-index\": \"0\"") || html.contains("data-stagger-index=\"0\"")
    );
    for cue in ["rise-in", "enter", "settle", "pulse"] {
        assert!(
            html.contains(&format!("data-motion-cue=\"{cue}\"")),
            "missing TimelineScope cue {cue}",
        );
    }
    assert!(html.contains("data-ui-transparency=\"reduced\""));
}

#[test]
fn gallery_frame_stage_preview_renders_starting_frame_caption() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    assert!(html.contains("Frame 0 / 180"));
}

#[test]
fn gallery_includes_kinetic_box_and_presence_gate_entries() {
    let docs = component_gallery::component_docs();

    let kb = docs
        .iter()
        .find(|doc| doc.name == "KineticBox")
        .expect("KineticBox doc exists");
    assert_eq!(kb.status, component_gallery::ComponentStatus::Ready);
    assert!(kb.render.is_some());

    let pg = docs
        .iter()
        .find(|doc| doc.name == "PresenceGate")
        .expect("PresenceGate doc exists");
    assert_eq!(pg.status, component_gallery::ComponentStatus::Ready);
    assert!(pg.render.is_some());
}

#[test]
fn gallery_kinetic_box_preview_renders_three_cues() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for cue in ["rise-in", "fade-in", "slide-up"] {
        assert!(
            html.contains(&format!("data-motion-cue=\"{cue}\"")),
            "missing KineticBox cue {cue}",
        );
    }
}

#[test]
fn gallery_presence_gate_preview_renders_present_and_hidden_tiles() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Visible state"));
    assert!(html.contains("Hidden state"));
    assert!(html.contains("gallery-variant-grid--2col"));
}

#[test]
fn gallery_capture_stage_preview_renders_three_viewport_profiles() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for caption in [
        "Mobile · 360 × 640",
        "Tablet · 768 × 1024",
        "Desktop · 1440 × 900",
    ] {
        assert!(
            html.contains(caption),
            "missing CaptureStage caption {caption}",
        );
    }

    for viewport in ["mobile", "tablet", "desktop"] {
        assert!(
            html.contains(&format!("data-viewport=\"{viewport}\""))
                || html.contains(&format!("viewport=\"{viewport}\""))
                || html.contains(&format!(">{viewport}<")),
            "missing CaptureStage viewport prop {viewport}",
        );
    }
}

#[test]
fn root_readme_uses_kinetics_crate_name() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
    let readme = std::fs::read_to_string(readme_path).expect("README.md should be readable");

    assert!(readme.contains("use kinetics::prelude::*"));
    assert!(readme.contains("crates/kinetics"));
    assert!(!readme.contains("unified_ui"));
    assert!(!readme.contains("Unified UI"));
}

#[test]
fn gallery_includes_presence_entry_with_lifecycle_attrs() {
    let docs = component_gallery::component_docs();
    let p = docs
        .iter()
        .find(|d| d.name == "Presence")
        .expect("Presence doc exists");
    assert_eq!(p.status, component_gallery::ComponentStatus::Ready);
    assert!(p.render.is_some());

    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("data-presence-cue=\"rise\""), "got {html}");
    assert!(
        html.contains("data-presence-state=\"visible\""),
        "got {html}"
    );
    assert!(html.contains("Present"));
    assert!(html.contains("Hidden"));
}

#[test]
fn gallery_icon_button_is_ready_with_tone_size_matrix() {
    let docs = component_gallery::component_docs();
    let ib = docs
        .iter()
        .find(|d| d.name == "IconButton")
        .expect("IconButton doc exists");
    assert_eq!(ib.status, component_gallery::ComponentStatus::Ready);
    assert!(ib.render.is_some());

    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for tone in ["Neutral", "Primary", "Danger"] {
        for size in ["Compact", "Default", "Spacious"] {
            assert!(
                html.contains(&format!("{tone} · {size}")),
                "missing IconButton tile {tone} · {size}",
            );
        }
    }
    assert!(html.contains("ui-icon-button--danger"));
    assert!(html.contains("ui-icon-button--compact"));
}

#[test]
fn gallery_sequence_preview_renders_three_cues_with_inline_styles() {
    let docs = component_gallery::component_docs();
    let s = docs
        .iter()
        .find(|d| d.name == "Sequence")
        .expect("Sequence doc exists");
    assert_eq!(s.status, component_gallery::ComponentStatus::Ready);
    assert!(s.render.is_some());

    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    let inline_style_count =
        html.matches("style=\"opacity").count() + html.matches("style=\"transform").count();
    assert!(
        inline_style_count >= 1,
        "expected at least 1 inline-style KineticBox descendant on initial scrub frame; got {inline_style_count}",
    );
}

#[test]
fn gallery_shared_layout_and_shared_element_are_ready() {
    let docs = component_gallery::component_docs();
    let sl = docs
        .iter()
        .find(|d| d.name == "SharedLayout")
        .expect("SharedLayout doc exists");
    let se = docs
        .iter()
        .find(|d| d.name == "SharedElement")
        .expect("SharedElement doc exists");
    assert_eq!(sl.status, component_gallery::ComponentStatus::Ready);
    assert_eq!(se.status, component_gallery::ComponentStatus::Ready);
    assert!(sl.render.is_some());
    assert!(se.render.is_some());

    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    assert!(html.contains("data-shared-id=\""));
    assert!(html.contains("class=\"ui-shared-layout\""));
}

#[test]
fn gallery_shell_emits_all_four_preference_data_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains(r#"data-ui-theme="light""#));
    assert!(html.contains(r#"data-ui-density="comfortable""#));
    assert!(html.contains(r#"data-ui-motion="normal""#));
    assert!(html.contains(r#"data-ui-glass-policy="translucent""#));
}

#[test]
fn preference_bar_renders_all_four_toggle_groups() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains(r#"role="radiogroup""#));
    // One radiogroup per preference.
    let radiogroup_count = html.matches(r#"role="radiogroup""#).count();
    assert!(
        radiogroup_count >= 4,
        "expected >=4 radiogroups, got {radiogroup_count}"
    );

    // Each labelled.
    for label in ["Theme", "Density", "Motion", "Glass"] {
        assert!(html.contains(label), "missing toggle group label: {label}");
    }

    // The current value of each shows aria-checked=true on exactly one option.
    for value in ["Light", "Comfortable", "Normal", "Translucent"] {
        assert!(
            html.contains(value),
            "missing default-selected option: {value}"
        );
    }
}

#[test]
fn gallery_css_includes_ambient_mesh_and_toggle_group_styles() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for selector in [
        ".gallery-toggle-group",
        ".gallery-ambient-mesh",
        ".gallery-section--glass-stage",
    ] {
        assert!(
            html.contains(selector),
            "missing CSS selector {selector}",
        );
    }

    // Sticky position on the controls bar so it stays reachable while scrolling.
    assert!(html.contains("position: sticky"));
}

#[test]
fn motion_previews_use_replay_frame() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    // Each motion-category live demo should be wrapped in a gallery-demo-frame.
    let frame_count = html.matches("gallery-demo-frame").count();
    assert!(
        frame_count >= 3,
        "expected >=3 demo frames in motion previews, got {frame_count}"
    );
    // Replay button is present.
    assert!(html.contains("Replay"));
}

#[test]
fn timeline_previews_use_scrub_frame_with_range_slider() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    let scrub_count = html.matches("gallery-demo-frame--scrub").count();
    assert!(
        scrub_count >= 2,
        "expected >=2 scrub frames (Sequence, TimelineScope, FrameStage), got {scrub_count}"
    );
    let range_count = html.matches(r#"type="range""#).count();
    assert!(range_count >= 2, "expected >=2 range sliders, got {range_count}");
}

#[test]
fn shared_layout_preview_uses_flip_frame_with_swap_control() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    assert!(html.contains("gallery-demo-frame--flip"));
    assert!(html.contains("Swap layout"));
}
