use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LowerThirdAccent {
    #[default]
    Primary,
    Secondary,
}

/// Broadcast-style chyron with name + role. Sits at the bottom-left
/// of the parent container; consumer is responsible for positioning
/// (we apply no absolute positioning).
#[component]
pub fn LowerThird(
    name: String,
    role: String,
    accent: Option<LowerThirdAccent>,
) -> Element {
    let accent = accent.unwrap_or_default();
    let accent_class = match accent {
        LowerThirdAccent::Primary => "ui-block-lower-third--primary",
        LowerThirdAccent::Secondary => "ui-block-lower-third--secondary",
    };
    let aria = format!("{name}, {role}");
    rsx! {
        div {
            class: "ui-block-lower-third {accent_class}",
            "aria-label": "{aria}",
            "data-block": "lower-third",
            div { class: "ui-block-lower-third__bar" }
            div { class: "ui-block-lower-third__text",
                div { class: "ui-block-lower-third__name", "{name}" }
                div { class: "ui-block-lower-third__role", "{role}" }
            }
        }
    }
}
