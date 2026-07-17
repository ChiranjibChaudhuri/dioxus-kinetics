//! Billing surfaces: `PricingTable`/`PlanCard`, `UsageMeter`, and
//! `InvoiceList`. Presentational components driven by plain value types;
//! `usage_fraction` and the tone thresholds are renderer-neutral so they can
//! be SSR-driven and unit-tested.

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Pricing
// ---------------------------------------------------------------------------

/// One subscription plan rendered by [`PlanCard`] and listed in
/// [`PricingTable`].
#[derive(Clone, Debug, PartialEq)]
pub struct PricingPlan {
    pub name: String,
    pub price: String,
    pub period: String,
    pub features: Vec<String>,
    pub featured: bool,
    pub cta_label: String,
}

impl PricingPlan {
    pub fn new(name: impl Into<String>, price: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            price: price.into(),
            period: String::new(),
            features: Vec::new(),
            featured: false,
            cta_label: "Choose plan".to_string(),
        }
    }

    pub fn per(mut self, period: impl Into<String>) -> Self {
        self.period = period.into();
        self
    }

    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    pub fn featured(mut self) -> Self {
        self.featured = true;
        self
    }

    pub fn cta(mut self, label: impl Into<String>) -> Self {
        self.cta_label = label.into();
        self
    }
}

/// A single plan card: name, price, period, feature list, and a CTA button.
#[component]
pub fn PlanCard(
    plan: PricingPlan,
    #[props(default)] cta_variant: PlanCtaVariant,
    on_select: Option<EventHandler<String>>,
) -> Element {
    let class = if plan.featured {
        "ui-plan-card ui-plan-card--featured"
    } else {
        "ui-plan-card"
    };
    let cta_class = match cta_variant {
        PlanCtaVariant::Primary => "ui-plan-card-cta ui-plan-card-cta--primary",
        PlanCtaVariant::Ghost => "ui-plan-card-cta ui-plan-card-cta--ghost",
    };
    let plan_name = plan.name.clone();
    rsx! {
        article { class: "{class}",
            header { class: "ui-plan-card-header",
                h3 { class: "ui-plan-card-name", "{plan.name}" }
                div { class: "ui-plan-card-price-row",
                    span { class: "ui-plan-card-price", "{plan.price}" }
                    if !plan.period.is_empty() {
                        span { class: "ui-plan-card-period", "/{plan.period}" }
                    }
                }
            }
            if !plan.features.is_empty() {
                ul { class: "ui-plan-card-features",
                    for feature in plan.features.iter() {
                        li { class: "ui-plan-card-feature", "{feature}" }
                    }
                }
            }
            button {
                class: "{cta_class}",
                r#type: "button",
                "aria-label": "{plan.cta_label} {plan_name}",
                onclick: move |_evt| {
                    if let Some(handler) = &on_select {
                        handler.call(plan_name.clone());
                    }
                },
                "{plan.cta_label}"
            }
        }
    }
}

/// Visual treatment of a plan's call-to-action button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlanCtaVariant {
    #[default]
    Primary,
    Ghost,
}

