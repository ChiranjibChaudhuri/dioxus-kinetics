//! Generates SVG `<filter>` chains for the Tier 4 fallback backend. Each
//! material is rendered as a `<filter>` element with `feGaussianBlur` +
//! `feSpecularLighting` + `feColorMatrix` + (optionally) `feDisplacementMap`.
//! The resulting filter id can be referenced via
//! `backdrop-filter: url(#kinetics-glass-{id})` on the surface element.

use ui_glass::{GlassFeatures, LiquidMaterial};

/// Stable id derived from material features + key parameters.
pub fn filter_id(material: &LiquidMaterial) -> String {
    let f = material.features.bits();
    let blur = (material.blur_radius_px * 10.0) as u32;
    let refract = (material.refraction_strength * 100.0) as u32;
    format!("kinetics-glass-{f:x}-{blur:x}-{refract:x}")
}

/// Generate the `<filter>` element body for a material.
pub fn filter_element(material: &LiquidMaterial) -> String {
    let id = filter_id(material);
    let mut out =
        format!("<filter id=\"{id}\" x=\"-20%\" y=\"-20%\" width=\"140%\" height=\"140%\">");

    if material.features.contains(GlassFeatures::REFRACT) {
        let scale = (material.refraction_strength * 10.0) as i32;
        let freq = format!("{:.2}", material.noise_frequency * 0.02);
        out.push_str(&format!(
            "<feTurbulence type=\"fractalNoise\" baseFrequency=\"{freq}\" numOctaves=\"2\" result=\"turb\"/>\
             <feDisplacementMap in=\"SourceGraphic\" in2=\"turb\" scale=\"{scale}\" result=\"disp\"/>",
        ));
    } else {
        out.push_str("<feOffset in=\"SourceGraphic\" dx=\"0\" dy=\"0\" result=\"disp\"/>");
    }

    if material.features.contains(GlassFeatures::BLUR) {
        let std = format!("{:.1}", material.blur_radius_px * 0.5);
        out.push_str(&format!(
            "<feGaussianBlur in=\"disp\" stdDeviation=\"{std}\" result=\"blur\"/>",
        ));
    } else {
        out.push_str("<feOffset in=\"disp\" dx=\"0\" dy=\"0\" result=\"blur\"/>");
    }

    let sat = format!("{:.2}", material.saturation);
    out.push_str(&format!(
        "<feColorMatrix in=\"blur\" type=\"saturate\" values=\"{sat}\" result=\"sat\"/>",
    ));

    if material.features.contains(GlassFeatures::SPECULAR) {
        let intensity = format!("{:.2}", material.light_intensity);
        let elev = format!(
            "{:.1}",
            60.0 + material.light_angle_rad.to_degrees().abs() * 0.1
        );
        out.push_str(&format!(
            "<feSpecularLighting in=\"sat\" specularExponent=\"4\" specularConstant=\"{intensity}\" lighting-color=\"#ffffff\" result=\"spec\">\
             <feDistantLight azimuth=\"45\" elevation=\"{elev}\"/>\
             </feSpecularLighting>\
             <feComposite in=\"spec\" in2=\"SourceAlpha\" operator=\"in\" result=\"specMasked\"/>\
             <feComposite in=\"sat\" in2=\"specMasked\" operator=\"arithmetic\" k1=\"0\" k2=\"1\" k3=\"1\" k4=\"0\"/>",
        ));
    }

    out.push_str("</filter>");
    out
}

/// Generate a single `<svg>` `<defs>` block containing filters for all given
/// materials. Apps insert this once at the root of the document; surfaces
/// then reference filters by id.
pub fn defs_for(materials: &[&LiquidMaterial]) -> String {
    let filters: Vec<String> = materials.iter().map(|m| filter_element(m)).collect();
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"0\" height=\"0\" style=\"position:absolute;width:0;height:0;\" aria-hidden=\"true\"><defs>{}</defs></svg>",
        filters.join(""),
    )
}
