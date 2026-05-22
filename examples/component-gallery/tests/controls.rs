use component_gallery::controls::{DensityPref, GlassPolicyUi, MotionPref, ThemePref};

#[test]
fn theme_pref_has_two_attribute_values() {
    assert_eq!(ThemePref::Light.attr_value(), "light");
    assert_eq!(ThemePref::Dark.attr_value(), "dark");
}

#[test]
fn density_pref_has_three_attribute_values() {
    assert_eq!(DensityPref::Compact.attr_value(), "compact");
    assert_eq!(DensityPref::Comfortable.attr_value(), "comfortable");
    assert_eq!(DensityPref::Spacious.attr_value(), "spacious");
}

#[test]
fn motion_pref_maps_to_normal_or_reduced_attribute() {
    assert_eq!(MotionPref::Normal.attr_value(), "normal");
    assert_eq!(MotionPref::Reduced.attr_value(), "reduced");
}

#[test]
fn glass_policy_ui_maps_to_translucent_or_solid_attribute() {
    assert_eq!(GlassPolicyUi::Translucent.attr_value(), "translucent");
    assert_eq!(GlassPolicyUi::Solid.attr_value(), "solid");
}

#[test]
fn enums_round_trip_via_attr_value() {
    assert_eq!(ThemePref::from_attr("dark"), Some(ThemePref::Dark));
    assert_eq!(ThemePref::from_attr("nope"), None);
    assert_eq!(MotionPref::from_attr("reduced"), Some(MotionPref::Reduced));
    assert_eq!(
        GlassPolicyUi::from_attr("solid"),
        Some(GlassPolicyUi::Solid)
    );
    assert_eq!(
        DensityPref::from_attr("compact"),
        Some(DensityPref::Compact)
    );
}

#[test]
fn gallery_default_constants_match_documented_fallbacks() {
    use component_gallery::controls::{
        DEFAULT_DENSITY, DEFAULT_GLASS, DEFAULT_MOTION, DEFAULT_THEME,
    };
    assert_eq!(DEFAULT_THEME, ThemePref::Light);
    assert_eq!(DEFAULT_DENSITY, DensityPref::Comfortable);
    assert_eq!(DEFAULT_MOTION, MotionPref::Normal);
    assert_eq!(DEFAULT_GLASS, GlassPolicyUi::Translucent);
}
