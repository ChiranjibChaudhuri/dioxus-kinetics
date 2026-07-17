//! Form orchestration: a renderer-neutral validation engine plus a `<form>`
//! wrapper that surfaces a shared error summary and exposes field errors
//! through context for opt-in field binding.
//!
//! The engine (`FormSchema` / `FieldRules` / `validate`) is pure and needs no
//! DOM — it is unit-tested directly and mirrors the co-location pattern used
//! by `sortable` (`apply_kanban_move` + `SortableList`).

use std::collections::HashMap;

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Pure validation engine.
// ---------------------------------------------------------------------------

/// Collected named form values keyed by field name. The pure engine reads
/// this; the `Form` component builds it from a Dioxus `FormEvent` at submit.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormValues {
    values: HashMap<String, String>,
}

impl FormValues {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.values.insert(name.into(), value.into());
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

/// Field name -> error message. Returned by [`validate`]; consumed by the
/// `Form` summary and the `use_form_error` context hook.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormErrors {
    errors: HashMap<String, String>,
}

impl FormErrors {
    pub fn new() -> Self {
        Self {
            errors: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, message: impl Into<String>) {
        self.errors.insert(name.into(), message.into());
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.errors.get(name).map(String::as_str)
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.errors.iter()
    }
}

/// Declarative validation rules for a single field. Built with the chained
/// builder; the first failing rule wins (see [`FieldRules::first_error`]).
pub struct FieldRules {
    required: Option<String>,
    min_length: Option<(usize, String)>,
    max_length: Option<(usize, String)>,
    min_value: Option<(f64, String)>,
    max_value: Option<(f64, String)>,
    email: Option<String>,
    matches: Option<(String, String)>,
    custom: Option<(Box<dyn Fn(&str) -> bool + Send + Sync>, String)>,
}

impl Default for FieldRules {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldRules {
    pub fn new() -> Self {
        Self {
            required: None,
            min_length: None,
            max_length: None,
            min_value: None,
            max_value: None,
            email: None,
            matches: None,
            custom: None,
        }
    }

    /// Block empty values. Applied before every other rule.
    pub fn required(mut self, message: impl Into<String>) -> Self {
        self.required = Some(message.into());
        self
    }

    /// Minimum character count. Skipped when the value is empty and the
    /// field is not `required`.
    pub fn min_length(mut self, n: usize, message: impl Into<String>) -> Self {
        self.min_length = Some((n, message.into()));
        self
    }

    /// Maximum character count.
    pub fn max_length(mut self, n: usize, message: impl Into<String>) -> Self {
        self.max_length = Some((n, message.into()));
        self
    }

    /// Inclusive numeric lower bound. Non-numeric values fail.
    pub fn min_value(mut self, n: f64, message: impl Into<String>) -> Self {
        self.min_value = Some((n, message.into()));
        self
    }

    /// Inclusive numeric upper bound. Non-numeric values fail.
    pub fn max_value(mut self, n: f64, message: impl Into<String>) -> Self {
        self.max_value = Some((n, message.into()));
        self
    }

    /// Require a plausible email format (single `@`, a `.` in the domain,
    /// non-empty local part). No regex dependency.
    pub fn email(mut self, message: impl Into<String>) -> Self {
        self.email = Some(message.into());
        self
    }

    /// Require equality with another field's value (e.g. password confirm).
    pub fn matches(mut self, other_field: impl Into<String>, message: impl Into<String>) -> Self {
        self.matches = Some((other_field.into(), message.into()));
        self
    }

    /// Custom predicate — returns `true` when the value is valid.
    pub fn custom(
        mut self,
        check: impl Fn(&str) -> bool + Send + Sync + 'static,
        message: impl Into<String>,
    ) -> Self {
        self.custom = Some((Box::new(check), message.into()));
        self
    }

