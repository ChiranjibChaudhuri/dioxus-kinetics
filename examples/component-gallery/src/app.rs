use dioxus::prelude::*;
use ui_styles::library_css;

use crate::docs::{categories, component_docs, ComponentCategory, ComponentDoc, ComponentStatus};
use crate::styles::GALLERY_CSS;

#[component]
pub fn App() -> Element {
    let shared_css = library_css();

    rsx! {
        style { "{shared_css}" }
        style { "{GALLERY_CSS}" }
        div {
            class: "gallery-shell",
            "data-ui-theme": "light",
            "data-ui-density": "comfortable",
            aside { class: "gallery-rail",
                div { class: "gallery-brand",
                    span { class: "gallery-mark", "UI" }
                    div {
                        h1 { "Unified UI" }
                        p { "Component reference" }
                    }
                }
                nav { class: "gallery-nav", aria_label: "Component categories",
                    for category in categories() {
                        a { href: "#{category.slug()}", "{category.label()}" }
                    }
                }
            }
            main { class: "gallery-main",
                header { class: "gallery-header",
                    p { class: "gallery-eyebrow", "Dioxus SaaS library" }
                    h2 { "Unified UI Component Gallery" }
                    p {
                        "Semantic components grouped by product function, with live rendered examples for available primitives and disabled coming-soon entries for the next phase."
                    }
                }
                section { class: "gallery-controls", "aria-label": "Preview settings",
                    div { class: "gallery-control-group",
                        span { class: "gallery-control-label", "Theme" }
                        button { class: "ui-button ui-button--primary", r#type: "button", "Light" }
                        button { class: "ui-button ui-button--secondary", r#type: "button", "Dark" }
                    }
                    div { class: "gallery-control-group",
                        span { class: "gallery-control-label", "Density" }
                        button { class: "ui-button ui-button--secondary", r#type: "button", "Compact" }
                        button { class: "ui-button ui-button--primary", r#type: "button", "Comfortable" }
                        button { class: "ui-button ui-button--secondary", r#type: "button", "Spacious" }
                    }
                }
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
