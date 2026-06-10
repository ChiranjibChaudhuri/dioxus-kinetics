use dioxus::prelude::*;
use ui_dioxus::{ChartTone, DonutGauge, Sparkline};

// ---------------------------------------------------------------------------
// Course vocabulary
// ---------------------------------------------------------------------------

/// Lifecycle of a lesson inside a `CourseOutline`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LessonState {
    /// Not started, but selectable.
    #[default]
    Available,
    /// The learner's current position; rendered with `aria-current="step"`.
    Current,
    /// Finished; shows a completion check.
    Completed,
    /// Gated by prerequisites; rendered disabled.
    Locked,
}

impl LessonState {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Current => "current",
            Self::Completed => "completed",
            Self::Locked => "locked",
        }
    }

    pub const fn is_selectable(self) -> bool {
        !matches!(self, Self::Locked)
    }
}

/// One lesson row in a course module.
#[derive(Clone, Debug, PartialEq)]
pub struct CourseLesson {
    pub id: String,
    pub title: String,
    pub duration: String,
    pub state: LessonState,
}

impl CourseLesson {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            duration: String::new(),
            state: LessonState::default(),
        }
    }

    pub fn with_duration(mut self, duration: impl Into<String>) -> Self {
        self.duration = duration.into();
        self
    }

    pub fn with_state(mut self, state: LessonState) -> Self {
        self.state = state;
        self
    }
}

/// A titled group of lessons.
#[derive(Clone, Debug, PartialEq)]
pub struct CourseModule {
    pub id: String,
    pub title: String,
    pub lessons: Vec<CourseLesson>,
}

impl CourseModule {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        lessons: Vec<CourseLesson>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            lessons,
        }
    }
}

/// `(completed, total)` lesson counts across a set of modules.
pub fn course_progress(modules: &[CourseModule]) -> (usize, usize) {
    let mut completed = 0;
    let mut total = 0;
    for module in modules {
        for lesson in &module.lessons {
            total += 1;
            if lesson.state == LessonState::Completed {
                completed += 1;
            }
        }
    }
    (completed, total)
}

fn module_progress(module: &CourseModule) -> (usize, usize) {
    course_progress(std::slice::from_ref(module))
}

/// "completed of total" fraction, NaN-safe for empty courses.
fn progress_fraction(completed: usize, total: usize) -> f32 {
    if total == 0 {
        0.0
    } else {
        (completed as f32 / total as f32).clamp(0.0, 1.0)
    }
}

// ---------------------------------------------------------------------------
// CourseOutline
// ---------------------------------------------------------------------------

