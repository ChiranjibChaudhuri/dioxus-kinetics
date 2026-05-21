use dioxus::prelude::*;

#[component]
pub fn TextField(
    id: String,
    label: String,
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] help_text: String,
    #[props(default)] error_text: String,
    #[props(default)] leading_text: String,
    #[props(default)] trailing_text: String,
    #[props(default)] disabled: bool,
    #[props(default)] invalid: bool,
) -> Element {
    let described_by = described_by(&id, !help_text.is_empty(), !error_text.is_empty());
    let field_class = if invalid {
        "ui-field ui-text-field ui-field--invalid"
    } else {
        "ui-field ui-text-field"
    };

    rsx! {
        div { class: "{field_class}",
            label { class: "ui-field-label", r#for: "{id}", "{label}" }
            div { class: "ui-field-row",
                if !leading_text.is_empty() {
                    span { class: "ui-field-adornment ui-field-adornment--leading", "{leading_text}" }
                }
                input {
                    id: "{id}",
                    class: "ui-field-control",
                    value: "{value}",
                    placeholder: "{placeholder}",
                    disabled,
                    "aria-invalid": if invalid { "true" } else { "false" },
                    "aria-describedby": "{described_by}",
                }
                if !trailing_text.is_empty() {
                    span { class: "ui-field-adornment ui-field-adornment--trailing", "{trailing_text}" }
                }
            }
            if !help_text.is_empty() {
                p { id: "{id}-help", class: "ui-field-help", "{help_text}" }
            }
            if !error_text.is_empty() {
                p { id: "{id}-error", class: "ui-field-error", "{error_text}" }
            }
        }
    }
}

#[component]
pub fn Checkbox(
    id: String,
    label: String,
    #[props(default)] description: String,
    #[props(default)] checked: bool,
    #[props(default)] indeterminate: bool,
    #[props(default)] disabled: bool,
) -> Element {
    let wrapper_class = if indeterminate {
        "ui-checkbox ui-checkbox--mixed"
    } else {
        "ui-checkbox"
    };
    let aria_checked = if indeterminate {
        "mixed"
    } else if checked {
        "true"
    } else {
        "false"
    };

    rsx! {
        div { class: "{wrapper_class}",
            input {
                id: "{id}",
                class: "ui-checkbox-input",
                r#type: "checkbox",
                checked,
                disabled,
                "aria-checked": "{aria_checked}",
            }
            div { class: "ui-checkbox-copy",
                label { class: "ui-checkbox-label", r#for: "{id}", "{label}" }
                if !description.is_empty() {
                    p { class: "ui-checkbox-description", "{description}" }
                }
            }
        }
    }
}

#[component]
pub fn Switch(
    id: String,
    label: String,
    #[props(default)] description: String,
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
) -> Element {
    let aria_checked = if checked { "true" } else { "false" };
    let aria_disabled = if disabled { "true" } else { "false" };

    rsx! {
        div { class: "ui-switch",
            button {
                id: "{id}",
                class: "ui-switch-control",
                r#type: "button",
                role: "switch",
                disabled,
                "aria-checked": "{aria_checked}",
                "aria-disabled": "{aria_disabled}",
                span { class: "ui-switch-thumb" }
            }
            div { class: "ui-switch-copy",
                label { class: "ui-switch-label", r#for: "{id}", "{label}" }
                if !description.is_empty() {
                    p { class: "ui-switch-description", "{description}" }
                }
            }
        }
    }
}

fn described_by(id: &str, has_help: bool, has_error: bool) -> String {
    match (has_help, has_error) {
        (true, true) => format!("{id}-help {id}-error"),
        (true, false) => format!("{id}-help"),
        (false, true) => format!("{id}-error"),
        (false, false) => String::new(),
    }
}
