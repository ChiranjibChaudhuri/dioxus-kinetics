#![forbid(unsafe_code)]

use ui_glass::GlassRecipe;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CssStyleWriter {
    declarations: Vec<(String, String)>,
}

impl CssStyleWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.declarations.push((name.into(), value.into()));
        self
    }

    pub fn to_inline_style(&self) -> String {
        self.declarations
            .iter()
            .map(|(name, value)| format!("{name}:{value};"))
            .collect()
    }
}

pub fn glass_style(recipe: &GlassRecipe, supports_backdrop_filter: bool) -> String {
    let mut writer = CssStyleWriter::new()
        .set(
            "background",
            if supports_backdrop_filter && !recipe.force_solid {
                recipe.background.css_rgba()
            } else {
                recipe.fallback_background.css_rgba()
            },
        )
        .set("border", format!("1px solid {}", recipe.border.css_rgba()))
        .set("color", recipe.foreground.css_rgba())
        .set(
            "border-radius",
            format!("{}px", trim_float(recipe.radius_px)),
        )
        .set(
            "box-shadow",
            format!(
                "0 18px 42px rgba(20, 23, 28, {:.3})",
                finite_or_zero(recipe.shadow_alpha)
            ),
        );

    if supports_backdrop_filter && !recipe.force_solid && recipe.backdrop_blur_px > 0.0 {
        writer = writer.set(
            "backdrop-filter",
            format!(
                "blur({}px) saturate({}%)",
                trim_float(recipe.backdrop_blur_px),
                recipe.saturate_percent
            ),
        );
    }

    writer.to_inline_style()
}

pub fn material_style(recipe: &ui_glass::GlassRecipe) -> String {
    CssStyleWriter::new()
        .set("--ui-material-bg", recipe.background.css_rgba())
        .set(
            "--ui-material-solid-bg",
            recipe.fallback_background.css_rgba(),
        )
        .set("--ui-material-border", recipe.border.css_rgba())
        .set(
            "--ui-material-blur",
            format!("{}px", trim_float(recipe.backdrop_blur_px)),
        )
        .set(
            "--ui-material-saturate",
            format!("{}%", recipe.saturate_percent),
        )
        .set("background", "var(--ui-material-bg)")
        .set("border-color", "var(--ui-material-border)")
        .set(
            "backdrop-filter",
            "blur(var(--ui-material-blur)) saturate(var(--ui-material-saturate))",
        )
        .set(
            "-webkit-backdrop-filter",
            "blur(var(--ui-material-blur)) saturate(var(--ui-material-saturate))",
        )
        .to_inline_style()
}

/// Wide-gamut sibling of [`material_style`]: emits the same `--ui-material-*`
/// custom properties but expresses colors with `color(display-p3 ...)` so the
/// material can opt into a wider working space on capable displays. The values
/// are gamut-clamped to sRGB by construction, so this stays visually identical
/// to [`material_style`]; callers can gate it behind an `@supports` query.
pub fn material_style_p3(recipe: &ui_glass::GlassRecipe) -> String {
    CssStyleWriter::new()
        .set("--ui-material-bg", recipe.background.css_p3())
        .set(
            "--ui-material-solid-bg",
            recipe.fallback_background.css_p3(),
        )
        .set("--ui-material-border", recipe.border.css_p3())
        .set(
            "--ui-material-blur",
            format!("{}px", trim_float(recipe.backdrop_blur_px)),
        )
        .set(
            "--ui-material-saturate",
            format!("{}%", recipe.saturate_percent),
        )
        .set("background", "var(--ui-material-bg)")
        .set("border-color", "var(--ui-material-border)")
        .set(
            "backdrop-filter",
            "blur(var(--ui-material-blur)) saturate(var(--ui-material-saturate))",
        )
        .set(
            "-webkit-backdrop-filter",
            "blur(var(--ui-material-blur)) saturate(var(--ui-material-saturate))",
        )
        .to_inline_style()
}

fn trim_float(value: f32) -> String {
    let value = finite_or_zero(value);

    if value.fract() == 0.0 {
        format!("{}", value as i32)
    } else {
        format!("{value:.2}")
    }
}

fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}
