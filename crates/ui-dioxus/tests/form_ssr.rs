use dioxus::prelude::*;
use ui_dioxus::{
    validate, Button, EntryForm, FieldRules, Form, FormErrors, FormSchema, FormValues,
};

// ---------------------------------------------------------------------------
// Pure validation engine — renderer-neutral, no DOM required.
// ---------------------------------------------------------------------------

#[test]
fn required_rule_blocks_empty_and_passes_filled() {
    let schema =
        FormSchema::new().with_field("name", FieldRules::new().required("Name is required"));
    let empty = FormValues::new();
    let errs = validate(&schema, &empty);
    assert_eq!(errs.get("name"), Some("Name is required"));
    assert_eq!(errs.len(), 1);

    let mut filled = FormValues::new();
    filled.insert("name", "Ada");
    assert!(validate(&schema, &filled).is_empty());
}

#[test]
fn optional_field_skips_rules_when_empty() {
    let schema = FormSchema::new().with_field("bio", FieldRules::new().min_length(10, "Too short"));
    // No `required` → empty value passes.
    assert!(validate(&schema, &FormValues::new()).is_empty());

    let mut short = FormValues::new();
    short.insert("bio", "hi");
    assert_eq!(validate(&schema, &short).get("bio"), Some("Too short"));

    let mut ok = FormValues::new();
    ok.insert("bio", "long enough bio");
    assert!(validate(&schema, &ok).is_empty());
}

#[test]
fn length_and_numeric_bounds_are_enforced() {
    let schema = FormSchema::new()
        .with_field("code", FieldRules::new().max_length(4, "Max 4"))
        .with_field(
            "age",
            FieldRules::new()
                .min_value(18.0, "Min 18")
                .max_value(99.0, "Max 99"),
        );

    let mut data = FormValues::new();
    data.insert("code", "TOOLONG");
    data.insert("age", "12");
    let errs = validate(&schema, &data);
    assert_eq!(errs.get("code"), Some("Max 4"));
    assert_eq!(errs.get("age"), Some("Min 18"));

    let mut good = FormValues::new();
    good.insert("code", "AB");
    good.insert("age", "30");
    assert!(validate(&schema, &good).is_empty());
}

#[test]
fn email_rule_validates_format() {
    let schema = FormSchema::new().with_field("email", FieldRules::new().email("Bad email"));
    let mut bad = FormValues::new();
    bad.insert("email", "not-an-email");
    assert_eq!(validate(&schema, &bad).get("email"), Some("Bad email"));

    let mut ok = FormValues::new();
    ok.insert("email", "ada@example.com");
    assert!(validate(&schema, &ok).is_empty());
}

#[test]
fn matches_field_rule_compares_two_fields() {
    let schema = FormSchema::new()
        .with_field("password", FieldRules::new().required("Required"))
        .with_field(
            "confirm",
            FieldRules::new().matches("password", "Passwords differ"),
        );

    let mut same = FormValues::new();
    same.insert("password", "hunter2");
    same.insert("confirm", "hunter2");
    assert!(validate(&schema, &same).is_empty());

    let mut differ = FormValues::new();
    differ.insert("password", "hunter2");
    differ.insert("confirm", "hunter3");
    let errs = validate(&schema, &differ);
    assert_eq!(errs.get("confirm"), Some("Passwords differ"));
    assert!(errs.get("password").is_none());
}

#[test]
fn custom_rule_runs_after_builtins() {
    let schema = FormSchema::new().with_field(
        "coupon",
        FieldRules::new().custom(|v| v.starts_with("SAV"), "Coupon must start with SAV"),
    );

    let mut bad = FormValues::new();
    bad.insert("coupon", "X21");
    assert_eq!(
        validate(&schema, &bad).get("coupon"),
        Some("Coupon must start with SAV")
    );

    let mut ok = FormValues::new();
    ok.insert("coupon", "SAV21");
    assert!(validate(&schema, &ok).is_empty());
}

#[test]
fn only_the_first_failing_rule_is_reported() {
    let schema = FormSchema::new().with_field(
        "x",
        FieldRules::new()
            .required("required-msg")
            .min_length(5, "min-msg"),
    );
    let mut data = FormValues::new();
    data.insert("x", "ab"); // fails min_length, not required
    assert_eq!(validate(&schema, &data).get("x"), Some("min-msg"));
}

// ---------------------------------------------------------------------------
// Form component — SSR contract.
// ---------------------------------------------------------------------------

#[test]
fn form_renders_form_element_with_class() {
    let html = dioxus_ssr::render_element(rsx! {
        Form {
            Button { "Submit" }
        }
    });
    assert!(html.contains("<form"), "{html}");
    assert!(html.contains("ui-form"), "{html}");
    assert!(html.contains("Submit"), "{html}");
}

#[test]
fn form_passes_through_action_method_and_novalidate() {
    let html = dioxus_ssr::render_element(rsx! {
        Form {
            action: "/save".to_string(),
            method: "post".to_string(),
            Button { "Go" }
        }
    });
    assert!(html.contains(r#"action="/save""#), "{html}");
    assert!(html.contains(r#"method="post""#), "{html}");
    assert!(html.contains("novalidate"), "{html}");
}

#[test]
fn form_renders_accessible_error_summary_when_errors_present() {
    let mut errors = FormErrors::new();
    errors.insert("email", "Invalid email");
    errors.insert("name", "Required");

    let html = dioxus_ssr::render_element(rsx! {
        Form {
            errors: Some(errors),
            Button { "Submit" }
        }
    });
    assert!(html.contains("ui-form-summary"), "{html}");
    assert!(html.contains(r#"role="alert""#), "{html}");
    assert!(html.contains("Invalid email"), "{html}");
    assert!(html.contains("Required"), "{html}");
    assert!(html.contains(r#"data-field="email""#), "{html}");
}

#[test]
fn form_omits_summary_when_there_are_no_errors() {
    let html = dioxus_ssr::render_element(rsx! {
        Form {
            errors: Some(FormErrors::new()),
            Button { "Submit" }
        }
    });
    assert!(!html.contains("ui-form-summary"), "{html}");
}

#[test]
fn entry_form_alias_renders_identically() {
    let html = dioxus_ssr::render_element(rsx! {
        EntryForm {
            Button { "Send" }
        }
    });
    assert!(html.contains("<form"), "{html}");
    assert!(html.contains("ui-form"), "{html}");
}
