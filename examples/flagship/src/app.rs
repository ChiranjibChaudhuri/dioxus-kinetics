use component_gallery::persistence::{prefers_color_scheme_dark, prefers_reduced_motion};
use dioxus::prelude::*;
use ui_runtime::ReducedMotion;
use ui_styles::library_css;

use crate::sections::cta::CallToAction;
use crate::sections::features::Features;
use crate::sections::hero::Hero;
use crate::sections::metrics::Metrics;
use crate::sections::story::Story;
use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    // Read OS-level prefers-reduced-motion and prefers-color-scheme once
    // at mount. The flagship has no runtime preference UI (unlike the
    // gallery's PreferenceBar), so OS changes during a session require a
    // refresh. That's acceptable for a marketing page; subscribing to
    // MediaQueryList changes (forget'd Closures) is not justified here.
    let reduced = prefers_reduced_motion();
    let dark = prefers_color_scheme_dark();
    use_context_provider(|| ReducedMotion(reduced));

    let motion_attr = if reduced { "reduced" } else { "normal" };
    let theme_attr = if dark { "dark" } else { "light" };

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main {
            class: "flagship-shell",
            "data-ui-motion": "{motion_attr}",
            "data-ui-theme": "{theme_attr}",
            Hero {}
            Story {}
            Features {}
            Metrics {}
            CallToAction {}
        }
    }
}
