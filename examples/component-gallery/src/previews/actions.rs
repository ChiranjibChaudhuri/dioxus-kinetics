use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn button_preview() -> Element {
    rsx! {
        div { class: "gallery-inline",
            Button { variant: ButtonVariant::Primary, "Save changes" }
            Button { variant: ButtonVariant::Secondary, "Review" }
            Button { variant: ButtonVariant::Ghost, "Dismiss" }
            Button { variant: ButtonVariant::Danger, "Delete" }
        }
    }
}

pub fn command_menu_preview() -> Element {
    rsx! {
        CommandMenu {
            open: false,
            query: "rep",
            selected_id: "reports",
            groups: vec![CommandGroup::new(
                "Navigation",
                vec![
                    CommandItem::new("dashboard", "Open dashboard", "Go to overview"),
                    CommandItem::new("reports", "Open reports", "Review exports"),
                ],
            )],
        }
    }
}

pub fn toolbar_preview() -> Element {
    rsx! {
        Toolbar {
            primary: vec!["New".to_string(), "Filter".to_string(), "Export".to_string()],
            secondary: "Updated now",
        }
    }
}

pub fn icon_button_preview() -> Element {
    let tones = [
        (IconButtonTone::Neutral, "Neutral"),
        (IconButtonTone::Primary, "Primary"),
        (IconButtonTone::Danger, "Danger"),
    ];
    let sizes = [
        (IconButtonSize::Compact, "Compact"),
        (IconButtonSize::Default, "Default"),
        (IconButtonSize::Spacious, "Spacious"),
    ];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3x3",
            for (tone, tone_label) in tones {
                for (size, size_label) in sizes {
                    div { class: "gallery-variant-tile",
                        span { class: "gallery-variant-label", "{tone_label} · {size_label}" }
                        IconButton {
                            label: format!("{tone_label} {size_label}"),
                            tone: tone,
                            size: size,
                            Plus { size: 16 }
                        }
                    }
                }
            }
        }
    }
}
