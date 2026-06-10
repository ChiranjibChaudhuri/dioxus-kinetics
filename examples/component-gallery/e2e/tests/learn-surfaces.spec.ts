import { test, expect, type Page, type Locator } from "@playwright/test";
import { mountGallery } from "./_lib/mount.js";

function entryFor(page: Page, name: string): Locator {
  return page
    .locator("article.gallery-entry")
    .filter({ has: page.locator(`h4:text-is("${name}")`) })
    .first();
}

/**
 * Mount and close the open-by-default fixed overlays (Sheet, AssistantPanel)
 * that intercept pointer events aimed at other entries — same workaround as
 * new-surfaces.spec.ts. Trigger buttons inside entries use dispatchEvent
 * because the fixed Toaster demo overlaps parts of the grid.
 */
async function prepare(page: Page) {
  await mountGallery(page);
  const openSheet = page.locator(".ui-sheet[data-state='open']");
  if ((await openSheet.count()) > 0) {
    await openSheet.getByRole("button", { name: "Close" }).click({ force: true });
    await expect(openSheet).toHaveCount(0);
  }
  const assistant = page.locator(".ui-assistant-panel");
  if ((await assistant.count()) > 0) {
    await assistant.getByRole("button", { name: "Close" }).click({ force: true });
    await expect(assistant).toHaveCount(0);
  }
}

test.describe("Course structure", () => {
  test("CourseOutline renders lesson states and reports selection", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "CourseOutline");
    await entry.scrollIntoViewIfNeeded();

    const outline = entry.locator(".ui-course-outline");
    await expect(outline.locator(".ui-course-module")).toHaveCount(2);
    await expect(outline.locator(".ui-course-lesson--completed")).toHaveCount(1);
    await expect(outline.locator(".ui-course-lesson--locked")).toHaveCount(2);

    // Current lesson carries aria-current=step; locked lessons are disabled.
    const current = outline.locator("[aria-current='step']");
    await expect(current).toHaveCount(1);
    await expect(current).toContainText("Lifetimes in practice");
    const locked = outline.locator(".ui-course-lesson--locked button").first();
    await expect(locked).toBeDisabled();

    // Module headers report completion counts.
    await expect(outline.locator(".ui-course-module-count").first()).toHaveText("1 / 3");

    // Selecting an available lesson reports its id to the host.
    await outline.getByRole("button", { name: /Traits and generics/ }).dispatchEvent("click");
    await expect(entry.locator(".gallery-variant-label, .gallery-demo-frame-header span").first())
      .toContainText("traits");
  });

  test("CourseProgressCard and ResumeLearning expose meter semantics", async ({ page }) => {
    await prepare(page);
    const progress = entryFor(page, "CourseProgressCard");
    await progress.scrollIntoViewIfNeeded();
    // 9 of 14 → 64%.
    await expect(progress.locator(".ui-donut-gauge")).toHaveAttribute("aria-valuenow", "64");
    await expect(progress.locator(".ui-course-progress-counts")).toContainText("9 of 14");

    const resume = entryFor(page, "ResumeLearning");
    await resume.scrollIntoViewIfNeeded();
    const bar = resume.locator(".ui-resume-learning-track");
    await expect(bar).toHaveAttribute("aria-valuenow", "45");
    await resume.getByRole("button", { name: "Continue" }).dispatchEvent("click");
    await expect(resume.locator(".gallery-variant-label").first()).toHaveText("Resumed!");
  });
});

test.describe("Assessment", () => {
  test("QuestionCard grades a single-choice answer on reveal", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "QuestionCard");
    await entry.scrollIntoViewIfNeeded();

    const card = entry.locator(".ui-question-card").first();
    const checkButton = entry.getByRole("button", { name: "Check answer" });
    await expect(checkButton).toBeDisabled();

    // Pick the correct option, then check.
    await card.getByRole("radio", { name: /Aliasing and mutability/ }).dispatchEvent("click");
    await expect(checkButton).toBeEnabled();
    await checkButton.dispatchEvent("click");

    await expect(card).toHaveClass(/ui-question-card--correct/);
    await expect(card.locator(".ui-question-verdict")).toHaveText("Correct");
    await expect(card.locator(".ui-question-explanation")).toContainText("mutable reference");
    await expect(card.locator(".ui-quiz-option--correct")).toHaveCount(1);
    // Reveal locks the inputs.
    await expect(card.locator("input[type='radio']").first()).toBeDisabled();
  });

  test("QuestionCard ordering reorders with the keyboard", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "QuestionCard");
    await entry.scrollIntoViewIfNeeded();

    const ordering = entry.locator(".ui-quiz-ordering");
    const labels = ordering.locator(".ui-sortable-label");
    await expect(labels.first()).toHaveText("Parse to AST");

    // Grab the first item and move it down one slot via keyboard.
    const handle = ordering.locator(".ui-sortable-handle").first();
    await handle.focus();
    await handle.press(" ");
    await handle.press("ArrowDown");
    await expect(labels.first()).toHaveText("Borrow check");
  });

  test("QuizResults and QuizTimer expose their states", async ({ page }) => {
    await prepare(page);
    const results = entryFor(page, "QuizResults");
    await results.scrollIntoViewIfNeeded();
    await expect(results.locator(".ui-donut-gauge")).toHaveAttribute("aria-valuetext", "80%");
    await expect(results.locator(".ui-quiz-results-dot")).toHaveCount(10);
    await expect(results.locator(".ui-quiz-results-dot--incorrect")).toHaveCount(2);

    const timer = entryFor(page, "QuizTimer");
    await timer.scrollIntoViewIfNeeded();
    const timers = timer.locator(".ui-quiz-timer");
    await expect(timers).toHaveCount(2);
    await expect(timers.first()).not.toHaveClass(/--warning/);
    await expect(timers.first().locator(".ui-quiz-timer-clock")).toHaveText("3:04");
    await expect(timers.nth(1)).toHaveClass(/ui-quiz-timer--warning/);
    await expect(timers.nth(1).locator(".ui-quiz-timer-clock")).toHaveText("0:38");
  });
});