    /// First failing rule's message, or `None` when valid. `data` is passed so
    /// cross-field rules (`matches`) can read sibling values.
    fn first_error(&self, value: &str, data: &FormValues) -> Option<String> {
        if value.is_empty() {
            // Empty + required → fail; otherwise optional fields pass here.
            return self.required.clone();
        }
        if let Some((n, msg)) = &self.min_length {
            if value.chars().count() < *n {
                return Some(msg.clone());
            }
        }
        if let Some((n, msg)) = &self.max_length {
            if value.chars().count() > *n {
                return Some(msg.clone());
            }
        }
        if let Some((min, msg)) = &self.min_value {
            match value.parse::<f64>() {
                Ok(v) if v < *min => return Some(msg.clone()),
                Err(_) => return Some(msg.clone()),
                _ => {}
            }
        }
        if let Some((max, msg)) = &self.max_value {
            match value.parse::<f64>() {
                Ok(v) if v > *max => return Some(msg.clone()),
                Err(_) => return Some(msg.clone()),
                _ => {}
            }
        }
        if let Some(msg) = &self.email {
            if !is_plausible_email(value) {
                return Some(msg.clone());
            }
        }
        if let Some((other, msg)) = &self.matches {
            if data.get(other).unwrap_or("") != value {
                return Some(msg.clone());
            }
        }
        if let Some((check, msg)) = &self.custom {
            if !check(value) {
                return Some(msg.clone());
            }
        }
        None
    }
}

fn is_plausible_email(value: &str) -> bool {
    if value.matches('@').count() != 1 {
        return false;
    }
    let (local, domain) = value.split_once('@').unwrap();
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

/// A registered set of field rules, built with [`FormSchema::with_field`].
pub struct FormSchema {
    fields: Vec<(String, FieldRules)>,
}

impl Default for FormSchema {
    fn default() -> Self {
        Self { fields: Vec::new() }
    }
}

impl FormSchema {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn with_field(mut self, name: impl Into<String>, rules: FieldRules) -> Self {
        self.fields.push((name.into(), rules));
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &FieldRules)> {
        self.fields.iter().map(|(name, rules)| (name, rules))
    }
}

/// Run every field's rules and collect the first error per failing field.
/// Empty input with no `required` fields yields no errors.
pub fn validate(schema: &FormSchema, data: &FormValues) -> FormErrors {
    let mut errors = FormErrors::new();
    for (name, rules) in schema.iter() {
        let value = data.get(name).unwrap_or("");
        if let Some(message) = rules.first_error(value, data) {
            errors.insert(name.clone(), message);
        }
    }
    errors
}

// ---------------------------------------------------------------------------
// Form component.
// ---------------------------------------------------------------------------

type ErrorsSignal = Signal<Option<FormErrors>>;

/// Look up the error message (if any) for `field` from the nearest `Form`
/// ancestor. Returns `None` when no `Form` is in scope or the field is valid.
/// Call this unconditionally at the top of a child component — it is a hook.
pub fn use_form_error(field: &str) -> Option<String> {
    let signal: ErrorsSignal = try_use_context()?;
    let guard = signal.read();
    guard
        .as_ref()
        .and_then(|errors| errors.get(field).map(str::to_owned))
}

fn collect_form_values(evt: &FormEvent) -> FormValues {
    let mut out = FormValues::new();
    for (name, value) in evt.values() {
        if let dioxus::events::FormValue::Text(text) = value {
            out.insert(name, text);
        }
    }
    out
}

/// Semantic `<form>` wrapper. Prevents the default browser submit, forwards
/// collected named values via `on_submit`, renders an accessible error
/// summary when `errors` is non-empty, and publishes field errors through
/// context (see [`use_form_error`]).
///
/// Pair with the existing field components (`TextField`, `Checkbox`, …) by
/// giving each control a `name`; on submit the wrapper hands you a
/// [`FormValues`] you can feed straight to [`validate`].
#[component]
pub fn Form(
    #[props(default)] action: String,
    #[props(default)] method: String,
    #[props(default)] disabled: bool,
    /// Off by default — the engine authoritatively validates via `on_submit`.
    #[props(default = true)]
    novalidate: bool,
    #[props(default)] errors: Option<FormErrors>,
    on_submit: Option<EventHandler<FormValues>>,
    on_reset: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let mut errors_signal = use_context_provider(|| Signal::new(None::<FormErrors>));
    errors_signal.set(errors.clone());

    let class = if disabled {
        "ui-form ui-form--disabled".to_string()
    } else {
        "ui-form".to_string()
    };
    let aria_disabled = if disabled { "true" } else { "false" };

    rsx! {
        form {
            class: "{class}",
            action: "{action}",
            method: "{method}",
            "aria-disabled": "{aria_disabled}",
            novalidate,
            onsubmit: move |evt: FormEvent| {
                evt.prevent_default();
                if let Some(handler) = &on_submit {
                    handler.call(collect_form_values(&evt));
                }
            },
            onreset: move |_evt| {
                if let Some(handler) = &on_reset {
                    handler.call(());
                }
            },
            if let Some(errs) = errors.as_ref() {
                if !errs.is_empty() {
                    ul {
                        class: "ui-form-summary",
                        role: "alert",
                        "aria-label": "Form errors",
                        for (field, message) in errs.iter() {
                            li {
                                class: "ui-form-summary-item",
                                "data-field": "{field}",
                                "{message}"
                            }
                        }
                    }
                }
            }
            {children}
        }
    }
}
