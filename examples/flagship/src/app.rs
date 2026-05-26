use component_gallery::persistence::prefers_reduced_motion;
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

    // Read OS-level prefers-reduced-motion once at mount. The flagship has
    // no runtime motion-pref toggle (unlike the gallery's PreferenceBar), so
    // changes during a session require a refresh. That's acceptable for a
    // marketing page; the cost of subscribing to changes (forget'd Closure +
    // MediaQueryList plumbing) is not justified for this surface.
    let reduced = prefers_reduced_motion();
    use_context_provider(|| ReducedMotion(reduced));

    let motion_attr = if reduced { "reduced" } else { "normal" };

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main {
            class: "flagship-shell",
            "data-ui-motion": "{motion_attr}",
            Hero {}
            Story {}
            Features {}
            Metrics {}
            CallToAction {}
        }
    }
}
