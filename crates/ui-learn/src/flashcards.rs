use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Vocabulary
// ---------------------------------------------------------------------------

/// One card: prompt on the front, answer on the back.
#[derive(Clone, Debug, PartialEq)]
pub struct Flashcard {
    pub id: String,
    pub front: String,
    pub back: String,
}

impl Flashcard {
    pub fn new(id: impl Into<String>, front: impl Into<String>, back: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            front: front.into(),
            back: back.into(),
        }
    }
}

/// Anki-style review rating, fed to [`crate::next_review`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReviewRating {
    Again,
    Hard,
    Good,
    Easy,
}

impl ReviewRating {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Again => "Again",
            Self::Hard => "Hard",
            Self::Good => "Good",
            Self::Easy => "Easy",
        }
    }

    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Again => "again",
            Self::Hard => "hard",
            Self::Good => "good",
            Self::Easy => "easy",
        }
    }

    pub const ALL: [ReviewRating; 4] = [Self::Again, Self::Hard, Self::Good, Self::Easy];
}

/// "3 of 20" session counter, clamped one-based.
fn session_counter(index: usize, count: usize) -> String {
    if count == 0 {
        return "0 of 0".to_string();
    }
    format!("{} of {}", index.min(count - 1) + 1, count)
}

// ---------------------------------------------------------------------------
// FlipCard
// ---------------------------------------------------------------------------

/// A two-sided card with a 3D flip. The whole card is a toggle button
/// (`aria-pressed` mirrors `flipped`); the hidden face is `aria-hidden` and
/// inert. Under reduced motion the rotation is replaced by an instant
/// swap — same states, no 3D travel.
#[component]
pub fn FlipCard(
    front: String,
    back: String,
    flipped: bool,
    on_flip: EventHandler<bool>,
    #[props(default = "Flashcard, activate to flip".to_string())] label: String,
) -> Element {
    let class = format!(
        "ui-flip-card{}",
        if flipped {
            " ui-flip-card--flipped"
        } else {
            ""
        }
    );

    rsx! {
        button {
            class: "{class}",
            r#type: "button",
            "aria-label": "{label}",
            "aria-pressed": if flipped { "true" } else { "false" },
            onclick: move |_| on_flip.call(!flipped),
            span { class: "ui-flip-card-inner",
                span {
                    class: "ui-flip-card-face ui-flip-card-face--front",
                    "aria-hidden": if flipped { "true" } else { "false" },
                    "{front}"
                }
                span {
                    class: "ui-flip-card-face ui-flip-card-face--back",
                    "aria-hidden": if flipped { "false" } else { "true" },
                    "{back}"
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// FlashcardDeck
// ---------------------------------------------------------------------------

/// A review session over a deck of cards: session counter, the current
/// [`FlipCard`], and — once the answer is showing — the four rating buttons.
///
/// Controlled: the host owns `index` and `flipped`, advances on `on_rate`
/// (typically by feeding the rating to [`crate::next_review`] and moving to
/// the next due card), and resets `flipped` between cards.
#[component]
pub fn FlashcardDeck(
    cards: Vec<Flashcard>,
    index: usize,
    flipped: bool,
    on_flip: EventHandler<bool>,
    on_rate: EventHandler<ReviewRating>,
    #[props(default = "Flashcards".to_string())] label: String,
) -> Element {
    let count = cards.len();
    if count == 0 {
        return rsx! {
            section { class: "ui-flashcard-deck ui-flashcard-deck--empty", "aria-label": "{label}",
                p { class: "ui-flashcard-deck-empty", "No cards due. Nice work!" }
            }
        };
    }
    let index = index.min(count - 1);
    let card = &cards[index];

    rsx! {
        section { class: "ui-flashcard-deck", "aria-label": "{label}",
            p { class: "ui-flashcard-deck-counter ui-tabular", "{session_counter(index, count)}" }
            FlipCard {
                front: card.front.clone(),
                back: card.back.clone(),
                flipped,
                on_flip: move |next: bool| on_flip.call(next),
                label: "Card {index + 1}: activate to flip".to_string(),
            }
            div { class: "ui-flashcard-ratings",
                if flipped {
                    for rating in ReviewRating::ALL {
                        button {
                            key: "{rating.label()}",
                            class: "ui-flashcard-rating ui-flashcard-rating--{rating.class_suffix()}",
                            r#type: "button",
                            onclick: move |_| on_rate.call(rating),
                            "{rating.label()}"
                        }
                    }
                } else {
                    p { class: "ui-flashcard-deck-hint", "Flip the card to rate your recall." }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_labels_and_classes() {
        assert_eq!(ReviewRating::Again.label(), "Again");
        assert_eq!(ReviewRating::Easy.class_suffix(), "easy");
        assert_eq!(ReviewRating::ALL.len(), 4);
    }

    #[test]
    fn session_counter_is_clamped_one_based() {
        assert_eq!(session_counter(0, 20), "1 of 20");
        assert_eq!(session_counter(19, 20), "20 of 20");
        assert_eq!(session_counter(99, 20), "20 of 20");
        assert_eq!(session_counter(0, 0), "0 of 0");
    }
}
