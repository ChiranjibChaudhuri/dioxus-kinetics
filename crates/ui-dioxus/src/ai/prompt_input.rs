//! `PromptInput` — an auto-growing prompt composer with a send/stop
//! action. Enter submits; Shift+Enter inserts a newline. While the
//! assistant streams, the send affordance becomes a square stop button.

use dioxus::prelude::*;

/// Auto-grow the prompt textarea by setting its height to its
/// `scrollHeight`, capped at ~200px (after which it scrolls). Scoped to
/// this instance's textarea by its unique `id` so multiple
/// `PromptInput`s on a page each grow independently. The listener is
/// attached to the element, so it is garbage-collected with the node.
fn install_prompt_autogrow(id: &str) {
    let script = format!(
        r#"
        (function() {{
            const ta = document.getElementById('{id}');
            if (!ta || ta.__kineticsAutogrow) return;
            ta.__kineticsAutogrow = true;
            const grow = () => {{
                ta.style.height = 'auto';
                ta.style.height = Math.min(ta.scrollHeight, 200) + 'px';
            }};
            ta.addEventListener('input', grow);
            grow();
        }})();
    "#
    );
    let _ = dioxus::document::eval(&script);
}

/// A chat-style prompt composer.
///
/// `value` is fully controlled by the caller via `on_input`. Submitting
/// (Enter or the send button) fires `on_submit` with the current value;
/// `prevent_default` stops the browser's native form navigation.
/// `Shift+Enter` is allowed through so the textarea inserts a newline.
/// While `streaming`, the action button switches to a stop control that
/// fires `on_stop`.
#[component]
pub fn PromptInput(
    value: String,
    #[props(default)] streaming: bool,
    #[props(default = "Ask anything…".to_string())] placeholder: String,
    on_input: Option<EventHandler<String>>,
    on_submit: Option<EventHandler<String>>,
    on_stop: Option<EventHandler<()>>,
) -> Element {
    let submit_value = value.clone();
    let key_value = value.clone();

    // Stable per-instance id so the auto-grow eval targets *this*
    // textarea (getElementById) rather than the first match on the page.
    let textarea_id =
        use_hook(|| format!("ui-prompt-textarea-{}", dioxus_core::current_scope_id().0));

    // Reset the textarea height whenever the controlled value changes
    // (e.g. cleared on submit) so the box shrinks back down.
    use_effect(use_reactive((&value, &textarea_id), move |(_value, id)| {
        let script = format!(
            r#"const ta=document.getElementById('{id}'); if(ta){{ta.style.height='auto'; ta.style.height=Math.min(ta.scrollHeight,200)+'px';}}"#
        );
        let _ = dioxus::document::eval(&script);
    }));

    rsx! {
        form {
            class: "ui-prompt-input",
            onsubmit: move |evt| {
                evt.prevent_default();
                if let Some(handler) = &on_submit {
                    handler.call(submit_value.clone());
                }
            },
            textarea {
                class: "ui-prompt-textarea",
                id: "{textarea_id}",
                value: "{value}",
                placeholder: "{placeholder}",
                rows: "1",
                "aria-label": "{placeholder}",
                onmounted: {
                    let textarea_id = textarea_id.clone();
                    move |_evt| {
                        install_prompt_autogrow(&textarea_id);
                    }
                },
                oninput: move |evt| {
                    if let Some(handler) = &on_input {
                        handler.call(evt.value());
                    }
                },
                onkeydown: move |evt| {
                    // Enter submits; Shift+Enter falls through to insert a newline.
                    if evt.key() == Key::Enter && !evt.modifiers().shift() {
                        evt.prevent_default();
                        if let Some(handler) = &on_submit {
                            handler.call(key_value.clone());
                        }
                    }
                },
            }
            if streaming {
                button {
                    class: "ui-prompt-stop",
                    r#type: "button",
                    "aria-label": "Stop",
                    onclick: move |_evt| {
                        if let Some(handler) = &on_stop {
                            handler.call(());
                        }
                    },
                    svg {
                        "viewBox": "0 0 24 24",
                        width: "18",
                        height: "18",
                        fill: "currentColor",
                        "aria-hidden": "true",
                        rect { x: "6", y: "6", width: "12", height: "12", rx: "2" }
                    }
                }
            } else {
                button {
                    class: "ui-prompt-send",
                    r#type: "submit",
                    "aria-label": "Send",
                    svg {
                        "viewBox": "0 0 24 24",
                        width: "18",
                        height: "18",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        "aria-hidden": "true",
                        path { d: "M12 19V5M5 12l7-7 7 7" }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /// Mirror of the submit gate in `onkeydown`: Enter without Shift
    /// submits; Shift+Enter does not.
    fn enter_submits(is_enter: bool, shift: bool) -> bool {
        is_enter && !shift
    }

    #[test]
    fn plain_enter_submits_shift_enter_does_not() {
        assert!(enter_submits(true, false));
        assert!(!enter_submits(true, true));
        assert!(!enter_submits(false, false));
    }
}
