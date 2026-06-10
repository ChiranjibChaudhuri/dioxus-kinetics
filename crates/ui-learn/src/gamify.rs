use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// XpBar
// ---------------------------------------------------------------------------

/// Progress fraction toward the next level, NaN/zero-safe.
fn xp_fraction(current: u32, next_level: u32) -> f32 {
    if next_level == 0 {
        0.0
    } else {
        (current as f32 / next_level as f32).clamp(0.0, 1.0)
    }
}

/// Experience progress toward the next level: level chip, XP counts, and a
/// fill bar. Set `leveled_up` for the render after a level-up to play the
/// celebration pulse (a CSS animation, stilled under reduced motion).
#[component]
pub fn XpBar(
    level: u32,
    current_xp: u32,
    next_level_xp: u32,
    #[props(default = "Experience".to_string())] label: String,
    #[props(default)] leveled_up: bool,
) -> Element {
    let fraction = xp_fraction(current_xp, next_level_xp);
    let percent = (fraction * 100.0).round() as u32;
    let class = format!(
        "ui-xp-bar{}",
        if leveled_up {
            " ui-xp-bar--level-up"
        } else {
            ""
        }
    );

    rsx! {
        div { class: "{class}",
            span { class: "ui-xp-bar-level", "aria-hidden": "true", "Lv {level}" }
            div { class: "ui-xp-bar-body",
                div {
                    class: "ui-xp-bar-track",
                    role: "progressbar",
                    "aria-valuemin": "0",
                    "aria-valuemax": "{next_level_xp}",
                    "aria-valuenow": "{current_xp.min(next_level_xp)}",
                    "aria-valuetext": "Level {level}, {current_xp} of {next_level_xp} experience",
                    "aria-label": "{label}",
                    div { class: "ui-xp-bar-fill", style: "width:{percent}%" }
                }
                span { class: "ui-xp-bar-counts ui-tabular", "aria-hidden": "true",
                    "{current_xp} / {next_level_xp} XP"
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// StreakBadge
// ---------------------------------------------------------------------------

/// A consecutive-days streak chip. `active` means today already counts —
/// the flame fills in and gently pulses (reduced-motion gated); inactive
/// renders the outline flame as a nudge.
#[component]
pub fn StreakBadge(days: u32, #[props(default)] active: bool) -> Element {
    let class = format!(
        "ui-streak-badge{}",
        if active {
            " ui-streak-badge--active"
        } else {
            ""
        }
    );

    rsx! {
        span { class: "{class}", role: "img", "aria-label": "{days}-day streak",
            svg {
                class: "ui-streak-badge-flame",
                view_box: "0 0 16 16",
                "aria-hidden": "true",
                path {
                    d: "M8 1.5c.4 2.2-.9 3.4-1.9 4.5C5 7.2 4.2 8.3 4.2 10a3.8 3.8 0 0 0 7.6 0c0-1.3-.5-2.3-1.1-3.2-.3.7-.8 1.2-1.4 1.4.4-2.6-.2-5-1.3-6.7Z",
                    fill: if active { "currentColor" } else { "none" },
                    stroke: "currentColor",
                    stroke_width: "1.4",
                    stroke_linejoin: "round",
                }
            }
            span { class: "ui-streak-badge-count ui-tabular", "aria-hidden": "true", "{days}" }
        }
    }
}

// ---------------------------------------------------------------------------
// AchievementUnlock
// ---------------------------------------------------------------------------

/// Number of CSS particle spans in the celebration burst. Their trajectories
/// are pure functions of the index (no randomness), so renders are
/// deterministic and capture-safe.
const PARTICLE_COUNT: usize = 12;

/// An achievement-unlocked card with a deterministic particle burst.
/// Announced via `role="status"` so the unlock is read without stealing
/// focus; the burst and badge pop are CSS animations disabled under reduced
/// motion (a static highlight ring remains). Set `celebrate: false` for the
/// quiet variant in sober products.
#[component]
pub fn AchievementUnlock(
    title: String,
    #[props(default)] description: String,
    #[props(default = true)] visible: bool,
    #[props(default = true)] celebrate: bool,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    if !visible {
        return rsx! {};
    }
    let class = format!(
        "ui-achievement{}",
        if celebrate {
            " ui-achievement--celebrate"
        } else {
            ""
        }
    );

    rsx! {
        div { class: "{class}", role: "status",
            div { class: "ui-achievement-burst", "aria-hidden": "true",
                for index in 0..PARTICLE_COUNT {
                    span {
                        key: "{index}",
                        class: "ui-achievement-particle",
                        style: "--ui-particle-angle:{index * 360 / PARTICLE_COUNT}deg;--ui-particle-delay:{index * 30}ms",
                    }
                }
            }
            span { class: "ui-achievement-medal", "aria-hidden": "true",
                svg { view_box: "0 0 16 16",
                    circle { cx: "8", cy: "6.5", r: "4.5", fill: "currentColor" }
                    path {
                        d: "M5.5 10.5 4 15l4-2 4 2-1.5-4.5",
                        fill: "currentColor",
                        opacity: "0.55",
                    }
                }
            }
            div { class: "ui-achievement-body",
                p { class: "ui-achievement-eyebrow", "Achievement unlocked" }
                strong { class: "ui-achievement-title", "{title}" }
                if !description.is_empty() {
                    p { class: "ui-achievement-description", "{description}" }
                }
            }
            if on_dismiss.is_some() {
                button {
                    class: "ui-button ui-button--ghost ui-achievement-dismiss",
                    r#type: "button",
                    "aria-label": "Dismiss",
                    onclick: move |_| {
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    },
                    "✕"
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Leaderboard
// ---------------------------------------------------------------------------

/// One leaderboard row. `score` is a preformatted string so hosts control
/// units ("1,250 XP", "98%"); `highlight` marks the viewing learner's row.
#[derive(Clone, Debug, PartialEq)]
pub struct LeaderboardEntry {
    pub name: String,
    pub score: String,
    pub highlight: bool,
}

impl LeaderboardEntry {
    pub fn new(name: impl Into<String>, score: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            score: score.into(),
            highlight: false,
        }
    }

    pub fn highlighted(mut self) -> Self {
        self.highlight = true;
        self
    }
}

/// Medal modifier for 1-based ranks; podium places get metal tints.
fn rank_class(rank: usize) -> &'static str {
    match rank {
        1 => " ui-leaderboard-rank--gold",
        2 => " ui-leaderboard-rank--silver",
        3 => " ui-leaderboard-rank--bronze",
        _ => "",
    }
}

/// An ordered standings list. Ranks derive from entry order (host sorts);
/// the top three get podium treatments and the `highlight` row is visually
/// pinned as "you".
#[component]
pub fn Leaderboard(label: String, entries: Vec<LeaderboardEntry>) -> Element {
    rsx! {
        ol { class: "ui-leaderboard", "aria-label": "{label}",
            for (index, entry) in entries.iter().enumerate() {
                {
                    let rank = index + 1;
                    let row_class = format!(
                        "ui-leaderboard-row{}",
                        if entry.highlight { " ui-leaderboard-row--you" } else { "" }
                    );
                    rsx! {
                        li { key: "{index}", class: "{row_class}",
                            span { class: "ui-leaderboard-rank ui-tabular{rank_class(rank)}", "{rank}" }
                            span { class: "ui-leaderboard-name",
                                "{entry.name}"
                                if entry.highlight {
                                    span { class: "ui-leaderboard-you-tag", "You" }
                                }
                            }
                            span { class: "ui-leaderboard-score ui-tabular", "{entry.score}" }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xp_fraction_clamps_and_handles_zero() {
        assert_eq!(xp_fraction(50, 100), 0.5);
        assert_eq!(xp_fraction(150, 100), 1.0);
        assert_eq!(xp_fraction(10, 0), 0.0);
    }

    #[test]
    fn rank_class_covers_podium_only() {
        assert_eq!(rank_class(1), " ui-leaderboard-rank--gold");
        assert_eq!(rank_class(2), " ui-leaderboard-rank--silver");
        assert_eq!(rank_class(3), " ui-leaderboard-rank--bronze");
        assert_eq!(rank_class(4), "");
    }

    #[test]
    fn leaderboard_entry_builder() {
        let entry = LeaderboardEntry::new("Ada", "1,250 XP").highlighted();
        assert!(entry.highlight);
        assert_eq!(entry.score, "1,250 XP");
    }
}
