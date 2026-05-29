use dioxus::prelude::*;

pub const CLOSE_PATH_D: &str = "M6 6l12 12M18 6L6 18";
pub const CHECK_PATH_D: &str = "M5 12l4 4 10-10";
pub const CHEVRON_DOWN_PATH_D: &str = "M6 9l6 6 6-6";
pub const CHEVRON_RIGHT_PATH_D: &str = "M9 6l6 6-6 6";
pub const PLUS_PATH_D: &str = "M12 5v14M5 12h14";
pub const MINUS_PATH_D: &str = "M5 12h14";
pub const TRASH_PATH_D: &str = "M4 7h16M9 7V4h6v3M6 7l1 13h10l1-13M10 11v6M14 11v6";
pub const SEARCH_PATH_D: &str = "M10 17a7 7 0 1 1 0-14 7 7 0 0 1 0 14zM21 21l-6-6";
pub const SPARKLE_PATH_D: &str =
    "M12 3l1.6 4.6L18 9.2l-4.4 1.6L12 15l-1.6-4.2L6 9.2l4.4-1.6L12 3zM18 14l.8 2.2L21 17l-2.2.8L18 20l-.8-2.2L15 17l2.2-.8L18 14z";
pub const STOP_PATH_D: &str = "M7 7h10v10H7z";
pub const SEND_PATH_D: &str = "M12 19V5M5 12l7-7 7 7";
pub const QUOTE_PATH_D: &str = "M7 7H4v6h3l-2 4h2l2-4V7zM17 7h-3v6h3l-2 4h2l2-4V7z";
pub const GLOBE_PATH_D: &str =
    "M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18zM3 12h18M12 3c2.5 2.4 3.9 5.7 4 9-.1 3.3-1.5 6.6-4 9-2.5-2.4-3.9-5.7-4-9 .1-3.3 1.5-6.6 4-9z";
pub const COPY_PATH_D: &str = "M9 9h10v10H9zM5 15H4V5h10v1";
pub const LINK_PATH_D: &str =
    "M10 14a4 4 0 0 0 5.66 0l3-3a4 4 0 0 0-5.66-5.66l-1.5 1.5M14 10a4 4 0 0 0-5.66 0l-3 3a4 4 0 0 0 5.66 5.66l1.5-1.5";

fn stroke_icon(d: &'static str, size: u32) -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 24 24",
            width: "{size}",
            height: "{size}",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            "aria-hidden": "true",
            path { d: "{d}" }
        }
    }
}

#[component]
pub fn Close(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(CLOSE_PATH_D, size)
}

#[component]
pub fn Check(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(CHECK_PATH_D, size)
}

#[component]
pub fn ChevronDown(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(CHEVRON_DOWN_PATH_D, size)
}

#[component]
pub fn ChevronRight(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(CHEVRON_RIGHT_PATH_D, size)
}

#[component]
pub fn Plus(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(PLUS_PATH_D, size)
}

#[component]
pub fn Minus(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(MINUS_PATH_D, size)
}

#[component]
pub fn Trash(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(TRASH_PATH_D, size)
}

#[component]
pub fn Search(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(SEARCH_PATH_D, size)
}

#[component]
pub fn Sparkle(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(SPARKLE_PATH_D, size)
}

#[component]
pub fn Stop(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(STOP_PATH_D, size)
}

#[component]
pub fn Send(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(SEND_PATH_D, size)
}

#[component]
pub fn Quote(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(QUOTE_PATH_D, size)
}

#[component]
pub fn Globe(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(GLOBE_PATH_D, size)
}

#[component]
pub fn Copy(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(COPY_PATH_D, size)
}

#[component]
pub fn Link(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(LINK_PATH_D, size)
}
