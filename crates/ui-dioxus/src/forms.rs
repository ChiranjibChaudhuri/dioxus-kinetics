use dioxus::prelude::*;

/// Semantic input variants. Maps to the corresponding HTML `type` attribute so
/// browsers and assistive tech can apply the right keyboard, validation, and
/// autofill behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextFieldType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Search,
    Tel,
    Url,
}

impl TextFieldType {
    pub const fn as_html(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Number => "number",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
        }
    }
}

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
    #[props(default)] readonly: bool,
    #[props(default)] required: bool,
    #[props(default)] autocomplete: String,
    #[props(default)] input_type: TextFieldType,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
) -> Element {
    let described_by = described_by(&id, !help_text.is_empty(), !error_text.is_empty());
    let field_class = if invalid {
        "ui-field ui-text-field ui-field--invalid"
    } else {
        "ui-field ui-text-field"
    };
    let type_attr = input_type.as_html();
    let autocomplete_attr = if autocomplete.is_empty() {
        "off".to_string()
    } else {
        autocomplete.clone()
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
                    r#type: "{type_attr}",
                    value: "{value}",
                    placeholder: "{placeholder}",
                    disabled,
                    readonly,
                    required,
                    autocomplete: "{autocomplete_attr}",
                    "aria-invalid": if invalid { "true" } else { "false" },
                    "aria-required": if required { "true" } else { "false" },
                    "aria-describedby": "{described_by}",
                    oninput: move |evt| {
                        if let Some(handler) = &oninput {
                            handler.call(evt);
                        }
                    },
                    onchange: move |evt| {
                        if let Some(handler) = &onchange {
                            handler.call(evt);
                        }
                    },
                    onfocus: move |evt| {
                        if let Some(handler) = &onfocus {
                            handler.call(evt);
                        }
                    },
                    onblur: move |evt| {
                        if let Some(handler) = &onblur {
                            handler.call(evt);
                        }
                    },
                }
                if !trailing_text.is_empty() {
                    span { class: "ui-field-adornment ui-field-adornment--trailing", "{trailing_text}" }
                }
            }
            if !help_text.is_empty() {
                p { id: "{id}-help", class: "ui-field-help", "{help_text}" }
            }
            if !error_text.is_empty() {
                p { id: "{id}-error", class: "ui-field-error", role: "alert", "{error_text}" }
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
    onchange: Option<EventHandler<FormEvent>>,
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

    // `indeterminate` is a DOM property, not an HTML attribute. We have to sync
    // it imperatively after mount and whenever the value changes; otherwise the
    // native tri-state visual never appears.
    let sync_id = id.clone();
    use_effect(move || {
        let _ = checked; // re-run when checked toggles so we always reapply
        sync_indeterminate(&sync_id, indeterminate);
    });
    let mount_id = id.clone();

    rsx! {
        div { class: "{wrapper_class}",
            input {
                id: "{id}",
                class: "ui-checkbox-input",
                r#type: "checkbox",
                checked,
                disabled,
                "aria-checked": "{aria_checked}",
                onmounted: move |_evt| {
                    sync_indeterminate(&mount_id, indeterminate);
                },
                onchange: move |evt| {
                    if let Some(handler) = &onchange {
                        handler.call(evt);
                    }
                },
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

fn sync_indeterminate(id: &str, indeterminate: bool) {
    if id.is_empty() {
        return;
    }
    // Only forward simple identifier characters; the value is interpolated into
    // a script string. Reject anything that could break out of the literal.
    if !id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.')
    {
        return;
    }
    let value = if indeterminate { "true" } else { "false" };
    let _ = dioxus::document::eval(&format!(
        "const el = document.getElementById('{id}'); if (el) el.indeterminate = {value};"
    ));
}

#[component]
pub fn Switch(
    id: String,
    label: String,
    #[props(default)] description: String,
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    onchange: Option<EventHandler<bool>>,
) -> Element {
    let aria_checked = if checked { "true" } else { "false" };
    let aria_disabled = if disabled { "true" } else { "false" };
    let label_id = format!("{id}-label");
    let description_id = format!("{id}-description");
    let described_by = if description.is_empty() {
        String::new()
    } else {
        description_id.clone()
    };

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
                "aria-labelledby": "{label_id}",
                "aria-describedby": "{described_by}",
                onclick: move |_evt| {
                    if disabled {
                        return;
                    }
                    if let Some(handler) = &onchange {
                        handler.call(!checked);
                    }
                },
                span { class: "ui-switch-thumb" }
            }
            div { class: "ui-switch-copy",
                span {
                    id: "{label_id}",
                    class: "ui-switch-label",
                    role: "presentation",
                    onclick: move |_evt| {
                        if disabled {
                            return;
                        }
                        if let Some(handler) = &onchange {
                            handler.call(!checked);
                        }
                    },
                    "{label}"
                }
                if !description.is_empty() {
                    p { id: "{description_id}", class: "ui-switch-description", "{description}" }
                }
            }
        }
    }
}

/// One choice in a `RadioGroup`. `value` round-trips through
/// `on_change`; `label` is the visible text; `description` is an
/// optional subtitle; `disabled` greys out the row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub description: String,
    pub disabled: bool,
}

impl RadioOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: String::new(),
            disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Mutually-exclusive choice picker rendered as native
