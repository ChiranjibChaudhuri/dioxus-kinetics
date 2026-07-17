use dioxus::prelude::*;
use ui_dioxus::{
    password_strength, AuthCard, CodeInput, Invoice, InvoiceList, InvoiceStatus, MfaCodeInput,
    OAuthButton, OAuthProvider, PasswordStrength, PasswordStrengthMeter, PlanCard, PricingPlan,
    PricingTable, SignInCard, StrengthMeter, TextField, UsageMeter,
};

#[test]
fn sign_in_card_renders_title_and_body() {
    let html = dioxus_ssr::render_element(rsx! {
        SignInCard {
            title: "Welcome back".to_string(),
            description: "Sign in to continue".to_string(),
            TextField { id: "email".to_string(), label: "Email".to_string() }
        }
    });
    assert!(html.contains("ui-auth-card"), "{html}");
    assert!(html.contains("Welcome back"), "{html}");
    assert!(html.contains("Sign in to continue"), "{html}");
    assert!(html.contains("Email"), "{html}");
}

#[test]
fn auth_card_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! {
        AuthCard { title: "T".to_string(), TextField { id: "e".to_string(), label: "E".to_string() } }
    });
    let b = dioxus_ssr::render_element(rsx! {
        SignInCard { title: "T".to_string(), TextField { id: "e".to_string(), label: "E".to_string() } }
    });
    assert_eq!(a, b);
}

#[test]
fn oauth_button_renders_provider_label() {
    let html = dioxus_ssr::render_element(rsx! {
        OAuthButton { provider: OAuthProvider::Github }
    });
    assert!(html.contains("ui-oauth-button--github"), "{html}");
    assert!(html.contains("Continue with GitHub"), "{html}");
}

#[test]
fn password_strength_meter_shows_bars_and_label() {
    let html = dioxus_ssr::render_element(rsx! {
        PasswordStrengthMeter { password: "Abcdefgh1!xy".to_string(), show_label: true }
    });
    assert!(html.contains("ui-password-strength--strong"), "{html}");
    assert!(html.contains("ui-password-strength-bar--on"), "{html}");
    assert!(html.contains("strong"), "{html}");
}

#[test]
fn strength_meter_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! {
        StrengthMeter { password: "x".to_string() }
    });
    let b = dioxus_ssr::render_element(rsx! {
        PasswordStrengthMeter { password: "x".to_string() }
    });
    assert_eq!(a, b);
}

#[test]
fn password_strength_levels() {
    assert_eq!(password_strength(""), PasswordStrength::None);
    assert_eq!(password_strength("Abcdefgh1!xy"), PasswordStrength::Strong);
}

#[test]
fn mfa_code_input_renders_six_cells() {
    let html = dioxus_ssr::render_element(rsx! {
        MfaCodeInput { value: "12".to_string() }
    });
    assert!(html.contains("ui-mfa-code"), "{html}");
    // six single-char inputs
    assert_eq!(html.matches(r#"maxlength="1""#).count(), 6, "{html}");
    assert!(html.contains(r#"aria-label="Digit 1""#), "{html}");
}

#[test]
fn code_input_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! { CodeInput {} });
    let b = dioxus_ssr::render_element(rsx! { MfaCodeInput {} });
    assert_eq!(a, b);
}

#[test]
fn plan_card_renders_featured_and_features() {
    let plan = PricingPlan::new("Pro", "$29")
        .per("month")
        .feature("Unlimited seats")
        .featured();
    let html = dioxus_ssr::render_element(rsx! { PlanCard { plan } });
    assert!(html.contains("ui-plan-card--featured"), "{html}");
    assert!(html.contains("$29"), "{html}");
    assert!(html.contains("/month"), "{html}");
    assert!(html.contains("Unlimited seats"), "{html}");
    assert!(html.contains("Choose plan"), "{html}");
}

#[test]
fn pricing_table_renders_each_plan() {
    let plans = vec![
        PricingPlan::new("Starter", "$0"),
        PricingPlan::new("Pro", "$29").featured(),
    ];
    let html = dioxus_ssr::render_element(rsx! { PricingTable { plans } });
    assert!(html.contains("ui-pricing-table"), "{html}");
    assert!(html.contains("Starter"), "{html}");
    assert!(html.contains("Pro"), "{html}");
}

#[test]
fn usage_meter_renders_tone_and_progress() {
    let html = dioxus_ssr::render_element(rsx! {
        UsageMeter { label: "Seats".to_string(), used: 9.0, limit: 10.0, unit: "".to_string() }
    });
    assert!(html.contains("ui-usage-meter--critical"), "{html}");
    assert!(html.contains(r#"role="progressbar""#), "{html}");
    assert!(html.contains("width:90%"), "{html}");
}

#[test]
fn invoice_list_renders_status_badges() {
    let invoices = vec![
        Invoice::new("INV-1", "2026-01-01", "$29.00", InvoiceStatus::Paid),
        Invoice::new("INV-2", "2026-02-01", "$29.00", InvoiceStatus::Overdue),
    ];
    let html = dioxus_ssr::render_element(rsx! { InvoiceList { invoices } });
    assert!(html.contains("ui-invoice-list"), "{html}");
    assert!(html.contains("ui-invoice-status--paid"), "{html}");
    assert!(html.contains("ui-invoice-status--overdue"), "{html}");
    assert!(html.contains("INV-1"), "{html}");
    assert!(html.contains("Overdue"), "{html}");
}
