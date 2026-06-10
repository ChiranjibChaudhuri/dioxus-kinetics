use dioxus::prelude::*;
use ui_dioxus::{ChartTone, DonutGauge, SortableItem, SortableList};

// ---------------------------------------------------------------------------
// Question vocabulary
// ---------------------------------------------------------------------------

/// One selectable option in a choice or ordering question.
#[derive(Clone, Debug, PartialEq)]
pub struct QuizChoice {
    pub id: String,
    pub text: String,
}

impl QuizChoice {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

/// The shape of a question, including its answer key. Keys live in the model
/// so [`grade_answer`] can score locally; keep questions server-side until
/// reveal if your quiz is high-stakes.
#[derive(Clone, Debug, PartialEq)]
pub enum QuizPrompt {
    /// Pick exactly one choice.
    SingleChoice {
        choices: Vec<QuizChoice>,
        correct: String,
    },
    /// Pick every correct choice (set equality).
    MultiSelect {
        choices: Vec<QuizChoice>,
        correct: Vec<String>,
    },
    TrueFalse {
        correct: bool,
    },
    /// Arrange the items into `correct` order.
    Ordering {
        items: Vec<QuizChoice>,
        correct: Vec<String>,
    },
    /// Free text, graded against `accepted` after normalization.
    ShortAnswer {
        accepted: Vec<String>,
    },
}

/// A full question: prompt text, answer shape, optional explanation shown
/// after reveal.
#[derive(Clone, Debug, PartialEq)]
pub struct QuizQuestion {
    pub id: String,
    pub prompt: String,
    pub kind: QuizPrompt,
    pub explanation: String,
}

impl QuizQuestion {
    pub fn new(id: impl Into<String>, prompt: impl Into<String>, kind: QuizPrompt) -> Self {
        Self {
            id: id.into(),
            prompt: prompt.into(),
            kind,
            explanation: String::new(),
        }
    }

