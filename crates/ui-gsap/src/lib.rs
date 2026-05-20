#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GsapCapability {
    Timeline,
    Scroll,
    Flip,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GsapBackend {
    _private: (),
}

impl GsapBackend {
    pub const fn target(&self) -> &'static str {
        "web"
    }

    pub const fn capabilities(&self) -> &'static [GsapCapability] {
        &[
            GsapCapability::Timeline,
            GsapCapability::Scroll,
            GsapCapability::Flip,
        ]
    }
}
