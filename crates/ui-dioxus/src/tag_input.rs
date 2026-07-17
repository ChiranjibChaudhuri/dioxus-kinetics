//! `TagInput` — a free-text, chip-style multi-value editor. Typing Enter
//! commits the current draft as a tag (deduplicated); Backspace on an empty
//! draft removes the trailing tag; each chip exposes a remove control. The
//! component is controlled: callers own `tags` and apply updates from
//! `on_change`.

use dioxus::prelude::*;

#[component]
pub fn TagInput(
    id: String,
    label: String,
    tags: Vec<String>,
    #[props(default)] placeholder: String,
    #[props(default)] help_text: String,
    #[props(default)] disabled: bool,
    /// Optional cap on the number of tags; the control disables at the limit.
    #[props(default)]
    max: Option<usize>,
    /// Emits the full new tag list on add or remove.
    on_change: Option<EventHandler<Vec<String>>>,
) -> Element {
    let mut draft = use_signal(String::new);
    let at_limit = max.is_some_and(|limit| tags.len() >= limit);
    let control_disabled = disabled || at_limit;
    let described_by = if help_text.is_empty() {
        String::new()
    } else {
        format!("{id}-help")
    };

    rsx! {
        div { class: "ui-tag-input",
            label { class: "ui-tag-input-label", r#for: "{id}", "{label}" }
            div { class: "ui-tag-input-field",
                for (index, tag) in tags.iter().enumerate() {
                    span {
                        class: "ui-tag-input-chip",
                        key: "{tag}-{index}",
                        span { class: "ui-tag-input-chip-label", "{tag}" }
                        if on_change.is_some() {
                            button {
                                class: "ui-tag-input-chip-remove",
                                r#type: "button",
                                disabled,
                                "aria-label": "Remove {tag}",
                                onclick: {
                                    let handler = on_change.clone();
                                    let owned_tags = tags.clone();
                                    move |_| {
                                        let handler = handler.clone();
                                        if let Some(h) = handler {
                                            let mut next = owned_tags.clone();
                                            if index < next.len() {
                                                next.remove(index);
                                            }
                                            h.call(next);
                                        }
                                    }
                                },
                                "×"
                            }
                        }
                    }
                }
                input {
                    id: "{id}",
                    class: "ui-tag-input-control",
                    r#type: "text",
                    placeholder: "{placeholder}",
                    value: "{draft}",
                    disabled: control_disabled,
                    "aria-describedby": "{described_by}",
                    oninput: move |evt: FormEvent| {
                        draft.set(evt.value());
                    },
                    onkeydown: {
                        let handler = on_change.clone();
                        let owned_tags = tags.clone();
                        move |evt: KeyboardEvent| {
                            if disabled {
                                return;
                            }
                            match evt.code() {
                                Code::Enter => {
                                    let candidate = draft.read().trim().to_string();
                                    if !candidate.is_empty()
                                        && !owned_tags.contains(&candidate)
                                        && max.is_none_or(|limit| owned_tags.len() < limit)
                                    {
                                        let mut next = owned_tags.clone();
                                        next.push(candidate);
                                        if let Some(h) = handler.clone() {
                                            h.call(next);
                                        }
                                        draft.set(String::new());
                                    }
                                }
                                Code::Backspace if draft.read().is_empty() && !owned_tags.is_empty() => {
                                    let mut next = owned_tags.clone();
                                    next.pop();
                                    if let Some(h) = handler.clone() {
                                        h.call(next);
                                    }
                                }
                                _ => {}
                            }
                        }
                    },
                }
            }
            if !help_text.is_empty() {
                p { id: "{id}-help", class: "ui-tag-input-help", "{help_text}" }
            }
        }
    }
}
