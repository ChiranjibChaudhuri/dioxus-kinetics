use dioxus::prelude::*;
use kinetics::prelude::*;

// ---------------------------------------------------------------------------
// Liquid Glass (Apple-style)
// ---------------------------------------------------------------------------

pub fn liquid_glass_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "LiquidGlass \u{2014} neutral" }
                LiquidGlass {
                    div { style: "padding:28px 32px; font-size:17px; font-weight:600;",
                        "A thick, edge-lit glass surface."
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Forms
// ---------------------------------------------------------------------------

pub fn form_preview() -> Element {
    let mut errors = FormErrors::new();
    errors.insert("email", "Enter a valid email address.");
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "Form with validation summary" }
            Form {
                errors: Some(errors),
                Stack {
                    gap: "md".to_string(),
                    TextField { id: "email".to_string(), label: "Email".to_string(), placeholder: "you@example.com".to_string() }
                    TextField { id: "name".to_string(), label: "Full name".to_string() }
                    Button { variant: ButtonVariant::Primary, "Create account" }
                }
            }
        }
    }
}

pub fn tag_input_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "TagInput / ChipInput" }
            TagInput {
                id: "skills".to_string(),
                label: "Skills".to_string(),
                tags: vec!["Rust".to_string(), "Dioxus".to_string(), "WGSL".to_string()],
                placeholder: "Add a skill".to_string(),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AI-native surfaces
// ---------------------------------------------------------------------------

pub fn answer_panel_preview() -> Element {
    let sources = vec![
        AnswerSource::new("Dioxus Kinetics docs", "dioxus-kinetics.dev")
            .snippet("A Dioxus-first UI workspace.")
            .href("https://example.com/dk"),
        AnswerSource::new("GitHub", "github.com"),
        AnswerSource::new("RustConf talk", "rustconf.com"),
    ];
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "AnswerPanel \u{2014} Perplexity-style answer" }
            AnswerPanel {
                query: "What is Dioxus Kinetics?".to_string(),
                answer: "A Dioxus-first UI workspace with cinematic motion, liquid glass, and frame-accurate video export \u{2014} all from one Rust facade.".to_string(),
                sources,
                related: vec!["How do I render a scene?".to_string(), "What is liquid glass?".to_string()],
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Charts (the new families)
// ---------------------------------------------------------------------------

pub fn area_chart_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "AreaChart \u{2014} multi-series trend" }
            AreaChart {
                label: "Revenue by week".to_string(),
                series: vec![
                    ChartSeries::new("This year", vec![12.0, 18.0, 15.0, 22.0, 28.0, 26.0, 34.0]),
                    ChartSeries::new("Last year", vec![8.0, 11.0, 10.0, 14.0, 16.0, 19.0, 21.0]),
                ],
                x_labels: vec!["W1".to_string(), "W2".to_string(), "W3".to_string(), "W4".to_string(), "W5".to_string(), "W6".to_string(), "W7".to_string()],
            }
        }
    }
}

pub fn funnel_chart_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "FunnelChart \u{2014} conversion" }
            FunnelChart {
                label: "Signup funnel".to_string(),
                stages: vec![
                    FunnelStage::new("Visited", 1000.0),
                    FunnelStage::new("Signed up", 420.0),
                    FunnelStage::new("Activated", 210.0),
                    FunnelStage::new("Paid", 90.0),
                ],
            }
        }
    }
}

pub fn gauge_chart_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "GaugeChart \u{2014} success" }
                GaugeChart { label: "CPU load".to_string(), value: 0.62, tone: ChartTone::Success }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "GaugeChart \u{2014} warning" }
                GaugeChart { label: "Memory".to_string(), value: 0.85, tone: ChartTone::Warning, display_value: "85%".to_string() }
            }
        }
    }
}

pub fn heatmap_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "Heatmap \u{2014} activity grid" }
            Heatmap {
                label: "Engagement by hour".to_string(),
                rows: vec![
                    HeatmapRow::new("Mon", vec![1.0, 4.0, 9.0, 7.0, 3.0]),
                    HeatmapRow::new("Tue", vec![2.0, 5.0, 8.0, 6.0, 4.0]),
                    HeatmapRow::new("Wed", vec![3.0, 6.0, 10.0, 9.0, 5.0]),
                ],
                column_labels: vec!["09".to_string(), "12".to_string(), "15".to_string(), "18".to_string(), "21".to_string()],
            }
        }
    }
}

pub fn treemap_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "Treemap \u{2014} budget share" }
            Treemap {
                label: "Quarterly budget".to_string(),
                items: vec![
                    TreemapItem::new("Engineering", 60.0),
                    TreemapItem::new("Sales", 25.0),
                    TreemapItem::new("Operations", 10.0),
                    TreemapItem::new("Other", 5.0),
                ],
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------

pub fn sign_in_card_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "SignInCard" }
            SignInCard {
                title: "Welcome back".to_string(),
                description: "Sign in to your workspace".to_string(),
                Stack {
                    gap: "md".to_string(),
                    TextField { id: "si-email".to_string(), label: "Email".to_string(), placeholder: "you@example.com".to_string() }
                    TextField { id: "si-pw".to_string(), label: "Password".to_string(), input_type: TextFieldType::Password }
                    Button { variant: ButtonVariant::Primary, "Sign in" }
                    OAuthButton { provider: OAuthProvider::Github }
                }
            }
        }
    }
}

pub fn password_strength_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Weak" }
                PasswordStrengthMeter { password: "abc".to_string(), show_label: true }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Good" }
                PasswordStrengthMeter { password: "Abcdefgh1".to_string(), show_label: true }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Strong" }
                PasswordStrengthMeter { password: "Abcdefgh1!xy".to_string(), show_label: true }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Billing
// ---------------------------------------------------------------------------

pub fn pricing_table_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile",
            span { class: "gallery-variant-label", "PricingTable" }
            PricingTable {
                plans: vec![
                    PricingPlan::new("Starter", "$0").per("month").feature("3 projects").feature("Community support"),
                    PricingPlan::new("Pro", "$29").per("month").feature("Unlimited projects").feature("Priority support").featured(),
                    PricingPlan::new("Team", "$99").per("month").feature("SSO").feature("Audit log"),
                ],
            }
        }
    }
}

pub fn usage_meter_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Normal" }
                UsageMeter { label: "Seats".to_string(), used: 4.0, limit: 10.0 }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Warning" }
                UsageMeter { label: "Storage".to_string(), used: 72.0, limit: 100.0, unit: " GB".to_string() }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Critical" }
                UsageMeter { label: "API calls".to_string(), used: 9600.0, limit: 10000.0 }
            }
        }
    }
}
