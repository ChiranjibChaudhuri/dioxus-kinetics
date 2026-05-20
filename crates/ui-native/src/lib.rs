#![forbid(unsafe_code)]

use ui_glass::GlassRecipe;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeCapabilities {
    pub backdrop_sampling: bool,
    pub filters: bool,
    pub elevation_shadows: bool,
}

impl NativeCapabilities {
    pub const fn minimal() -> Self {
        Self {
            backdrop_sampling: false,
            filters: false,
            elevation_shadows: true,
        }
    }

    pub const fn with_backdrop_filters() -> Self {
        Self {
            backdrop_sampling: true,
            filters: true,
            elevation_shadows: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativeGlassPlan {
    pub uses_backdrop_blur: bool,
    pub uses_simulated_glass: bool,
    pub effective_blur_px: f32,
}

pub fn plan_native_glass(
    recipe: &GlassRecipe,
    capabilities: NativeCapabilities,
) -> NativeGlassPlan {
    let can_blur = capabilities.backdrop_sampling && capabilities.filters && !recipe.force_solid;

    NativeGlassPlan {
        uses_backdrop_blur: can_blur,
        uses_simulated_glass: !can_blur,
        effective_blur_px: if can_blur && recipe.backdrop_blur_px.is_finite() {
            recipe.backdrop_blur_px
        } else {
            0.0
        },
    }
}
