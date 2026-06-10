use dioxus::prelude::*;
use kinetics::prelude::*;

// ---------------------------------------------------------------------------
// CourseOutline
// ---------------------------------------------------------------------------

pub fn course_outline_preview() -> Element {
    rsx! { CourseOutlinePreviewBody {} }
}

#[component]
fn CourseOutlinePreviewBody() -> Element {
    let mut selected = use_signal(String::new);
    let modules = vec![
        CourseModule::new(
            "rust-basics",
            "Module 1 · Foundations",
            vec![
                CourseLesson::new("ownership", "Ownership & borrowing")
                    .with_duration("12 min")
                    .with_state(LessonState::Completed),
                CourseLesson::new("lifetimes", "Lifetimes in practice")
                    .with_duration("18 min")
                    .with_state(LessonState::Current),
                CourseLesson::new("traits", "Traits and generics").with_duration("15 min"),
            ],
        ),
        CourseModule::new(
            "rust-async",
            "Module 2 · Async",
            vec![
                CourseLesson::new("futures", "Futures from scratch")
                    .with_duration("20 min")
                    .with_state(LessonState::Locked),
                CourseLesson::new("tokio", "The Tokio runtime")
                    .with_duration("16 min")
                    .with_state(LessonState::Locked),
            ],
        ),
    ];

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label",
                    if selected.read().is_empty() {
                        "Completed, current, available, and locked lessons"
                    } else {
                        "Selected lesson: {selected}"
                    }
                }
            }
            CourseOutline {
                label: "Rust fundamentals curriculum".to_string(),
                modules,
                on_select: move |id: String| selected.set(id),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CourseProgressCard + ResumeLearning
// ---------------------------------------------------------------------------

pub fn course_progress_card_preview() -> Element {
    rsx! {
        CourseProgressCard {
            title: "Rust fundamentals".to_string(),
            completed: 9,
            total: 14,
            time_remaining: "1 h 40 min".to_string(),
            trend: vec![1.0, 2.0, 1.0, 3.0, 2.0, 4.0, 3.0],
        }
    }
}

pub fn resume_learning_preview() -> Element {
    rsx! { ResumeLearningPreviewBody {} }
}

#[component]
fn ResumeLearningPreviewBody() -> Element {
    let mut resumed = use_signal(|| false);
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label",
                    if *resumed.read() { "Resumed!" } else { "Pick up where you left off" }
                }
                ResumeLearning {
                    course: "Rust fundamentals".to_string(),
                    lesson: "Lifetimes in practice".to_string(),
                    progress: 0.45,
                    on_resume: move |_| resumed.set(true),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// QuestionCard
// ---------------------------------------------------------------------------

pub fn question_card_preview() -> Element {
    rsx! { QuestionCardPreviewBody {} }
}

#[component]
fn QuestionCardPreviewBody() -> Element {
    let mut answer = use_signal(|| None::<QuizAnswer>);
    let mut revealed = use_signal(|| false);
    let mut order_answer = use_signal(|| None::<QuizAnswer>);

    let choice_question = QuizQuestion::new(
        "borrowck",
        "What does the borrow checker enforce at compile time?",
        QuizPrompt::SingleChoice {
            choices: vec![
                QuizChoice::new("gc", "Garbage collection pauses"),
                QuizChoice::new("aliasing", "Aliasing and mutability rules"),
                QuizChoice::new("layout", "Struct memory layout"),
            ],
            correct: "aliasing".into(),
        },
    )
    .with_explanation("Exactly one mutable reference, or any number of shared ones — never both.");

    let ordering_question = QuizQuestion::new(
        "pipeline",
        "Order the stages of a Rust build",
        QuizPrompt::Ordering {
            items: vec![
                QuizChoice::new("parse", "Parse to AST"),
                QuizChoice::new("borrow", "Borrow check"),
                QuizChoice::new("codegen", "LLVM codegen"),
            ],
            correct: vec!["parse".into(), "borrow".into(), "codegen".into()],
        },
    );

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Single choice · answer then check" }
                QuestionCard {
                    question: choice_question,
                    answer: answer.read().clone(),
                    revealed: *revealed.read(),
                    counter: "Question 1 of 2".to_string(),
                    on_answer: move |next: QuizAnswer| answer.set(Some(next)),
                }
                button {
                    class: "ui-button ui-button--primary",
                    r#type: "button",
                    disabled: answer.read().is_none() || *revealed.read(),
                    onclick: move |_| revealed.set(true),
                    "Check answer"
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Ordering · drag or keyboard-reorder" }
                QuestionCard {
                    question: ordering_question,
                    answer: order_answer.read().clone(),
                    revealed: false,
                    counter: "Question 2 of 2".to_string(),
                    on_answer: move |next: QuizAnswer| order_answer.set(Some(next)),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// QuizResults + QuizTimer
// ---------------------------------------------------------------------------

pub fn quiz_results_preview() -> Element {
    rsx! {
        QuizResults {
            correct: 8,
            total: 10,
            per_question: vec![true, true, false, true, true, true, false, true, true, true],
            on_retry: move |_| {},
        }
    }
}

pub fn quiz_timer_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Plenty of time" }
                QuizTimer { total_seconds: 300, remaining_seconds: 184 }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Final 20% · warning treatment" }
                QuizTimer { total_seconds: 300, remaining_seconds: 38 }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// FlashcardDeck
// ---------------------------------------------------------------------------

pub fn flashcard_deck_preview() -> Element {
    rsx! { FlashcardDeckPreviewBody {} }
}

#[component]
fn FlashcardDeckPreviewBody() -> Element {
    let mut index = use_signal(|| 0usize);
    let mut flipped = use_signal(|| false);

    let cards = vec![
        Flashcard::new(
            "c1",
            "What does `&mut T` guarantee?",
            "Exclusive access: no other live references.",
        ),
        Flashcard::new(
            "c2",
            "What is a `Future` until polled?",
            "Inert — it does nothing until an executor polls it.",
        ),
        Flashcard::new(
            "c3",
            "`Rc<T>` vs `Arc<T>`?",
            "Arc is atomically reference-counted and thread-safe.",
        ),
    ];
    let count = cards.len();

    rsx! {
        FlashcardDeck {
            cards,
            index: *index.read(),
            flipped: *flipped.read(),
            on_flip: move |next: bool| flipped.set(next),
            on_rate: move |_rating: ReviewRating| {
                flipped.set(false);
                let next = (index() + 1) % count;
                index.set(next);
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Gamification
// ---------------------------------------------------------------------------

pub fn xp_bar_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Mid-level progress" }
                XpBar { level: 7, current_xp: 340, next_level_xp: 500 }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Just levelled up · pulse on mount" }
                XpBar { level: 8, current_xp: 20, next_level_xp: 650, leveled_up: true }
            }
        }
    }
}

pub fn streak_badge_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Active · today counted" }
                StreakBadge { days: 12, active: true }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "At risk · nudge to practice" }
                StreakBadge { days: 12 }
            }
        }
    }
}

