#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelineCapability {
    Tracks,
    Labels,
    Stagger,
    SharedMove,
    ScrollProgress,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TimelineRuntime {
    _private: (),
}

impl TimelineRuntime {
    pub const fn runtime(&self) -> &'static str {
        "cross-platform"
    }

    pub const fn capabilities(&self) -> &'static [TimelineCapability] {
        &[
            TimelineCapability::Tracks,
            TimelineCapability::Labels,
            TimelineCapability::Stagger,
            TimelineCapability::SharedMove,
            TimelineCapability::ScrollProgress,
        ]
    }
}
