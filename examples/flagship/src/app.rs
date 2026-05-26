use dioxus::prelude::*;
use ui_styles::library_css;

use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main { class: "flagship-shell",
            p { "Flagship under construction." }
        }
    }
}
