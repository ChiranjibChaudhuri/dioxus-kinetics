//! `SourceCard` / `SourceRail` — search-result style source previews and
//! a horizontally scroll-snapping row to hold them.

use dioxus::prelude::*;

/// Compute the single-letter favicon fallback (first character of the
/// domain, uppercased) used when no favicon image is supplied.
fn favicon_letter(domain: &str) -> String {
    domain
        .trim()
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string())
}

/// A single source preview. With an `href` it renders as a link card;
/// without one it is a static `article`. Shows a favicon (image when
/// provided, otherwise a letter monogram), the title, an index+domain
/// line, and an optional snippet.
#[component]
pub fn SourceCard(
    index: u32,
    title: String,
    domain: String,
    #[props(default)] snippet: String,
    #[props(default)] href: String,
    #[props(default)] favicon: String,
) -> Element {
    let has_href = !href.is_empty();
    let has_favicon = !favicon.is_empty();
    let letter = favicon_letter(&domain);

    let inner = rsx! {
        span { class: "ui-source-favicon", "aria-hidden": "true",
            if has_favicon {
                img { src: "{favicon}", alt: "", loading: "lazy" }
            } else {
                "{letter}"
            }
        }
        span { class: "ui-source-title", "{title}" }
        span { class: "ui-source-domain", "{index} · {domain}" }
        if !snippet.is_empty() {
            span { class: "ui-source-snippet", "{snippet}" }
        }
    };

    if has_href {
        rsx! {
            span { role: "listitem",
                a {
                    class: "ui-source-card",
                    href: "{href}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    {inner}
                }
            }
        }
    } else {
        rsx! {
            article {
                class: "ui-source-card",
                role: "listitem",
                {inner}
            }
        }
    }
}

/// Horizontal, scroll-snapping rail of `SourceCard`s, exposed as an ARIA
/// list so the cards read as a coherent group.
#[component]
pub fn SourceRail(children: Element) -> Element {
    rsx! {
        div {
            class: "ui-source-rail",
            role: "list",
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn favicon_letter_uppercases_first_char() {
        assert_eq!(favicon_letter("rust-lang.org"), "R");
        assert_eq!(favicon_letter("example.com"), "E");
    }

    #[test]
    fn favicon_letter_handles_empty_and_whitespace() {
        assert_eq!(favicon_letter(""), "?");
        assert_eq!(favicon_letter("   "), "?");
        assert_eq!(favicon_letter("  wiki.org"), "W");
    }
}
