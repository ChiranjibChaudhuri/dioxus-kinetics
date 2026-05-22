#![forbid(unsafe_code)]

use ui_motion::{apply_ease, interpolate, Clamp, Transition};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineId(pub String);

impl TimelineId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KineticId(pub String);

impl KineticId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MotionTarget {
    SelfNode,
    Node(KineticId),
}

impl MotionTarget {
    pub const fn self_node() -> Self {
        Self::SelfNode
    }

    pub fn node(id: impl Into<String>) -> Self {
        Self::Node(KineticId::new(id))
    }
}

impl Default for MotionTarget {
    fn default() -> Self {
        Self::SelfNode
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimelineClock {
    Playback { elapsed_ms: f32 },
    Manual { elapsed_ms: f32 },
    Scroll { progress: f32 },
    Frame { frame: u64, fps: f32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineLabel {
    pub name: String,
    pub offset_ms: f32,
}

impl TimelineLabel {
    pub fn new(name: impl Into<String>, offset_ms: f32) -> Self {
        Self {
            name: name.into(),
            offset_ms: finite_non_negative(offset_ms),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepeatMode {
    None,
    Count { count: u32, yoyo: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Timeline {
    pub id: TimelineId,
    pub duration_ms: f32,
    pub labels: Vec<TimelineLabel>,
    pub tracks: Vec<TimelineTrack>,
    pub repeat: RepeatMode,
    pub fill: FillMode,
}

impl Timeline {
    pub fn new(id: impl Into<String>, duration_ms: f32) -> Self {
        Self {
            id: TimelineId::new(id),
            duration_ms: finite_non_negative(duration_ms),
            labels: Vec::new(),
            tracks: Vec::new(),
            repeat: RepeatMode::None,
            fill: FillMode::None,
        }
    }

    pub fn with_label(mut self, label: TimelineLabel) -> Self {
        self.labels.push(label);
        self
    }

    pub fn with_track(mut self, track: TimelineTrack) -> Self {
        self.tracks.push(track);
        self
    }

    pub const fn with_repeat(mut self, repeat: RepeatMode) -> Self {
        self.repeat = repeat;
        self
    }

    pub const fn with_fill(mut self, fill: FillMode) -> Self {
        self.fill = fill;
        self
    }

    pub fn label_offset(&self, name: &str) -> Option<f32> {
        self.labels
            .iter()
            .find(|label| label.name == name)
            .map(|label| label.offset_ms)
    }

    pub fn reduced_motion(&self) -> Self {
        let mut reduced = self.clone();
        reduced.duration_ms = 0.0;
        reduced.tracks = reduced
            .tracks
            .into_iter()
            .map(TimelineTrack::reduced_motion)
            .collect();
        reduced
    }

    pub fn sample(&self, clock: TimelineClock) -> TimelineSample {
        let elapsed_ms = self.resolve_elapsed_ms(clock);
        let position = self.map_repeat_elapsed(elapsed_ms);
        let states = if position.phase.can_emit(self.fill) {
            self.tracks
                .iter()
                .filter_map(|track| track.sample(position.local_ms, self.fill))
                .collect()
        } else {
            Vec::new()
        };

        TimelineSample { elapsed_ms, states }
    }

    fn resolve_elapsed_ms(&self, clock: TimelineClock) -> f32 {
        match clock {
            TimelineClock::Playback { elapsed_ms } | TimelineClock::Manual { elapsed_ms } => {
                finite_non_negative(elapsed_ms)
            }
            TimelineClock::Scroll { progress } => {
                finite_non_negative(progress).clamp(0.0, 1.0) * self.duration_ms
            }
            TimelineClock::Frame { frame, fps } => {
                let fps = if fps.is_finite() && fps > 0.0 {
                    fps
                } else {
                    60.0
                };
                (frame as f32 / fps) * 1000.0
            }
        }
    }

    fn map_repeat_elapsed(&self, elapsed_ms: f32) -> TimelinePosition {
        let duration_ms = finite_non_negative(self.duration_ms);
        if duration_ms == 0.0 {
            return TimelinePosition {
                local_ms: 0.0,
                phase: TimelinePhase::Active,
            };
        }

        let (repeat_count, yoyo) = match self.repeat {
            RepeatMode::None => (1, false),
            RepeatMode::Count { count, yoyo } => (count.max(1), yoyo),
        };
        let total_ms = duration_ms * repeat_count as f32;
        if elapsed_ms > total_ms {
            let local_ms = if yoyo && repeat_count % 2 == 0 {
                0.0
            } else {
                duration_ms
            };
            return TimelinePosition {
                local_ms,
                phase: TimelinePhase::After,
            };
        }

        let mut iteration = (elapsed_ms / duration_ms).floor() as u32;
        let mut local_ms = elapsed_ms - duration_ms * iteration as f32;
        if iteration >= repeat_count {
            iteration = repeat_count - 1;
            local_ms = duration_ms;
        }

        if yoyo && iteration % 2 == 1 {
            local_ms = duration_ms - local_ms;
        }

        TimelinePosition {
            local_ms,
            phase: TimelinePhase::Active,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TimelinePosition {
    local_ms: f32,
    phase: TimelinePhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TimelinePhase {
    Active,
    After,
}

impl TimelinePhase {
    const fn can_emit(self, fill: FillMode) -> bool {
        match self {
            Self::Active => true,
            Self::After => matches!(fill, FillMode::Forwards | FillMode::Both),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineTrack {
    pub target: MotionTarget,
    pub segments: Vec<MotionSegment>,
}

impl TimelineTrack {
    pub fn new(target: MotionTarget, segments: Vec<MotionSegment>) -> Self {
        Self { target, segments }
    }

    fn reduced_motion(self) -> Self {
        Self {
            target: self.target,
            segments: self
                .segments
                .into_iter()
                .map(MotionSegment::reduced_motion)
                .collect(),
        }
    }

    fn sample(&self, elapsed_ms: f32, fill: FillMode) -> Option<ResolvedMotionState> {
        let mut merged = MotionCueSample::default();
        let mut any = false;
        for segment in &self.segments {
            if let Some(sample) = segment.sample(elapsed_ms, fill) {
                merged = merged.merge(sample);
                any = true;
            }
        }
        if !any {
            return None;
        }
        Some(ResolvedMotionState {
            target: self.target.clone(),
            opacity: merged.opacity,
            translate_x: merged.translate_x,
            translate_y: merged.translate_y,
            scale: merged.scale,
            rotate_deg: merged.rotate_deg,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MotionSegment {
    pub start_ms: f32,
    pub duration_ms: f32,
    pub cue: MotionCue,
}

impl MotionSegment {
    pub fn new(start_ms: f32, duration_ms: f32, cue: MotionCue) -> Self {
        Self {
            start_ms: finite_non_negative(start_ms),
            duration_ms: finite_non_negative(duration_ms),
            cue,
        }
    }

    fn reduced_motion(self) -> Self {
        Self {
            start_ms: 0.0,
            duration_ms: 0.0,
            cue: self.cue.reduced_motion(),
        }
    }

    fn sample(&self, elapsed_ms: f32, fill: FillMode) -> Option<MotionCueSample> {
        let elapsed_ms = finite_non_negative(elapsed_ms);
        let end_ms = self.start_ms + self.duration_ms;
        if elapsed_ms < self.start_ms && !matches!(fill, FillMode::Backwards | FillMode::Both) {
            return None;
        }
        if elapsed_ms > end_ms && !matches!(fill, FillMode::Forwards | FillMode::Both) {
            return None;
        }

        let progress = if self.duration_ms == 0.0 {
            1.0
        } else {
            (elapsed_ms - self.start_ms) / self.duration_ms
        };
        Some(self.cue.sample(progress))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MotionCue {
    Opacity {
        from: f32,
        to: f32,
        transition: Transition,
    },
    Translate {
        axis: Axis,
        from: f32,
        to: f32,
        transition: Transition,
    },
    Scale {
        from: f32,
        to: f32,
        transition: Transition,
    },
    Rotate {
        from_deg: f32,
        to_deg: f32,
        transition: Transition,
    },
}

impl MotionCue {
    pub const fn opacity(from: f32, to: f32, transition: Transition) -> Self {
        Self::Opacity {
            from,
            to,
            transition,
        }
    }

    fn reduced_motion(self) -> Self {
        match self {
            Self::Opacity {
                from,
                to,
                transition,
            } => Self::Opacity {
                from,
                to,
                transition: transition.reduced(),
            },
            Self::Translate {
                axis,
                from,
                to,
                transition,
            } => Self::Translate {
                axis,
                from,
                to,
                transition: transition.reduced(),
            },
            Self::Scale {
                from,
                to,
                transition,
            } => Self::Scale {
                from,
                to,
                transition: transition.reduced(),
            },
            Self::Rotate {
                from_deg,
                to_deg,
                transition,
            } => Self::Rotate {
                from_deg,
                to_deg,
                transition: transition.reduced(),
            },
        }
    }

    pub fn sample(self, progress: f32) -> MotionCueSample {
        let p = if progress.is_finite() {
            progress.clamp(0.0, 1.0)
        } else {
            0.0
        };
        match self {
            Self::Opacity {
                from,
                to,
                transition,
            } => {
                let eased = apply_transition_progress(p, transition);
                MotionCueSample {
                    opacity: Some(interpolate(from, to, eased, Clamp::Yes)),
                    ..Default::default()
                }
            }
            Self::Translate {
                axis,
                from,
                to,
                transition,
            } => {
                let eased = apply_transition_progress(p, transition);
                let value = interpolate(from, to, eased, Clamp::Yes);
                let mut sample = MotionCueSample::default();
                match axis {
                    Axis::X => sample.translate_x = Some(value),
                    Axis::Y => sample.translate_y = Some(value),
                }
                sample
            }
            Self::Scale {
                from,
                to,
                transition,
            } => {
                let eased = apply_transition_progress(p, transition);
                MotionCueSample {
                    scale: Some(interpolate(from, to, eased, Clamp::Yes)),
                    ..Default::default()
                }
            }
            Self::Rotate {
                from_deg,
                to_deg,
                transition,
            } => {
                let eased = apply_transition_progress(p, transition);
                MotionCueSample {
                    rotate_deg: Some(interpolate(from_deg, to_deg, eased, Clamp::Yes)),
                    ..Default::default()
                }
            }
        }
    }
}

fn apply_transition_progress(progress: f32, transition: Transition) -> f32 {
    match transition {
        Transition::Tween { ease, .. } => apply_ease(progress.clamp(0.0, 1.0), ease),
        Transition::Spring(_) => progress.clamp(0.0, 1.0),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineSample {
    pub elapsed_ms: f32,
    pub states: Vec<ResolvedMotionState>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResolvedMotionState {
    pub target: MotionTarget,
    pub opacity: Option<f32>,
    pub translate_x: Option<f32>,
    pub translate_y: Option<f32>,
    pub scale: Option<f32>,
    pub rotate_deg: Option<f32>,
}

impl ResolvedMotionState {
    pub fn inline_style(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if let Some(opacity) = self.opacity {
            parts.push(format!("opacity: {opacity}"));
        }
        let mut transform: Vec<String> = Vec::new();
        if self.translate_x.is_some() || self.translate_y.is_some() {
            let x = self.translate_x.unwrap_or(0.0);
            let y = self.translate_y.unwrap_or(0.0);
            transform.push(format!("translate({x}px, {y}px)"));
        }
        if let Some(scale) = self.scale {
            transform.push(format!("scale({scale})"));
        }
        if let Some(rotate) = self.rotate_deg {
            transform.push(format!("rotate({rotate}deg)"));
        }
        if !transform.is_empty() {
            parts.push(format!("transform: {}", transform.join(" ")));
        }
        parts.join("; ")
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StaggerFlow {
    ByIndex { step_ms: f32 },
}

impl StaggerFlow {
    pub fn offsets(self, count: usize) -> Vec<f32> {
        match self {
            Self::ByIndex { step_ms } => {
                let step_ms = finite_non_negative(step_ms);
                (0..count).map(|index| index as f32 * step_ms).collect()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MotionCueSample {
    pub opacity: Option<f32>,
    pub translate_x: Option<f32>,
    pub translate_y: Option<f32>,
    pub scale: Option<f32>,
    pub rotate_deg: Option<f32>,
}

impl MotionCueSample {
    pub fn merge(self, other: Self) -> Self {
        Self {
            opacity: other.opacity.or(self.opacity),
            translate_x: other.translate_x.or(self.translate_x),
            translate_y: other.translate_y.or(self.translate_y),
            scale: other.scale.or(self.scale),
            rotate_deg: other.rotate_deg.or(self.rotate_deg),
        }
    }
}

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

fn finite_non_negative(value: f32) -> f32 {
    finite_or_zero(value).max(0.0)
}

fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}
