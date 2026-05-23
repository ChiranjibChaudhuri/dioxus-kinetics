//! Modal dialog with focus trap and Escape-to-dismiss.

use dioxus::prelude::*;

/// Installs a Tab-cycling focus trap on the most recently mounted
/// `.ui-dialog-panel`. The handler is registered on the panel element so when
/// the panel is removed from the DOM, the listener is garbage-collected
/// together with it — no Rust-side teardown needed.
fn install_dialog_focus_trap() {
    const FOCUSABLE_SELECTOR: &str =
        "button:not([disabled]),[href],input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex=\"-1\"])";
    let script = format!(
        r#"
        (function() {{
            const panel = document.querySelector('.ui-dialog-panel');
            if (!panel || panel.__kineticsTrap) return;
            panel.__kineticsTrap = true;
            panel.addEventListener('keydown', (e) => {{
                if (e.key !== 'Tab') return;
                const f = panel.querySelectorAll('{selector}');
                if (f.length === 0) {{ e.preventDefault(); panel.focus(); return; }}
                const first = f[0];
                const last = f[f.length - 1];
                const active = document.activeElement;
                if (e.shiftKey && (active === first || active === panel)) {{
                    e.preventDefault();
                    last.focus();
                }} else if (!e.shiftKey && active === last) {{
                    e.preventDefault();
                    first.focus();
                }}
            }});
        }})();
        "#,
        selector = FOCUSABLE_SELECTOR,
    );
    let _ = dioxus::document::eval(&script);
}

#[component]
pub fn Dialog(
    title: String,
    #[props(default)] open: bool,
    #[props(default)] description: String,
    #[props(default)] body: String,
    #[props(default)] actions: Vec<String>,
    #[props(default = true)] dismissible: bool,
    on_dismiss: Option<EventHandler<()>>,
    on_action: Option<EventHandler<String>>,
) -> Element {
    if !open {
        return rsx! {};
    }

    let has_description = !description.is_empty();
    let described_by = if has_description {
        "ui-dialog-description"
    } else {
        ""
    };

    rsx! {
        div {
            class: "ui-dialog",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": "ui-dialog-title",
            "aria-describedby": "{described_by}",
            onkeydown: move |evt| {
                if dismissible && evt.key() == Key::Escape {
                    evt.stop_propagation();
                    if let Some(handler) = &on_dismiss {
                        handler.call(());
                    }
                }
            },
            div {
                class: "ui-dialog-backdrop",
                onclick: move |_evt| {
                    if dismissible {
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    }
                },
            }
            div {
                class: "ui-dialog-panel",
                tabindex: "-1",
                onmounted: move |evt| {
                    spawn(async move {
                        let _ = evt.set_focus(true).await;
                    });
                    install_dialog_focus_trap();
                },
                h2 { id: "ui-dialog-title", class: "ui-dialog-title", "{title}" }
                if has_description {
                    p { id: "ui-dialog-description", class: "ui-dialog-description", "{description}" }
                }
                if !body.is_empty() {
                    div { class: "ui-dialog-body", "{body}" }
                }
                if !actions.is_empty() {
                    div { class: "ui-dialog-actions",
                        for action in actions {
                            {
                                let action_id = action.clone();
                                rsx! {
                                    button {
                                        class: "ui-button ui-button--secondary",
                                        r#type: "button",
                                        onclick: move |_evt| {
                                            if let Some(handler) = &on_action {
                                                handler.call(action_id.clone());
                                            }
                                        },
                                        "{action}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
