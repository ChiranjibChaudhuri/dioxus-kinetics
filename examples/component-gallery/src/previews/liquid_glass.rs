use dioxus::prelude::*;
use ui_glass::{AmbientMesh, LiquidMaterial};
use ui_glass_dioxus::LiquidSurface;
use ui_glass_engine::background::{BackgroundSource, Gradient, GradientStop};
use ui_tokens::Color;

pub fn liquid_surface_preview() -> Element {
    rsx! {
        LiquidSurface {
            material: LiquidMaterial::floating()
                .ambient_mesh(AmbientMesh::Aurora)
                .pointer_reactive()
                .radius(24.0)
                .tint(Color::rgba(255, 255, 255, 1.0), 0.18),
            background: Some(BackgroundSource::Gradient(Gradient::conic(
                [0.5, 0.5], 0.0,
                vec![
                    GradientStop { offset: 0.0, color: Color::rgba( 80, 100, 220, 1.0) },
                    GradientStop { offset: 0.5, color: Color::rgba(180,  80, 180, 1.0) },
                    GradientStop { offset: 1.0, color: Color::rgba( 80, 100, 220, 1.0) },
                ],
            ))),
            width: 400,
            height: 240,
            div {
                style: "padding: 16px; color: white; font-weight: 600;",
                "Hover me"
            }
        }
    }
}