/// A course curriculum tree: modules with per-lesson state (available /
/// current / completed / locked). Lessons are real buttons — locked ones are
/// `disabled`, the current one carries `aria-current="step"` — and each
/// module header reports its completion count.
///
/// Controlled: render the states you store; `on_select` reports the clicked
/// lesson id.
#[component]
pub fn CourseOutline(
    label: String,
    modules: Vec<CourseModule>,
    on_select: EventHandler<String>,
) -> Element {
    rsx! {
        nav { class: "ui-course-outline", "aria-label": "{label}",
            for module in modules.iter() {
                {
                    let (done, total) = module_progress(module);
                    rsx! {
                        section { key: "{module.id}", class: "ui-course-module",
                            header { class: "ui-course-module-header",
                                h4 { class: "ui-course-module-title", "{module.title}" }
                                span { class: "ui-course-module-count ui-tabular",
                                    "{done} / {total}"
                                }
                            }
                            ul { class: "ui-course-lessons",
                                for lesson in module.lessons.iter() {
                                    {
                                        let id = lesson.id.clone();
                                        let state = lesson.state;
                                        rsx! {
                                            li { key: "{lesson.id}",
                                                class: "ui-course-lesson ui-course-lesson--{state.class_suffix()}",
                                                button {
                                                    class: "ui-course-lesson-button",
                                                    r#type: "button",
                                                    disabled: !state.is_selectable(),
                                                    "aria-current": if state == LessonState::Current { "step" } else { "false" },
                                                    onclick: move |_| on_select.call(id.clone()),
                                                    span { class: "ui-course-lesson-marker", "aria-hidden": "true",
                                                        match state {
                                                            LessonState::Completed => rsx! {
                                                                svg { view_box: "0 0 16 16",
                                                                    path {
                                                                        d: "M3 8.5 6.5 12 13 4.5",
                                                                        fill: "none",
                                                                        stroke: "currentColor",
                                                                        stroke_width: "2",
                                                                        stroke_linecap: "round",
                                                                        stroke_linejoin: "round",
                                                                    }
                                                                }
                                                            },
                                                            LessonState::Locked => rsx! {
                                                                svg { view_box: "0 0 16 16",
                                                                    rect {
                                                                        x: "3.5", y: "7", width: "9", height: "6.5",
                                                                        rx: "1.5", fill: "currentColor",
                                                                    }
                                                                    path {
                                                                        d: "M5.5 7V5.5a2.5 2.5 0 0 1 5 0V7",
                                                                        fill: "none",
                                                                        stroke: "currentColor",
                                                                        stroke_width: "1.6",
                                                                    }
                                                                }
                                                            },
                                                            _ => rsx! { span { class: "ui-course-lesson-dot" } },
                                                        }
                                                    }
                                                    span { class: "ui-course-lesson-body",
                                                        span { class: "ui-course-lesson-title", "{lesson.title}" }
                                                        if !lesson.duration.is_empty() {
                                                            span { class: "ui-course-lesson-duration", "{lesson.duration}" }
                                                        }
                                                    }
                                                    if state == LessonState::Locked {
                                                        span { class: "visually-hidden", "Locked" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CourseProgressCard
// ---------------------------------------------------------------------------

/// A course-level progress readout: completion gauge, lesson counts, and an
/// optional recent-activity trend line.
#[component]
pub fn CourseProgressCard(
    title: String,
    completed: usize,
    total: usize,
    #[props(default)] time_remaining: String,
    #[props(default)] trend: Vec<f32>,
) -> Element {
    let fraction = progress_fraction(completed, total);
    let percent = (fraction * 100.0).round() as u32;

    rsx! {
        article { class: "ui-course-progress-card",
            div { class: "ui-course-progress-gauge",
                DonutGauge {
                    label: "{title}: {percent} percent complete",
                    value: fraction,
                    description: "complete".to_string(),
                    tone: ChartTone::Primary,
                }
            }
            div { class: "ui-course-progress-body",
                h4 { class: "ui-course-progress-title", "{title}" }
                p { class: "ui-course-progress-counts",
                    span { class: "ui-tabular", "{completed} of {total}" }
                    " lessons completed"
                }
                if !time_remaining.is_empty() {
                    p { class: "ui-course-progress-remaining", "{time_remaining} remaining" }
                }
                if trend.len() >= 2 {
                    div { class: "ui-course-progress-trend",
                        Sparkline {
                            points: trend,
                            label: "Recent learning activity".to_string(),
                            tone: ChartTone::Primary,
                            filled: true,
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ResumeLearning
// ---------------------------------------------------------------------------

/// A "pick up where you left off" strip: course + lesson context, a thin
/// progress track, and one primary action.
#[component]
pub fn ResumeLearning(
    course: String,
    lesson: String,
    /// Lesson progress in `0.0..=1.0`.
    progress: f32,
    on_resume: EventHandler<()>,
    #[props(default = "Continue".to_string())] action_label: String,
) -> Element {
    let fraction = if progress.is_finite() {
        progress.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let percent = (fraction * 100.0).round() as u32;

    rsx! {
        section {
            class: "ui-resume-learning",
            "aria-label": "Resume learning: {course}",
            div { class: "ui-resume-learning-body",
                p { class: "ui-resume-learning-course", "{course}" }
                p { class: "ui-resume-learning-lesson", "{lesson}" }
                div {
                    class: "ui-resume-learning-track",
                    role: "progressbar",
                    "aria-valuemin": "0",
                    "aria-valuemax": "100",
                    "aria-valuenow": "{percent}",
                    "aria-label": "Lesson progress",
                    div {
                        class: "ui-resume-learning-fill",
                        style: "width:{percent}%",
                    }
                }
            }
            button {
                class: "ui-button ui-button--primary",
                r#type: "button",
                onclick: move |_| on_resume.call(()),
                "{action_label}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_modules() -> Vec<CourseModule> {
        vec![
            CourseModule::new(
                "m1",
                "Basics",
                vec![
                    CourseLesson::new("l1", "Intro").with_state(LessonState::Completed),
                    CourseLesson::new("l2", "Setup").with_state(LessonState::Current),
                ],
            ),
            CourseModule::new(
                "m2",
                "Advanced",
                vec![CourseLesson::new("l3", "Deep dive").with_state(LessonState::Locked)],
            ),
        ]
    }

    #[test]
    fn course_progress_counts_completed_lessons() {
        assert_eq!(course_progress(&sample_modules()), (1, 3));
        assert_eq!(course_progress(&[]), (0, 0));
    }

    #[test]
    fn module_progress_is_per_module() {
        let modules = sample_modules();
        assert_eq!(module_progress(&modules[0]), (1, 2));
        assert_eq!(module_progress(&modules[1]), (0, 1));
    }

    #[test]
    fn progress_fraction_is_safe_for_empty_courses() {
        assert_eq!(progress_fraction(0, 0), 0.0);
        assert_eq!(progress_fraction(1, 2), 0.5);
        assert_eq!(progress_fraction(5, 4), 1.0);
    }

    #[test]
    fn lesson_state_selectability() {
        assert!(LessonState::Available.is_selectable());
        assert!(LessonState::Current.is_selectable());
        assert!(LessonState::Completed.is_selectable());
        assert!(!LessonState::Locked.is_selectable());
    }

    #[test]
    fn lesson_state_class_suffixes() {
        assert_eq!(LessonState::Available.class_suffix(), "available");
        assert_eq!(LessonState::Current.class_suffix(), "current");
        assert_eq!(LessonState::Completed.class_suffix(), "completed");
        assert_eq!(LessonState::Locked.class_suffix(), "locked");
    }
}
