//! Authentication surfaces: `SignInCard` layout, `OAuthButton` provider
//! button, `PasswordStrengthMeter` (fed by the pure [`password_strength`]
//! scorer), and `MfaCodeInput`. All presentational; pair with `TextField`,
//! `Button`, and `Form` to assemble a real auth flow.

use dioxus::prelude::*;

/// OAuth identity provider. Drives the `OAuthButton` label and accent class.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OAuthProvider {
    #[default]
    Generic,
    Google,
    Github,
    Apple,
    Microsoft,
}

impl OAuthProvider {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Generic => "SSO",
            Self::Google => "Google",
            Self::Github => "GitHub",
            Self::Apple => "Apple",
            Self::Microsoft => "Microsoft",
        }
    }

    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Google => "google",
            Self::Github => "github",
            Self::Apple => "apple",
            Self::Microsoft => "microsoft",
        }
    }
}

/// Subjective strength of a password, scored by [`password_strength`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PasswordStrength {
    #[default]
    None,
    Weak,
    Fair,
    Good,
    Strong,
}

impl PasswordStrength {
    /// Number of filled bars (0..4) for the meter.
    pub const fn level(self) -> usize {
        match self {
            Self::None => 0,
            Self::Weak => 1,
            Self::Fair => 2,
            Self::Good => 3,
            Self::Strong => 4,
        }
    }

    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Weak => "weak",
            Self::Fair => "fair",
            Self::Good => "good",
            Self::Strong => "strong",
        }
    }
}

/// Score a password against simple heuristics (length, mixed case, digits,
/// symbols). Renderer-neutral so the meter can be SSR-driven and unit-tested.
pub fn password_strength(password: &str) -> PasswordStrength {
    if password.is_empty() {
        return PasswordStrength::None;
    }
    let mut score = 0;
    if password.len() >= 8 {
        score += 1;
    }
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_upper = password.chars().any(|c| c.is_uppercase());
    if has_lower && has_upper {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        score += 1;
    }
    if password.len() >= 12 {
        score += 1;
    }
    match score {
        0 | 1 => PasswordStrength::Weak,
        2 => PasswordStrength::Fair,
        3 => PasswordStrength::Good,
        _ => PasswordStrength::Strong,
    }
}

/// Layout card for a sign-in / sign-up form: a titled glass surface with a
/// body slot for fields and OAuth buttons.
#[component]
pub fn SignInCard(
    title: String,
    #[props(default)] description: String,
    children: Element,
) -> Element {
    rsx! {
        section { class: "ui-auth-card",
            header { class: "ui-auth-card-header",
                h2 { class: "ui-auth-card-title", "{title}" }
                if !description.is_empty() {
                    p { class: "ui-auth-card-description", "{description}" }
                }
            }
            div { class: "ui-auth-card-body", {children} }
        }
    }
}

/// Provider-styled social-auth button.
#[component]
pub fn OAuthButton(
    provider: OAuthProvider,
    #[props(default)] label: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let display_label = if label.is_empty() {
        format!("Continue with {}", provider.label())
    } else {
        label
    };
    let class = format!(
        "ui-oauth-button ui-oauth-button--{}",
        provider.class_suffix()
    );
    rsx! {
        button {
            class: "{class}",
            r#type: "button",
            "aria-label": "{display_label}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            "{display_label}"
        }
    }
}

/// Four-bar password strength meter driven by [`password_strength`]. The
/// live region announces the level for assistive tech.
#[component]
pub fn PasswordStrengthMeter(password: String, #[props(default)] show_label: bool) -> Element {
    let strength = password_strength(&password);
    let level = strength.level();
    let suffix = strength.class_suffix();
    let class = format!("ui-password-strength ui-password-strength--{suffix}");
    rsx! {
        div { class: "{class}",
            div { class: "ui-password-strength-bars", "aria-hidden": "true",
                for index in 0..4 {
                    span { class: if index < level { "ui-password-strength-bar ui-password-strength-bar--on" } else { "ui-password-strength-bar" } }
                }
            }
            if show_label {
                span { class: "ui-password-strength-label", "aria-live": "polite", "{suffix}" }
            }
        }
    }
}

/// N-digit one-time code input (default 6). Each digit is its own
/// single-character input; the full code round-trips through `on_change`.
#[component]
pub fn MfaCodeInput(
    #[props(default = 6)] length: usize,
    #[props(default)] value: String,
    #[props(default)] disabled: bool,
    on_change: Option<EventHandler<String>>,
) -> Element {
    let chars: Vec<String> = (0..length.max(1))
        .map(|i| {
            value
                .chars()
                .nth(i)
                .map(|c| c.to_string())
                .unwrap_or_default()
        })
        .collect();
    rsx! {
        fieldset { class: "ui-mfa-code", disabled,
            legend { class: "ui-mfa-code-legend", "One-time code" }
            div { class: "ui-mfa-code-cells",
                for (index, cell) in chars.iter().enumerate() {
                    input {
                        key: "{index}",
                        class: "ui-mfa-code-cell",
                        r#type: "text",
                        inputmode: "numeric",
                        maxlength: "1",
                        value: "{cell}",
                        "aria-label": "Digit {index + 1}",
                        disabled,
                        oninput: {
                            let current = value.clone();
                            move |evt: FormEvent| {
                                if let Some(handler) = &on_change {
                                    let digit = evt.value().chars().next().unwrap_or(' ');
                                    let mut next: Vec<char> = current.chars().collect();
                                    while next.len() <= index {
                                        next.push(' ');
                                    }
                                    next[index] = digit;
                                    let joined: String = next.into_iter().collect();
                                    handler.call(joined.trim_end().to_string());
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_password_is_none() {
        assert_eq!(password_strength(""), PasswordStrength::None);
    }

    #[test]
    fn short_lowercase_is_weak() {
        assert_eq!(password_strength("abc"), PasswordStrength::Weak);
    }

    #[test]
    fn mixed_case_and_digit_is_good() {
        assert_eq!(password_strength("Abcdefgh1"), PasswordStrength::Good);
    }

    #[test]
    fn long_mixed_with_symbol_is_strong() {
        assert_eq!(password_strength("Abcdefgh1!xyz"), PasswordStrength::Strong);
    }

    #[test]
    fn strength_levels_are_monotonic() {
        assert_eq!(PasswordStrength::None.level(), 0);
        assert_eq!(PasswordStrength::Weak.level(), 1);
        assert_eq!(PasswordStrength::Strong.level(), 4);
    }

    #[test]
    fn provider_labels_and_classes() {
        assert_eq!(OAuthProvider::Google.label(), "Google");
        assert_eq!(OAuthProvider::Github.class_suffix(), "github");
    }
}