test.describe("Flashcards", () => {
  test("FlipCard flips, ratings advance the deck", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "FlashcardDeck");
    await entry.scrollIntoViewIfNeeded();

    const deck = entry.locator(".ui-flashcard-deck");
    const card = deck.locator(".ui-flip-card");
    await expect(deck.locator(".ui-flashcard-deck-counter")).toHaveText("1 of 3");
    await expect(card).toHaveAttribute("aria-pressed", "false");
    await expect(deck.locator(".ui-flashcard-deck-hint")).toBeVisible();

    // Flip: ratings replace the hint.
    await card.dispatchEvent("click");
    await expect(card).toHaveClass(/ui-flip-card--flipped/);
    await expect(card).toHaveAttribute("aria-pressed", "true");
    const ratings = deck.locator(".ui-flashcard-rating");
    await expect(ratings).toHaveCount(4);

    // Rating advances to the next card, unflipped.
    await deck.locator(".ui-flashcard-rating--good").dispatchEvent("click");
    await expect(deck.locator(".ui-flashcard-deck-counter")).toHaveText("2 of 3");
    await expect(card).not.toHaveClass(/ui-flip-card--flipped/);
  });
});

test.describe("Gamification", () => {
  test("XpBar exposes progressbar semantics with level text", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "XpBar");
    await entry.scrollIntoViewIfNeeded();

    const bars = entry.locator(".ui-xp-bar");
    await expect(bars).toHaveCount(2);
    const track = bars.first().locator("[role='progressbar']");
    await expect(track).toHaveAttribute("aria-valuenow", "340");
    await expect(track).toHaveAttribute("aria-valuetext", /Level 7/);
    await expect(bars.nth(1)).toHaveClass(/ui-xp-bar--level-up/);
  });

  test("StreakBadge, AchievementUnlock, and Leaderboard render their states", async ({ page }) => {
    await prepare(page);

    const streak = entryFor(page, "StreakBadge");
    await streak.scrollIntoViewIfNeeded();
    await expect(streak.locator(".ui-streak-badge--active")).toHaveAttribute(
      "aria-label",
      "12-day streak",
    );

    const achievement = entryFor(page, "AchievementUnlock");
    await achievement.scrollIntoViewIfNeeded();
    const unlock = achievement.locator(".ui-achievement");
    await expect(unlock).toHaveAttribute("role", "status");
    await expect(unlock).toHaveClass(/ui-achievement--celebrate/);
    await expect(unlock.locator(".ui-achievement-particle")).toHaveCount(12);
    await expect(unlock).toContainText("Week-long streak");

    const leaderboard = entryFor(page, "Leaderboard");
    await leaderboard.scrollIntoViewIfNeeded();
    const rows = leaderboard.locator(".ui-leaderboard-row");
    await expect(rows).toHaveCount(5);
    await expect(rows.first().locator(".ui-leaderboard-rank")).toHaveClass(/--gold/);
    const you = leaderboard.locator(".ui-leaderboard-row--you");
    await expect(you).toHaveCount(1);
    await expect(you.locator(".ui-leaderboard-you-tag")).toHaveText("You");
  });
});

test.describe("Certificate", () => {
  test("CertificateCard names the full credential for assistive tech", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "CertificateCard");
    await entry.scrollIntoViewIfNeeded();

    const certificate = entry.locator(".ui-certificate");
    await expect(certificate).toHaveAttribute("role", "img");
    await expect(certificate).toHaveAttribute(
      "aria-label",
      /Ada Lovelace.*Rust Fundamentals.*Kinetics Academy/,
    );
    await expect(certificate.locator(".ui-certificate-recipient")).toHaveText("Ada Lovelace");
    await expect(certificate.locator(".ui-certificate-credential")).toContainText("KA-2026-0142");
  });
});
