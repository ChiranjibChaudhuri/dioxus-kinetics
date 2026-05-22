#[derive(Clone, Copy, Debug)]
pub struct ElevationScale {
    pub e0: &'static str,
    pub e1: &'static str,
    pub e2: &'static str,
    pub e3: &'static str,
}

pub const LIGHT_ELEVATION: ElevationScale = ElevationScale {
    e0: "0 1px 0 rgba(16, 23, 38, 0.04)",
    e1: "0 2px 6px rgba(16, 23, 38, 0.06), 0 8px 24px rgba(16, 23, 38, 0.05)",
    e2: "0 8px 18px rgba(16, 23, 38, 0.10), 0 22px 48px rgba(16, 23, 38, 0.10)",
    e3: "0 18px 32px rgba(16, 23, 38, 0.14), 0 40px 80px rgba(16, 23, 38, 0.18)",
};

pub const DARK_ELEVATION: ElevationScale = ElevationScale {
    e0: "0 1px 0 rgba(0, 0, 0, 0.18), inset 0 1px 0 rgba(255, 255, 255, 0.04)",
    e1: "0 2px 6px rgba(0, 0, 0, 0.30), 0 8px 24px rgba(0, 0, 0, 0.28), inset 0 1px 0 rgba(255, 255, 255, 0.05)",
    e2: "0 10px 22px rgba(0, 0, 0, 0.38), 0 26px 60px rgba(0, 0, 0, 0.42), inset 0 1px 0 rgba(255, 255, 255, 0.06)",
    e3: "0 22px 40px rgba(0, 0, 0, 0.46), 0 48px 96px rgba(0, 0, 0, 0.50), inset 0 1px 0 rgba(255, 255, 255, 0.08)",
};
