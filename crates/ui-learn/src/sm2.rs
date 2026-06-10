//! SM-2-lite spaced-repetition scheduling: the Anki-style four-rating model
//! reduced to a pure function over `(interval, ease, repetitions)`. Hosts
//! store the returned state per card and compute the due date themselves
//! (`now + interval_days`), keeping the components clock-free.

use crate::flashcards::ReviewRating;

/// Per-card scheduling state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReviewState {
    /// Days until the next review. `0.0` means "again this session".
    pub interval_days: f32,
    /// Ease factor; SM-2's floor of 1.3 applies.
    pub ease: f32,
    /// Consecutive successful reviews.
    pub repetitions: u32,
}

impl Default for ReviewState {
    fn default() -> Self {
        Self {
            interval_days: 0.0,
            ease: 2.5,
            repetitions: 0,
        }
    }
}

const MIN_EASE: f32 = 1.3;

/// Advances a card's schedule for one review.
///
/// - `Again` — lapse: repetitions reset, the card comes back this session,
///   and ease drops 0.2.
/// - `Hard` — barely passed: interval grows only 1.2×, ease drops 0.15.
/// - `Good` — passed: first review graduates to 1 day, the second to 6 days,
///   then interval × ease.
/// - `Easy` — trivial: interval × ease × 1.3 bonus, ease gains 0.15.
pub fn next_review(state: ReviewState, rating: ReviewRating) -> ReviewState {
    let ease = state.ease.max(MIN_EASE);
    match rating {
        ReviewRating::Again => ReviewState {
            interval_days: 0.0,
            ease: (ease - 0.2).max(MIN_EASE),
            repetitions: 0,
        },
        ReviewRating::Hard => ReviewState {
            interval_days: (state.interval_days * 1.2).max(1.0),
            ease: (ease - 0.15).max(MIN_EASE),
            repetitions: state.repetitions + 1,
        },
        ReviewRating::Good => ReviewState {
            interval_days: match state.repetitions {
                0 => 1.0,
                1 => 6.0,
                _ => state.interval_days * ease,
            },
            ease,
            repetitions: state.repetitions + 1,
        },
        ReviewRating::Easy => ReviewState {
            interval_days: match state.repetitions {
                0 => 4.0,
                _ => state.interval_days * ease * 1.3,
            },
            ease: ease + 0.15,
            repetitions: state.repetitions + 1,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn again_lapses_and_floors_ease() {
        let lapsed = next_review(
            ReviewState {
                interval_days: 30.0,
                ease: 1.4,
                repetitions: 5,
            },
            ReviewRating::Again,
        );
        assert_eq!(lapsed.interval_days, 0.0);
        assert_eq!(lapsed.repetitions, 0);
        assert_eq!(lapsed.ease, MIN_EASE);
    }

    #[test]
    fn good_graduates_one_then_six_then_multiplies() {
        let first = next_review(ReviewState::default(), ReviewRating::Good);
        assert_eq!(first.interval_days, 1.0);
        let second = next_review(first, ReviewRating::Good);
        assert_eq!(second.interval_days, 6.0);
        let third = next_review(second, ReviewRating::Good);
        assert_eq!(third.interval_days, 6.0 * 2.5);
        assert_eq!(third.repetitions, 3);
    }

    #[test]
    fn hard_grows_slowly_and_drops_ease() {
        let state = ReviewState {
            interval_days: 10.0,
            ease: 2.5,
            repetitions: 3,
        };
        let next = next_review(state, ReviewRating::Hard);
        assert_eq!(next.interval_days, 12.0);
        assert_eq!(next.ease, 2.35);
    }

    #[test]
    fn easy_bonuses_interval_and_ease() {
        let state = ReviewState {
            interval_days: 10.0,
            ease: 2.5,
            repetitions: 3,
        };
        let next = next_review(state, ReviewRating::Easy);
        assert_eq!(next.interval_days, 10.0 * 2.5 * 1.3);
        assert_eq!(next.ease, 2.65);

        let fresh = next_review(ReviewState::default(), ReviewRating::Easy);
        assert_eq!(fresh.interval_days, 4.0);
    }

    #[test]
    fn ease_never_drops_below_floor() {
        let mut state = ReviewState::default();
        for _ in 0..20 {
            state = next_review(state, ReviewRating::Hard);
        }
        assert!(state.ease >= MIN_EASE);
    }
}
