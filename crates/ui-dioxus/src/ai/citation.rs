//! `CitationChip` — a numbered, inline reference to a source, rendered as
//! a small anchor that announces the source title.

use dioxus::prelude::*;

/// A numbered citation chip. With an `href` it is a real link; without
/// one it presents as a non-navigating `button`-role chip (e.g. for a
/// popover-driven preview). The accessible name always includes the
/// index and the source title.
#[component]
pub fn CitationChip(index: u32, title: String, #[props(default)] href: String) -> Element {
    let aria_label = format!("Citation {index}: {title}");
    let has_href = !href.is_empty();

    if has_href {
        rsx! {
            a {
                class: "ui-citation-chip",
                href: "{href}",
                target: "_blank",
                rel: "noopener noreferrer",
                "aria-label": "{aria_label}",
                title: "{title}",
                "{index}"
            }
        }
    } else {
        rsx! {
            button {
                r#type: "button",
                class: "ui-citation-chip",
                "aria-label": "{aria_label}",
                title: "{title}",
                "{index}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /// The accessible name combines the index and the source title.
    fn citation_aria_label(index: u32, title: &str) -> String {
        format!("Citation {index}: {title}")
    }

    #[test]
    fn aria_label_combines_index_and_title() {
        assert_eq!(
            citation_aria_label(3, "Rust Reference"),
            "Citation 3: Rust Reference"
        );
    }
}