/// `<input type="radio">` elements sharing a `name`. Browsers handle
/// keyboard navigation and form submission; this component layers
/// label + description copy and selection state on top.
///
/// Different from `SegmentedControl`, which is a button-group with
/// `role="radiogroup"` — use that for short, equally-weighted choices,
/// and `RadioGroup` when each choice has descriptive copy or when the
/// host needs native radio semantics (e.g. `<form>` submission).
#[component]
pub fn RadioGroup(
    /// Stable id for the fieldset; per-option inputs are `{id}-{value}`.
    id: String,
    /// Group label rendered as the fieldset legend.
    label: String,
    /// HTML `name` shared by every radio input — required for the
    /// browser to enforce mutual exclusion.
    name: String,
    /// Currently-selected `value`. Empty string means nothing selected.
    #[props(default)]
    value: String,
    options: Vec<RadioOption>,
    #[props(default)] description: String,
    #[props(default)] disabled: bool,
    on_change: Option<EventHandler<String>>,
) -> Element {
    let description_id = format!("{id}-description");
    let described_by = if description.is_empty() {
        String::new()
    } else {
        description_id.clone()
    };

    rsx! {
        fieldset {
            class: "ui-radio-group",
            "aria-describedby": "{described_by}",
            disabled,
            legend { class: "ui-radio-group-legend", "{label}" }
            if !description.is_empty() {
                p {
                    id: "{description_id}",
                    class: "ui-radio-group-description",
                    "{description}"
                }
            }
            div { class: "ui-radio-group-list", role: "radiogroup",
                for option in options {
                    {
                        let option_value = option.value.clone();
                        let option_id = format!("{id}-{}", option.value);
                        let is_selected = option.value == value;
                        let row_class = if option.disabled {
                            "ui-radio ui-radio--disabled"
                        } else if is_selected {
                            "ui-radio ui-radio--selected"
                        } else {
                            "ui-radio"
                        };
                        let desc_id = format!("{option_id}-description");
                        let desc_target = if option.description.is_empty() {
                            String::new()
                        } else {
                            desc_id.clone()
                        };
                        let option_label = option.label.clone();
                        let option_description = option.description.clone();
                        let option_disabled = option.disabled;
                        rsx! {
                            div { class: "{row_class}",
                                input {
                                    id: "{option_id}",
                                    class: "ui-radio-input",
                                    r#type: "radio",
                                    name: "{name}",
                                    value: "{option_value}",
                                    checked: is_selected,
                                    disabled: option_disabled,
                                    "aria-describedby": "{desc_target}",
                                    onchange: move |_| {
                                        if option_disabled {
                                            return;
                                        }
                                        if let Some(handler) = &on_change {
                                            handler.call(option_value.clone());
                                        }
                                    },
                                }
                                div { class: "ui-radio-copy",
                                    label {
                                        class: "ui-radio-label",
                                        r#for: "{option_id}",
                                        "{option_label}"
                                    }
                                    if !option_description.is_empty() {
                                        p {
                                            id: "{desc_id}",
                                            class: "ui-radio-description",
                                            "{option_description}"
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
}

fn described_by(id: &str, has_help: bool, has_error: bool) -> String {
    match (has_help, has_error) {
        (true, true) => format!("{id}-help {id}-error"),
        (true, false) => format!("{id}-help"),
        (false, true) => format!("{id}-error"),
        (false, false) => String::new(),
    }
}

/// Continuous numeric input rendered as a native `<input type="range">`
/// so keyboard support (arrow keys, Page Up/Down, Home/End) and
/// touch/pointer drag both work out of the box. The native control is
/// styled via `.ui-slider`; the host stylesheet draws the track and
/// thumb tokens.
///
/// Provide `min`, `max`, and `step` (defaults: 0, 100, 1) to size the
/// range; `value` is the current numeric value; `value_text` is the
/// optional human-readable value announced to assistive tech via
/// `aria-valuetext` (use for non-numeric domains like "Small", "Medium",
/// "Large").
#[component]
pub fn Slider(
    id: String,
    label: String,
    value: f32,
    #[props(default = 0.0)] min: f32,
    #[props(default = 100.0)] max: f32,
    #[props(default = 1.0)] step: f32,
    #[props(default)] description: String,
    #[props(default)] value_text: String,
    #[props(default)] disabled: bool,
    onchange: Option<EventHandler<f32>>,
) -> Element {
    let described_by = if description.is_empty() {
        String::new()
    } else {
        format!("{id}-description")
    };
    let display_value_text = if value_text.is_empty() {
        format!("{value}")
    } else {
        value_text.clone()
    };

    rsx! {
        div { class: "ui-slider",
            label { class: "ui-slider-label", r#for: "{id}", "{label}" }
            input {
                id: "{id}",
                class: "ui-slider-input",
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                disabled,
                "aria-describedby": "{described_by}",
                "aria-valuetext": "{display_value_text}",
                oninput: move |evt| {
                    if disabled {
                        return;
                    }
                    if let Ok(parsed) = evt.value().parse::<f32>() {
                        if let Some(handler) = &onchange {
                            handler.call(parsed);
                        }
                    }
                },
            }
            if !description.is_empty() {
                p { id: "{id}-description", class: "ui-slider-description", "{description}" }
            }
        }
    }
}
