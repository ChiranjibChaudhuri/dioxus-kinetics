#![forbid(unsafe_code)]

//! Learning-management components for the kinetics catalog.
//!
//! Neutral vocabulary (course / module / lesson / score) so corporate
//! training, higher-ed, and consumer products can all adopt it; the louder
//! gamification surfaces are separate components a product simply doesn't
//! mount if it wants a sober tone. Everything is controlled — hosts own all
//! state — and every animation is CSS-driven, deterministic, and gated for
//! reduced motion.

mod certificate;
mod course;
mod flashcards;
mod gamify;
mod quiz;
mod sm2;

pub use certificate::CertificateCard;
pub use course::{
    course_progress, CourseLesson, CourseModule, CourseOutline, CourseProgressCard, LessonState,
    ResumeLearning,
};
pub use flashcards::{Flashcard, FlashcardDeck, FlipCard, ReviewRating};
pub use gamify::{AchievementUnlock, Leaderboard, LeaderboardEntry, StreakBadge, XpBar};
pub use quiz::{
    grade_answer, normalize_short_answer, QuestionCard, QuizAnswer, QuizChoice, QuizPrompt,
    QuizQuestion, QuizResults, QuizTimer,
};
pub use sm2::{next_review, ReviewState};
