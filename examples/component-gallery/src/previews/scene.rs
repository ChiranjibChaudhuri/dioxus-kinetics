use dioxus::prelude::*;

use crate::previews::scenes::product_intro::ProductIntroScene;

pub fn product_intro_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            ProductIntroScene {}
        }
    }
}