pub fn achievement_unlock_preview() -> Element {
    rsx! { AchievementPreviewBody {} }
}

#[component]
fn AchievementPreviewBody() -> Element {
    // Remounting via a bumped key replays the deterministic burst.
    let mut replay = use_signal(|| 0u32);
    let key = *replay.read();

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Deterministic particle burst" }
                button {
                    class: "ui-button ui-button--secondary",
                    r#type: "button",
                    onclick: move |_| replay.set(key + 1),
                    "Replay"
                }
            }
            div { key: "{key}",
                AchievementUnlock {
                    title: "Week-long streak".to_string(),
                    description: "Practised seven days in a row.".to_string(),
                }
            }
        }
    }
}

pub fn leaderboard_preview() -> Element {
    rsx! {
        Leaderboard {
            label: "Weekly standings".to_string(),
            entries: vec![
                LeaderboardEntry::new("Priya N.", "2,180 XP"),
                LeaderboardEntry::new("Marcus T.", "1,940 XP"),
                LeaderboardEntry::new("Sofia R.", "1,720 XP"),
                LeaderboardEntry::new("You", "1,510 XP").highlighted(),
                LeaderboardEntry::new("Daniel K.", "1,470 XP"),
            ],
        }
    }
}

// ---------------------------------------------------------------------------
// CertificateCard
// ---------------------------------------------------------------------------

pub fn certificate_card_preview() -> Element {
    rsx! {
        CertificateCard {
            recipient: "Ada Lovelace".to_string(),
            course: "Rust Fundamentals: Ownership to Async".to_string(),
            date: "9 June 2026".to_string(),
            issuer: "Kinetics Academy".to_string(),
            signature_name: "Grace Hopper".to_string(),
            credential_id: "KA-2026-0142".to_string(),
        }
    }
}