/// A row of [`PlanCard`]s. Emits the selected plan name via `on_select`.
#[component]
pub fn PricingTable(plans: Vec<PricingPlan>, on_select: Option<EventHandler<String>>) -> Element {
    rsx! {
        div { class: "ui-pricing-table",
            for plan in plans.iter() {
                PlanCard {
                    key: "{plan.name}",
                    plan: plan.clone(),
                    cta_variant: if plan.featured { PlanCtaVariant::Primary } else { PlanCtaVariant::Ghost },
                    on_select,
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Usage meter
// ---------------------------------------------------------------------------

/// `used / limit` clamped to `[0, 1]`; `None` when the limit is non-positive.
pub fn usage_fraction(used: f32, limit: f32) -> Option<f32> {
    if limit <= 0.0 || !used.is_finite() || !limit.is_finite() {
        return None;
    }
    Some((used / limit).clamp(0.0, 1.0))
}

/// Tone for a usage meter, escalating as the fraction approaches/exceeds 1.
pub fn usage_tone(fraction: f32) -> UsageTone {
    let f = if fraction.is_finite() {
        fraction.clamp(0.0, 1.0)
    } else {
        0.0
    };
    if f >= 0.9 {
        UsageTone::Critical
    } else if f >= 0.7 {
        UsageTone::Warning
    } else {
        UsageTone::Normal
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UsageTone {
    #[default]
    Normal,
    Warning,
    Critical,
}

impl UsageTone {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }
}

/// A labelled usage bar (`used` of `limit`). Tone escalates near the cap and
/// the live region announces the percentage.
#[component]
pub fn UsageMeter(label: String, used: f32, limit: f32, #[props(default)] unit: String) -> Element {
    let fraction = usage_fraction(used, limit).unwrap_or(0.0);
    let tone = usage_tone(fraction);
    let percent = (fraction * 100.0).round() as u32;
    let bar_style = format!("width:{percent}%");
    let class = format!("ui-usage-meter ui-usage-meter--{}", tone.class_suffix());
    let readout = format!("{used}{unit} / {limit}{unit}");
    rsx! {
        div { class: "{class}",
            div { class: "ui-usage-meter-head",
                span { class: "ui-usage-meter-label", "{label}" }
                span { class: "ui-usage-meter-readout", "aria-live": "polite", "{readout} ({percent}%)" }
            }
            div { class: "ui-usage-meter-track", role: "progressbar",
                "aria-valuemin": "0", "aria-valuemax": "100", "aria-valuenow": "{percent}",
                "aria-label": "{label}",
                div { class: "ui-usage-meter-bar", style: "{bar_style}" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Invoices
// ---------------------------------------------------------------------------

/// Lifecycle state of an invoice; drives the row's tone badge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InvoiceStatus {
    #[default]
    Draft,
    Paid,
    Due,
    Overdue,
}

impl InvoiceStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Paid => "Paid",
            Self::Due => "Due",
            Self::Overdue => "Overdue",
        }
    }

    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Paid => "paid",
            Self::Due => "due",
            Self::Overdue => "overdue",
        }
    }
}

/// One invoice row.
#[derive(Clone, Debug, PartialEq)]
pub struct Invoice {
    pub id: String,
    pub date: String,
    pub amount: String,
    pub status: InvoiceStatus,
}

impl Invoice {
    pub fn new(
        id: impl Into<String>,
        date: impl Into<String>,
        amount: impl Into<String>,
        status: InvoiceStatus,
    ) -> Self {
        Self {
            id: id.into(),
            date: date.into(),
            amount: amount.into(),
            status,
        }
    }
}

/// A compact list of invoices with a status badge per row.
#[component]
pub fn InvoiceList(invoices: Vec<Invoice>) -> Element {
    rsx! {
        ul { class: "ui-invoice-list",
            for invoice in invoices.iter() {
                li { class: "ui-invoice-row", key: "{invoice.id}",
                    span { class: "ui-invoice-id", "{invoice.id}" }
                    span { class: "ui-invoice-date", "{invoice.date}" }
                    span { class: "ui-invoice-amount", "{invoice.amount}" }
                    span { class: "ui-invoice-status ui-invoice-status--{invoice.status.class_suffix()}", "{invoice.status.label()}" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pricing_plan_builder_chains() {
        let plan = PricingPlan::new("Pro", "$29")
            .per("month")
            .feature("Unlimited seats")
            .featured()
            .cta("Start free trial");
        assert_eq!(plan.period, "month");
        assert_eq!(plan.features, vec!["Unlimited seats".to_string()]);
        assert!(plan.featured);
        assert_eq!(plan.cta_label, "Start free trial");
    }

    #[test]
    fn usage_fraction_clamps_and_rejects_bad_limit() {
        assert_eq!(usage_fraction(5.0, 10.0), Some(0.5));
        assert_eq!(usage_fraction(15.0, 10.0), Some(1.0));
        assert_eq!(usage_fraction(1.0, 0.0), None);
    }

    #[test]
    fn usage_tone_escalates() {
        assert_eq!(usage_tone(0.5), UsageTone::Normal);
        assert_eq!(usage_tone(0.75), UsageTone::Warning);
        assert_eq!(usage_tone(0.95), UsageTone::Critical);
    }

    #[test]
    fn invoice_status_labels_and_classes() {
        assert_eq!(InvoiceStatus::Paid.label(), "Paid");
        assert_eq!(InvoiceStatus::Overdue.class_suffix(), "overdue");
    }
}