    pub fn with_explanation(mut self, explanation: impl Into<String>) -> Self {
        self.explanation = explanation.into();
        self
    }
}

/// A learner's response, mirroring the [`QuizPrompt`] variants.
#[derive(Clone, Debug, PartialEq)]
pub enum QuizAnswer {
    Choice(String),
    Choices(Vec<String>),
    Bool(bool),
    Order(Vec<String>),
    Text(String),
}

// ---------------------------------------------------------------------------
// Pure grading (unit-tested)
// ---------------------------------------------------------------------------

/// Lowercase, trim, and collapse internal whitespace so "  The  Mitochondria "
/// grades equal to "the mitochondria".
pub fn normalize_short_answer(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Grades an answer against its question. `None` when the answer shape does
/// not match the question kind (a wiring bug, not a wrong answer).
pub fn grade_answer(question: &QuizQuestion, answer: &QuizAnswer) -> Option<bool> {
    match (&question.kind, answer) {
        (QuizPrompt::SingleChoice { correct, .. }, QuizAnswer::Choice(picked)) => {
            Some(picked == correct)
        }
        (QuizPrompt::MultiSelect { correct, .. }, QuizAnswer::Choices(picked)) => {
            let mut want = correct.clone();
            let mut got = picked.clone();
            want.sort();
            want.dedup();
            got.sort();
            got.dedup();
            Some(want == got)
        }
        (QuizPrompt::TrueFalse { correct }, QuizAnswer::Bool(picked)) => Some(picked == correct),
        (QuizPrompt::Ordering { correct, .. }, QuizAnswer::Order(order)) => Some(order == correct),
        (QuizPrompt::ShortAnswer { accepted }, QuizAnswer::Text(text)) => {
            let normalized = normalize_short_answer(text);
            Some(
                accepted
                    .iter()
                    .any(|candidate| normalize_short_answer(candidate) == normalized),
            )
        }
        _ => None,
    }
}

/// Whether a choice id is part of the current (possibly multi) selection.
fn is_picked(answer: Option<&QuizAnswer>, id: &str) -> bool {
    match answer {
        Some(QuizAnswer::Choice(picked)) => picked == id,
        Some(QuizAnswer::Choices(picked)) => picked.iter().any(|entry| entry == id),
        _ => false,
    }
}

/// Toggles `id` within a multi-select answer.
fn toggle_choice(answer: Option<&QuizAnswer>, id: &str) -> QuizAnswer {
    let mut picked = match answer {
        Some(QuizAnswer::Choices(existing)) => existing.clone(),
        _ => Vec::new(),
    };
    if let Some(at) = picked.iter().position(|entry| entry == id) {
        picked.remove(at);
    } else {
        picked.push(id.to_string());
    }
    QuizAnswer::Choices(picked)
}

/// Per-option modifier class once the question is revealed.
fn revealed_option_class(correct_set: &[String], picked: bool, id: &str) -> &'static str {
    let is_correct = correct_set.iter().any(|entry| entry == id);
    match (is_correct, picked) {
        (true, _) => " ui-quiz-option--correct",
        (false, true) => " ui-quiz-option--incorrect",
        (false, false) => "",
    }
}

// ---------------------------------------------------------------------------
// QuestionCard
// ---------------------------------------------------------------------------

/// One quiz question with built-in rendering for all five [`QuizPrompt`]
/// shapes. Controlled: the host stores the `answer`, flips `revealed` when
/// it wants feedback shown, and grades with [`grade_answer`].
///
/// Choice options are native radio/checkbox inputs inside a
/// fieldset/legend; ordering reuses `SortableList` (full keyboard support);
/// reveal locks the inputs, marks correct/incorrect options, announces the
/// verdict through a `role="status"` line, and shows the explanation.
#[component]
pub fn QuestionCard(
    question: QuizQuestion,
    #[props(default)] answer: Option<QuizAnswer>,
    #[props(default)] revealed: bool,
    on_answer: EventHandler<QuizAnswer>,
    /// Optional context line like "Question 2 of 5".
    #[props(default)]
    counter: String,
) -> Element {
    let verdict = answer
        .as_ref()
        .and_then(|current| grade_answer(&question, current));
    let card_class = format!(
        "ui-question-card{}",
        match (revealed, verdict) {
            (true, Some(true)) => " ui-question-card--correct",
            (true, Some(false)) => " ui-question-card--incorrect",
            _ => "",
        }
    );

    rsx! {
        article { class: "{card_class}",
            fieldset { class: "ui-question-fieldset", disabled: revealed,
                legend { class: "ui-question-legend",
                    if !counter.is_empty() {
                        span { class: "ui-question-counter", "{counter}" }
                    }
                    span { class: "ui-question-prompt", "{question.prompt}" }
                }
                {question_body(&question, answer.as_ref(), revealed, on_answer)}
            }
            if revealed {
                div { class: "ui-question-feedback", role: "status",
                    strong { class: "ui-question-verdict",
                        match verdict {
                            Some(true) => "Correct",
                            Some(false) => "Incorrect",
                            None => "Not answered",
                        }
                    }
                    if !question.explanation.is_empty() {
                        p { class: "ui-question-explanation", "{question.explanation}" }
                    }
                }
            }
        }
    }
}

fn question_body(
    question: &QuizQuestion,
    answer: Option<&QuizAnswer>,
    revealed: bool,
    on_answer: EventHandler<QuizAnswer>,
) -> Element {
    let name = format!("quiz-{}", question.id);
    match &question.kind {
        QuizPrompt::SingleChoice { choices, correct } => {
            let correct_set = vec![correct.clone()];
            rsx! {
                ul { class: "ui-quiz-options",
                    for choice in choices.iter() {
                        {
                            let id = choice.id.clone();
                            let picked = is_picked(answer, &id);
                            let reveal_class = if revealed {
                                revealed_option_class(&correct_set, picked, &id)
                            } else {
                                ""
                            };
                            rsx! {
                                li { key: "{choice.id}",
                                    label { class: "ui-quiz-option{reveal_class}",
                                        input {
                                            r#type: "radio",
                                            name: "{name}",
                                            checked: picked,
                                            onchange: move |_| {
                                                on_answer.call(QuizAnswer::Choice(id.clone()));
                                            },
                                        }
                                        span { class: "ui-quiz-option-text", "{choice.text}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        QuizPrompt::MultiSelect { choices, correct } => {
            let correct_set = correct.clone();
            rsx! {
                ul { class: "ui-quiz-options",
                    for choice in choices.iter() {
                        {
                            let id = choice.id.clone();
                            let picked = is_picked(answer, &id);
                            let current = answer.cloned();
                            let reveal_class = if revealed {
                                revealed_option_class(&correct_set, picked, &id)
                            } else {
                                ""
                            };
                            rsx! {
                                li { key: "{choice.id}",
                                    label { class: "ui-quiz-option{reveal_class}",
                                        input {
                                            r#type: "checkbox",
                                            name: "{name}",
                                            checked: picked,
                                            onchange: move |_| {
                                                on_answer.call(toggle_choice(current.as_ref(), &id));
                                            },
                                        }
                                        span { class: "ui-quiz-option-text", "{choice.text}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        QuizPrompt::TrueFalse { correct } => {
            let picked = match answer {
                Some(QuizAnswer::Bool(value)) => Some(*value),
                _ => None,
            };
            rsx! {
                ul { class: "ui-quiz-options ui-quiz-options--inline",
                    for (value, text) in [(true, "True"), (false, "False")] {
                        {
                            let is_selected = picked == Some(value);
                            let reveal_class = if revealed {
                                if value == *correct {
                                    " ui-quiz-option--correct"
                                } else if is_selected {
                                    " ui-quiz-option--incorrect"
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            };
                            rsx! {
                                li { key: "{value}",
                                    label { class: "ui-quiz-option{reveal_class}",
                                        input {
                                            r#type: "radio",
                                            name: "{name}",
                                            checked: is_selected,
                                            onchange: move |_| {
                                                on_answer.call(QuizAnswer::Bool(value));
                                            },
                                        }
                                        span { class: "ui-quiz-option-text", "{text}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        QuizPrompt::Ordering { items, correct } => {
            // Render in the learner's current order; fall back to authored order.
            let order: Vec<String> = match answer {
                Some(QuizAnswer::Order(order)) => order.clone(),
                _ => items.iter().map(|item| item.id.clone()).collect(),
            };
            if revealed {
                let correct_set = correct.clone();
                rsx! {
                    ol { class: "ui-quiz-order-review",
                        for (position, id) in order.iter().enumerate() {
                            {
                                let text = items
                                    .iter()
                                    .find(|item| &item.id == id)
                                    .map(|item| item.text.as_str())
                                    .unwrap_or(id.as_str());
                                let placed_right = correct_set.get(position) == Some(id);
                                let class = if placed_right {
                                    "ui-quiz-option ui-quiz-option--correct"
                                } else {
                                    "ui-quiz-option ui-quiz-option--incorrect"
                                };
                                rsx! {
                                    li { key: "{id}", class: "{class}",
                                        span { class: "ui-quiz-option-text", "{text}" }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let sortable_items: Vec<SortableItem> = order
                    .iter()
                    .filter_map(|id| {
                        items
                            .iter()
                            .find(|item| &item.id == id)
                            .map(|item| SortableItem::new(item.id.clone(), item.text.clone()))
                    })
                    .collect();
                rsx! {
                    div { class: "ui-quiz-ordering",
                        SortableList {
                            label: "Arrange in the correct order".to_string(),
                            items: sortable_items,
                            on_reorder: move |next: Vec<String>| {
                                on_answer.call(QuizAnswer::Order(next));
                            },
                        }
                    }
                }
            }
        }
        QuizPrompt::ShortAnswer { accepted } => {
            let value = match answer {
                Some(QuizAnswer::Text(text)) => text.clone(),
                _ => String::new(),
            };
            let model_answer = accepted.first().cloned().unwrap_or_default();
            rsx! {
                div { class: "ui-quiz-short-answer",
                    input {
                        class: "ui-quiz-short-answer-input",
                        r#type: "text",
                        value: "{value}",
                        "aria-label": "Your answer",
                        disabled: revealed,
                        oninput: move |evt| {
                            on_answer.call(QuizAnswer::Text(evt.value()));
                        },
                    }
                    if revealed && !model_answer.is_empty() {
                        p { class: "ui-quiz-model-answer",
                            "Accepted answer: "
                            strong { "{model_answer}" }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// QuizResults
// ---------------------------------------------------------------------------

/// End-of-quiz summary: score gauge, per-question verdict dots, and an
/// optional retry action.
#[component]
pub fn QuizResults(
    correct: usize,
    total: usize,
    #[props(default = "Quiz results".to_string())] label: String,
    #[props(default)] per_question: Vec<bool>,
    on_retry: Option<EventHandler<()>>,
    #[props(default = "Try again".to_string())] retry_label: String,
) -> Element {
    let fraction = if total == 0 {
        0.0
    } else {
        (correct as f32 / total as f32).clamp(0.0, 1.0)
    };
    let percent = (fraction * 100.0).round() as u32;
    let tone = if fraction >= 0.8 {
        ChartTone::Success
    } else if fraction >= 0.5 {
        ChartTone::Primary
    } else {
        ChartTone::Warning
    };

    rsx! {
        section { class: "ui-quiz-results", "aria-label": "{label}",
            DonutGauge {
                label: "{label}: {percent} percent",
                value: fraction,
                display_value: "{percent}%",
                description: "score".to_string(),
                tone,
            }
            div { class: "ui-quiz-results-body",
                p { class: "ui-quiz-results-score",
                    span { class: "ui-tabular", "{correct} of {total}" }
                    " questions correct"
                }
                if !per_question.is_empty() {
                    ol { class: "ui-quiz-results-dots", "aria-label": "Per-question results",
                        for (index, was_correct) in per_question.iter().enumerate() {
                            li {
                                key: "{index}",
                                class: if *was_correct {
                                    "ui-quiz-results-dot ui-quiz-results-dot--correct"
                                } else {
                                    "ui-quiz-results-dot ui-quiz-results-dot--incorrect"
                                },
                                span { class: "visually-hidden",
                                    if *was_correct {
                                        "Question {index + 1}: correct"
                                    } else {
                                        "Question {index + 1}: incorrect"
                                    }
                                }
                            }
                        }
                    }
                }
                if on_retry.is_some() {
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| {
                            if let Some(handler) = &on_retry {
                                handler.call(());
                            }
                        },
                        "{retry_label}"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// QuizTimer
// ---------------------------------------------------------------------------

/// Formats whole seconds as `m:ss` (or `h:mm:ss` past an hour).
fn format_clock(total_seconds: u32) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

/// Whether the timer should enter its warning treatment (≤20% remaining).
fn timer_warning(remaining: u32, total: u32) -> bool {
    total > 0 && remaining * 5 <= total
}

/// A controlled countdown readout (`role="timer"`): the host ticks
/// `remaining_seconds` (so SSR and capture stay deterministic), the
/// component renders the clock, a shrinking track, and a warning treatment
/// in the final 20%.
#[component]
pub fn QuizTimer(
    total_seconds: u32,
    remaining_seconds: u32,
    #[props(default = "Time remaining".to_string())] label: String,
) -> Element {
    let remaining = remaining_seconds.min(total_seconds);
    let warning = timer_warning(remaining, total_seconds);
    let fraction = if total_seconds == 0 {
        0.0
    } else {
        remaining as f32 / total_seconds as f32
    };
    let class = format!(
        "ui-quiz-timer{}",
        if warning {
            " ui-quiz-timer--warning"
        } else {
            ""
        }
    );

    rsx! {
        div {
            class: "{class}",
            role: "timer",
            "aria-label": "{label}",
            span { class: "ui-quiz-timer-clock ui-tabular", "{format_clock(remaining)}" }
            div { class: "ui-quiz-timer-track", "aria-hidden": "true",
                div {
                    class: "ui-quiz-timer-fill",
                    style: "width:{(fraction * 100.0).clamp(0.0, 100.0)}%",
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single() -> QuizQuestion {
        QuizQuestion::new(
            "q1",
            "Pick one",
            QuizPrompt::SingleChoice {
                choices: vec![QuizChoice::new("a", "A"), QuizChoice::new("b", "B")],
                correct: "b".into(),
            },
        )
    }

    #[test]
    fn grades_single_choice() {
        assert_eq!(
            grade_answer(&single(), &QuizAnswer::Choice("b".into())),
            Some(true)
        );
        assert_eq!(
            grade_answer(&single(), &QuizAnswer::Choice("a".into())),
            Some(false)
        );
    }

    #[test]
    fn grades_multi_select_as_set() {
        let q = QuizQuestion::new(
            "q2",
            "Pick all",
            QuizPrompt::MultiSelect {
                choices: vec![
                    QuizChoice::new("a", "A"),
                    QuizChoice::new("b", "B"),
                    QuizChoice::new("c", "C"),
                ],
                correct: vec!["a".into(), "c".into()],
            },
        );
        assert_eq!(
            grade_answer(&q, &QuizAnswer::Choices(vec!["c".into(), "a".into()])),
            Some(true)
        );
        assert_eq!(
            grade_answer(&q, &QuizAnswer::Choices(vec!["a".into()])),
            Some(false)
        );
        assert_eq!(
            grade_answer(
                &q,
                &QuizAnswer::Choices(vec!["a".into(), "b".into(), "c".into()])
            ),
            Some(false)
        );
    }

    #[test]
    fn grades_true_false_and_ordering() {
        let tf = QuizQuestion::new("q3", "T or F", QuizPrompt::TrueFalse { correct: true });
        assert_eq!(grade_answer(&tf, &QuizAnswer::Bool(true)), Some(true));
        assert_eq!(grade_answer(&tf, &QuizAnswer::Bool(false)), Some(false));

        let ord = QuizQuestion::new(
            "q4",
            "Order",
            QuizPrompt::Ordering {
                items: vec![QuizChoice::new("x", "X"), QuizChoice::new("y", "Y")],
                correct: vec!["x".into(), "y".into()],
            },
        );
        assert_eq!(
            grade_answer(&ord, &QuizAnswer::Order(vec!["x".into(), "y".into()])),
            Some(true)
        );
        assert_eq!(
            grade_answer(&ord, &QuizAnswer::Order(vec!["y".into(), "x".into()])),
            Some(false)
        );
    }

    #[test]
    fn grades_short_answer_normalized() {
        let q = QuizQuestion::new(
            "q5",
            "Answer",
            QuizPrompt::ShortAnswer {
                accepted: vec!["The Mitochondria".into(), "mitochondrion".into()],
            },
        );
        assert_eq!(
            grade_answer(&q, &QuizAnswer::Text("  the   mitochondria ".into())),
            Some(true)
        );
        assert_eq!(
            grade_answer(&q, &QuizAnswer::Text("MITOCHONDRION".into())),
            Some(true)
        );
        assert_eq!(
            grade_answer(&q, &QuizAnswer::Text("the nucleus".into())),
            Some(false)
        );
    }

    #[test]
    fn mismatched_answer_shape_is_none() {
        assert_eq!(grade_answer(&single(), &QuizAnswer::Bool(true)), None);
        assert_eq!(grade_answer(&single(), &QuizAnswer::Text("b".into())), None);
    }

    #[test]
    fn normalize_collapses_case_and_whitespace() {
        assert_eq!(normalize_short_answer("  Hello   World "), "hello world");
        assert_eq!(normalize_short_answer(""), "");
    }

    #[test]
    fn toggle_choice_adds_and_removes() {
        let first = toggle_choice(None, "a");
        assert_eq!(first, QuizAnswer::Choices(vec!["a".into()]));
        let second = toggle_choice(Some(&first), "b");
        assert_eq!(second, QuizAnswer::Choices(vec!["a".into(), "b".into()]));
        let third = toggle_choice(Some(&second), "a");
        assert_eq!(third, QuizAnswer::Choices(vec!["b".into()]));
    }

    #[test]
    fn revealed_option_class_marks_correct_and_misses() {
        let correct = vec!["a".to_string()];
        assert_eq!(
            revealed_option_class(&correct, true, "a"),
            " ui-quiz-option--correct"
        );
        assert_eq!(
            revealed_option_class(&correct, false, "a"),
            " ui-quiz-option--correct"
        );
        assert_eq!(
            revealed_option_class(&correct, true, "b"),
            " ui-quiz-option--incorrect"
        );
        assert_eq!(revealed_option_class(&correct, false, "b"), "");
    }

    #[test]
    fn clock_formats_minutes_and_hours() {
        assert_eq!(format_clock(0), "0:00");
        assert_eq!(format_clock(75), "1:15");
        assert_eq!(format_clock(3_661), "1:01:01");
    }

    #[test]
    fn timer_warning_at_twenty_percent() {
        assert!(timer_warning(20, 100));
        assert!(timer_warning(0, 100));
        assert!(!timer_warning(21, 100));
        assert!(!timer_warning(0, 0));
    }
}
