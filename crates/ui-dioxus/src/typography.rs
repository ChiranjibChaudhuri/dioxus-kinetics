//! Typography primitives: `Heading` and `Text`.
//!
//! A single [`TextVariant`] scale (mirroring Apple's Human Interface
//! type ramp) drives both components. `Text` renders a block/inline
//! element (`p`/`span`/`div`) carrying the variant class; `Heading`
//! renders the semantically correct `h1`..`h6` for its `level` and
//! defaults its visual variant from that level so the document
//! outline and the visual hierarchy stay in sync without the consumer
//! having to repeat themselves.

use dioxus::prelude::*;

/// The shared type scale. Maps to `ui-text ui-text--<variant>` where
/// `<variant>` is the kebab-cased name (e.g. `LargeTitle` ->
/// `largetitle`, `Title3` -> `title3`). `Body` is the default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextVariant {
    Caption2,
    Caption,
    Footnote,
    Subhead,
    Callout,
    #[default]
    Body,
    Headline,
    Title3,
    Title2,
    Title1,
    LargeTitle,
    Display,
}

impl TextVariant {
    /// The variant's kebab-case suffix used in the CSS class.
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Caption2 => "caption2",
            Self::Caption => "caption",
            Self::Footnote => "footnote",
            Self::Subhead => "subhead",
            Self::Callout => "callout",
            Self::Body => "body",
            Self::Headline => "headline",
            Self::Title3 => "title3",
            Self::Title2 => "title2",
            Self::Title1 => "title1",
            Self::LargeTitle => "largetitle",
            Self::Display => "display",
        }
    }

    /// The full class string for this variant.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Caption2 => "ui-text ui-text--caption2",
            Self::Caption => "ui-text ui-text--caption",
            Self::Footnote => "ui-text ui-text--footnote",
            Self::Subhead => "ui-text ui-text--subhead",
            Self::Callout => "ui-text ui-text--callout",
            Self::Body => "ui-text ui-text--body",
            Self::Headline => "ui-text ui-text--headline",
            Self::Title3 => "ui-text ui-text--title3",
            Self::Title2 => "ui-text ui-text--title2",
            Self::Title1 => "ui-text ui-text--title1",
            Self::LargeTitle => "ui-text ui-text--largetitle",
            Self::Display => "ui-text ui-text--display",
        }
    }
}

/// The default visual variant for a heading at `level`. Levels 1..3
/// map to the matching title size; deeper levels fall back to `Body`.
const fn default_heading_variant(level: u8) -> TextVariant {
    match level {
        1 => TextVariant::Title1,
        2 => TextVariant::Title2,
        3 => TextVariant::Title3,
        _ => TextVariant::Body,
    }
}

/// A semantic heading. `level` (clamped to 1..=6) selects the
/// `h1`..`h6` tag; `variant` overrides the visual size, defaulting
/// from the level so the outline and the type scale agree.
#[component]
pub fn Heading(
    #[props(default = 2)] level: u8,
    #[props(default)] variant: Option<TextVariant>,
    children: Element,
) -> Element {
    let level = level.clamp(1, 6);
    let variant = variant.unwrap_or_else(|| default_heading_variant(level));
    let class = format!("ui-heading {}", variant.class_name());

    rsx! {
        {
            match level {
                1 => rsx! { h1 { class: "{class}", {children} } },
                2 => rsx! { h2 { class: "{class}", {children} } },
                3 => rsx! { h3 { class: "{class}", {children} } },
                4 => rsx! { h4 { class: "{class}", {children} } },
                5 => rsx! { h5 { class: "{class}", {children} } },
                _ => rsx! { h6 { class: "{class}", {children} } },
            }
        }
    }
}

/// Body / inline text. `as_element` chooses the rendered tag from a
/// small allowlist (`p`, `span`, `div`); anything else falls back to
/// `p`. `variant` selects the type scale (default `Body`).
#[component]
pub fn Text(
    #[props(default)] variant: TextVariant,
    #[props(default = "p".to_string())] as_element: String,
    children: Element,
) -> Element {
    let class = variant.class_name();

    rsx! {
        {
            match as_element.as_str() {
                "span" => rsx! { span { class: "{class}", {children} } },
                "div" => rsx! { div { class: "{class}", {children} } },
                _ => rsx! { p { class: "{class}", {children} } },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_variant_maps_to_kebab_class() {
        assert_eq!(TextVariant::Body.class_name(), "ui-text ui-text--body");
        assert_eq!(
            TextVariant::Caption2.class_name(),
            "ui-text ui-text--caption2"
        );
        assert_eq!(
            TextVariant::LargeTitle.class_name(),
            "ui-text ui-text--largetitle"
        );
        assert_eq!(TextVariant::Title3.class_name(), "ui-text ui-text--title3");
        assert_eq!(
            TextVariant::Display.class_name(),
            "ui-text ui-text--display"
        );
    }

    #[test]
    fn text_variant_default_is_body() {
        assert_eq!(TextVariant::default(), TextVariant::Body);
    }

    #[test]
    fn heading_level_picks_matching_title_variant() {
        assert_eq!(default_heading_variant(1), TextVariant::Title1);
        assert_eq!(default_heading_variant(2), TextVariant::Title2);
        assert_eq!(default_heading_variant(3), TextVariant::Title3);
        assert_eq!(default_heading_variant(4), TextVariant::Body);
        assert_eq!(default_heading_variant(6), TextVariant::Body);
    }
}
