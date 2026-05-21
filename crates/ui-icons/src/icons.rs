use dioxus::prelude::*;

pub const CLOSE_PATH_D: &str = "M6 6l12 12M18 6L6 18";
pub const CHECK_PATH_D: &str = "M5 12l4 4 10-10";
pub const CHEVRON_DOWN_PATH_D: &str = "M6 9l6 6 6-6";
pub const CHEVRON_RIGHT_PATH_D: &str = "M9 6l6 6-6 6";
pub const PLUS_PATH_D: &str = "M12 5v14M5 12h14";
pub const MINUS_PATH_D: &str = "M5 12h14";
pub const TRASH_PATH_D: &str =
    "M4 7h16M9 7V4h6v3M6 7l1 13h10l1-13M10 11v6M14 11v6";
pub const SEARCH_PATH_D: &str = "M10 17a7 7 0 1 1 0-14 7 7 0 0 1 0 14zM21 21l-6-6";

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
