use std::rc::Rc;

use dioxus::prelude::*;
use ui_runtime::SharedElementRegistry;

#[component]
pub fn SharedLayout(children: Element) -> Element {
    use_context_provider(|| Signal::new(Rc::new(SharedElementRegistry::default())));

    rsx! {
        div {
            class: "ui-shared-layout",
            {children}
        }
    }
}
