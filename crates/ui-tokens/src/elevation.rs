#[derive(Clone, Copy, Debug)]
pub struct ElevationScale {
    pub e0: &'static str,
    pub e1: &'static str,
    pub e2: &'static str,
    pub e3: &'static str,
}

// Light elevation ladder. Each lifted step (e1..e3) is composed of:
//   1. a tight key shadow (small blur, slight Y) for crisp contact;
//   2. a 0-offset hairline ambient ring that grounds the edge;
//   3. a large soft ambient shadow for the diffuse spread;
//   4. a single faint top specular rim (inset 0 1px 0 rgba(255,255,255,0.6))
//      that reads as a glossy Liquid-Glass highlight under top lighting.
// The rim is added ONCE per step (item 3 of the Wave-1 roadmap).
pub const LIGHT_ELEVATION: ElevationScale = ElevationScale {
    e0: "0 1px 0 rgba(16, 23, 38, 0.04)",
    e1: "0 1px 2px rgba(16, 23, 38, 0.08), 0 0 0 0.5px rgba(16, 23, 38, 0.05), 0 8px 24px rgba(16, 23, 38, 0.06), inset 0 1px 0 rgba(255, 255, 255, 0.6)",
    e2: "0 4px 10px rgba(16, 23, 38, 0.10), 0 0 0 0.5px rgba(16, 23, 38, 0.06), 0 22px 48px rgba(16, 23, 38, 0.12), inset 0 1px 0 rgba(255, 255, 255, 0.6)",
    e3: "0 10px 22px rgba(16, 23, 38, 0.14), 0 0 0 0.5px rgba(16, 23, 38, 0.07), 0 40px 80px rgba(16, 23, 38, 0.20), inset 0 1px 0 rgba(255, 255, 255, 0.6)",
};

pub const DARK_ELEVATION: ElevationScale = ElevationScale {
    e0: "0 1px 0 rgba(0, 0, 0, 0.18), inset 0 1px 0 rgba(255, 255, 255, 0.04)",
    e1: "0 2px 6px rgba(0, 0, 0, 0.30), 0 8px 24px rgba(0, 0, 0, 0.28), inset 0 1px 0 rgba(255, 255, 255, 0.05)",
    e2: "0 10px 22px rgba(0, 0, 0, 0.38), 0 26px 60px rgba(0, 0, 0, 0.42), inset 0 1px 0 rgba(255, 255, 255, 0.06)",
    e3: "0 22px 40px rgba(0, 0, 0, 0.46), 0 48px 96px rgba(0, 0, 0, 0.50), inset 0 1px 0 rgba(255, 255, 255, 0.08)",
};
