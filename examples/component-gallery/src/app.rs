use dioxus::prelude::*;
use ui_styles::library_css;

use crate::docs::{categories, component_docs, ComponentCategory, ComponentDoc, ComponentStatus};
use crate::styles::GALLERY_CSS;

#[component]
pub fn App() -> Element {
    let prefs = crate::controls::GalleryPrefs::use_provided();
    use_context_provider(|| prefs);

    let theme_attr = prefs.theme.read().attr_value();
    let density_attr = prefs.density.read().attr_value();
    let motion_attr = prefs.motion.read().attr_value();
    let glass_attr = prefs.glass.read().attr_value();

    let shared_css = library_css();

    rsx! {
        style { "{shared_css}" }
        style { "{GALLERY_CSS}" }
        div {
            class: "gallery-shell",
            "data-ui-theme": "{theme_attr}",
            "data-ui-density": "{density_attr}",
            "data-ui-motion": "{motion_attr}",
            "data-ui-glass-policy": "{glass_attr}",
            aside { class: "gallery-rail",
                div { class: "gallery-brand",
                    div {
                        class: "gallery-logo",
                        aria_label: "Kinetics",
                        dangerous_inner_html: crate::brand::KINETICS_LOGO_SVG,
                    }
                    span { class: "visually-hidden", "Kinetics component gallery" }
                }
                nav { class: "gallery-nav", aria_label: "Component categories",
                    for category in categories() {
                        a { href: "#{category.slug()}", "{category.label()}" }
                    }
                }
            }
            main { class: "gallery-main",
                header { class: "gallery-header",
                    p { class: "gallery-eyebrow", "Dioxus Kinetics library" }
                    h2 { "Kinetics Component Gallery" }
                    p {
                        "Semantic components grouped by product function, with live rendered examples for available primitives and disabled coming-soon entries for the next phase."
                    }
                }
                crate::controls::PreferenceBar {}
                nav { class: "gallery-mobile-tabs", aria_label: "Component categories",
                    for category in categories() {
                        a { href: "#{category.slug()}", "{category.label()}" }
                    }
                }
                for category in categories() {
                    CategorySection { category: *category }
                }
            }
        }
    }
}

#[component]
fn CategorySection(category: ComponentCategory) -> Element {
    let docs = component_docs()
        .iter()
        .filter(|doc| doc.category == category)
        .collect::<Vec<_>>();

    rsx! {
        section { id: "{category.slug()}", class: "gallery-section",
            div { class: "gallery-section-heading",
                h3 { "{category.label()}" }
                p { "{category.description()}" }
            }
            div { class: "gallery-grid",
                for doc in docs {
                    {component_entry(doc)}
                }
            }
        }
    }
}

fn component_entry(doc: &'static ComponentDoc) -> Element {
    let status_class = match doc.status {
        ComponentStatus::Ready => "gallery-status gallery-status--ready",
        ComponentStatus::ComingSoon => "gallery-status gallery-status--soon",
    };

    rsx! {
        article { class: "gallery-entry",
            div { class: "gallery-entry-copy",
                div { class: "gallery-entry-title",
                    h4 { "{doc.name}" }
                    span { class: "{status_class}", "{doc.status.label()}" }
                }
                p { "{doc.summary}" }
                div { class: "gallery-accessibility",
                    strong { "Accessibility" }
                    p { "{doc.accessibility}" }
                }
            }
            div { class: "gallery-example",
                {rendered_example(doc)}
            }
            pre { class: "gallery-code",
                code { class: "language-rust", "{doc.snippet}" }
            }
        }
    }
}

fn rendered_example(doc: &'static ComponentDoc) -> Element {
    match (doc.status, doc.render) {
        (ComponentStatus::Ready, Some(render)) => rsx! {
            div { class: "gallery-preview gallery-preview--ready", {render()} }
        },
        _ => rsx! {
            div { class: "gallery-preview gallery-preview--soon", aria_disabled: "true",
                span { "Coming soon" }
                p { "{doc.name} will render here when the component lands." }
            }
        },
    }
}
