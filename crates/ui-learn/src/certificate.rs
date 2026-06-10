use dioxus::prelude::*;

/// A completion certificate laid out for export: a fixed 5:3.5 aspect,
/// ornamental rule work drawn with CSS (no image assets), and serif display
/// type. Render it inside a `CaptureStage` and export through
/// kinetics-render for a shareable PNG, or print it directly.
///
/// Everything is a plain prop, so the same component serves live profile
/// pages and batch certificate generation.
#[component]
pub fn CertificateCard(
    recipient: String,
    course: String,
    /// Preformatted completion date ("12 June 2026") — host-formatted so
    /// locale and capture output stay deterministic.
    date: String,
    issuer: String,
    #[props(default)] signature_name: String,
    #[props(default = "Certificate of Completion".to_string())] heading: String,
    #[props(default)] credential_id: String,
) -> Element {
    rsx! {
        article {
            class: "ui-certificate",
            role: "img",
            "aria-label": "{heading}: {recipient}, {course}, {date}, issued by {issuer}",
            div { class: "ui-certificate-frame", "aria-hidden": "true" }
            div { class: "ui-certificate-body",
                p { class: "ui-certificate-issuer", "{issuer}" }
                h3 { class: "ui-certificate-heading", "{heading}" }
                p { class: "ui-certificate-presented", "This certifies that" }
                p { class: "ui-certificate-recipient", "{recipient}" }
                p { class: "ui-certificate-presented", "has successfully completed" }
                p { class: "ui-certificate-course", "{course}" }
                div { class: "ui-certificate-footer",
                    div { class: "ui-certificate-signature",
                        if !signature_name.is_empty() {
                            span { class: "ui-certificate-signature-name", "{signature_name}" }
                        }
                        span { class: "ui-certificate-signature-rule" }
                        span { class: "ui-certificate-signature-label", "Instructor" }
                    }
                    div { class: "ui-certificate-seal", "aria-hidden": "true",
                        svg { view_box: "0 0 48 48",
                            circle {
                                cx: "24", cy: "24", r: "20",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                            }
                            circle {
                                cx: "24", cy: "24", r: "15",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1",
                                stroke_dasharray: "3 2",
                            }
                            path {
                                d: "M17 24.5 22 29.5 31.5 19",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                            }
                        }
                    }
                    div { class: "ui-certificate-date",
                        span { class: "ui-certificate-date-value", "{date}" }
                        span { class: "ui-certificate-signature-rule" }
                        span { class: "ui-certificate-signature-label", "Date" }
                    }
                }
                if !credential_id.is_empty() {
                    p { class: "ui-certificate-credential ui-tabular", "Credential {credential_id}" }
                }
            }
        }
    }
}
