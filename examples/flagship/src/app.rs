use dioxus::prelude::*;
use ui_styles::library_css;

use crate::sections::hero::Hero;
use crate::sections::story::Story;
use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main { class: "flagship-shell",
            Hero {}
            Story {}
        }
    }
}
