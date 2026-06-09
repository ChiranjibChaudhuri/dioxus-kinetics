//! WCAG AA contrast guards for the accent/on-color token pairs that render as
//! text. These mirror the real CSS values declared in `base_css()` and the
//! self-tint math used by `.ui-badge--*` and `.ui-citation-chip`, so a future
//! palette edit that drops a foreground below AA 4.5:1 fails the build instead
//! of shipping an inaccessible surface.

use ui_styles::base_css;

type Rgb = (u8, u8, u8);

const WHITE: Rgb = (255, 255, 255);

/// Parse a `#rrggbb` literal into an sRGB triple.
fn hex(s: &str) -> Rgb {
    let s = s.trim_start_matches('#');
    let n = u32::from_str_radix(s, 16).expect("valid 6-digit hex");
    (
        ((n >> 16) & 0xff) as u8,
        ((n >> 8) & 0xff) as u8,
        (n & 0xff) as u8,
    )
}

/// WCAG relative luminance for an opaque sRGB color.
fn relative_luminance((r, g, b): Rgb) -> f64 {
    fn linearize(channel: u8) -> f64 {
        let c = channel as f64 / 255.0;
        if c <= 0.040_45 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * linearize(r) + 0.7152 * linearize(g) + 0.0722 * linearize(b)
}

/// WCAG contrast ratio between two opaque sRGB colors.
fn contrast_ratio(a: Rgb, b: Rgb) -> f64 {
    let (mut hi, mut lo) = (relative_luminance(a), relative_luminance(b));
    if hi < lo {
        std::mem::swap(&mut hi, &mut lo);
    }
    (hi + 0.05) / (lo + 0.05)
}

/// `color-mix(in srgb, tint pct%, base)` — the same blend the badge/chip rules
/// use to derive their tinted background from an accent over white.
fn srgb_mix(tint: Rgb, base: Rgb, pct: f64) -> Rgb {
    let mix =
        |t: u8, b: u8| (t as f64 * pct / 100.0 + b as f64 * (100.0 - pct) / 100.0).round() as u8;
    (
        mix(tint.0, base.0),
        mix(tint.1, base.1),
        mix(tint.2, base.2),
    )
}

/// Pull a `--ui-…: #rrggbb;` declaration out of a theme block. `block_start`
/// scopes the search to the light vs. dark token block.
fn token_hex(css: &str, block_start: &str, token: &str) -> Rgb {
    let from = css.find(block_start).expect("theme block exists");
    let scope = &css[from..];
    let needle = format!("{token}:");
    let decl_at = scope
        .find(&needle)
        .unwrap_or_else(|| panic!("token {token} declared in {block_start}"));
    let after = &scope[decl_at + needle.len()..];
    let value = after.split(';').next().unwrap().trim();
    hex(value)
}

const AA: f64 = 4.5;

#[test]
fn danger_button_on_color_clears_aa_in_both_themes() {
    let css = base_css();

    // Light: white-on-danger over the light danger fill.
    let light_danger = token_hex(&css, ":root", "--ui-danger");
    let light_on = token_hex(&css, ":root", "--ui-on-danger");
    let light_ratio = contrast_ratio(light_on, light_danger);
    assert!(
        light_ratio >= AA,
        "light danger button {light_ratio:.3}:1 < {AA}:1 (on={light_on:?} bg={light_danger:?})"
    );

    // Dark: the regression that started all this — white over #ff6b6b is only
    // ~2.78:1, so --ui-on-danger must be a dark on-color here.
    let dark_danger = token_hex(&css, "[data-ui-theme=\"dark\"]", "--ui-danger");
    let dark_on = token_hex(&css, "[data-ui-theme=\"dark\"]", "--ui-on-danger");
    let dark_ratio = contrast_ratio(dark_on, dark_danger);
    assert!(
        dark_ratio >= AA,
        "dark danger button {dark_ratio:.3}:1 < {AA}:1 (on={dark_on:?} bg={dark_danger:?})"
    );
}

#[test]
fn badge_accent_text_clears_aa_on_its_own_light_tint() {
    let css = base_css();

    // `.ui-badge--info` / `--success` / `--primary` paint the accent as text on
    // a `color-mix(in srgb, accent, transparent 86%)` background — i.e. a 14%
    // accent tint over the white surface.
    for token in ["--ui-info", "--ui-success", "--ui-primary"] {
        let accent = token_hex(&css, ":root", token);
        let tint = srgb_mix(accent, WHITE, 14.0);
        let ratio = contrast_ratio(accent, tint);
        assert!(
            ratio >= AA,
            "badge {token} text {ratio:.3}:1 < {AA}:1 on its own 14% tint (accent={accent:?} tint={tint:?})"
        );
    }
}

#[test]
fn citation_chip_text_clears_aa_on_its_light_tint() {
    let css = base_css();

    // `.ui-citation-chip` uses `color: var(--ui-primary)` on a
    // `color-mix(in srgb, var(--ui-primary), transparent 84%)` background — a
    // 16% primary tint over white.
    let primary = token_hex(&css, ":root", "--ui-primary");
    let tint = srgb_mix(primary, WHITE, 16.0);
    let ratio = contrast_ratio(primary, tint);
    assert!(
        ratio >= AA,
        "citation chip text {ratio:.3}:1 < {AA}:1 on its 16% primary tint (primary={primary:?} tint={tint:?})"
    );
}

#[test]
fn solid_accent_on_white_is_not_regressed() {
    // Darkening the light accents for self-tinted text must not break the
    // solid-fill variants (e.g. white-on-primary button).
    let css = base_css();
    for token in ["--ui-info", "--ui-success", "--ui-primary"] {
        let accent = token_hex(&css, ":root", token);
        let ratio = contrast_ratio(WHITE, accent);
        assert!(
            ratio >= AA,
            "white-on-{token} {ratio:.3}:1 < {AA}:1 (accent={accent:?})"
        );
    }
}
