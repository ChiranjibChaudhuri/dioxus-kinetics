import { test, expect, type Page, type Locator } from "@playwright/test";
import { mountGallery } from "./_lib/mount.js";

function entryFor(page: Page, name: string): Locator {
  return page
    .locator("article.gallery-entry")
    .filter({ has: page.locator(`h4:text-is("${name}")`) })
    .first();
}

/**
 * mountGallery dismisses the open-by-default modal overlays (Dialog, Sheet,
 * AssistantPanel) before returning. Buttons inside entries are still
 * force-clicked / event-dispatched below for the same reason mount.ts
 * force-clicks the preference radios: surrounding tile prose and the fixed
 * Toaster demo can intercept the hit test.
 */
async function prepare(page: Page) {
  await mountGallery(page);
}

test.describe("Charts", () => {
  test("LineChart renders both series, the legend, and the SR data table", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "LineChart");
    await entry.scrollIntoViewIfNeeded();

    const chart = entry.locator(".ui-chart--line").first();
    await expect(chart).toBeVisible();
    await expect(chart.locator(".ui-chart-line")).toHaveCount(2);
    await expect(chart.locator(".ui-chart-area")).toHaveCount(2);
    await expect(chart.locator(".ui-chart-legend-item")).toHaveCount(2);
    await expect(chart.locator(".ui-chart-legend")).toContainText("Revenue");
    await expect(chart.locator(".ui-chart-legend")).toContainText("Forecast");

    // Accessible name on the SVG + mirrored data table for screen readers.
    await expect(chart.locator("svg[role='img']")).toHaveAttribute(
      "aria-label",
      /Monthly revenue/,
    );
    const table = chart.locator("table.visually-hidden");
    await expect(table).toHaveCount(1);
    await expect(table.locator("th[scope='row']")).toHaveCount(2);
  });

  test("BarChart renders grouped bars with series colors", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "BarChart");
    await entry.scrollIntoViewIfNeeded();

    const chart = entry.locator(".ui-chart--bar").first();
    await expect(chart).toBeVisible();
    // 2 series × 4 quarters.
    await expect(chart.locator(".ui-chart-bar")).toHaveCount(8);
    await expect(chart.locator(".ui-chart-bar.ui-chart-series--1")).toHaveCount(4);
    await expect(chart.locator(".ui-chart-bar.ui-chart-series--2")).toHaveCount(4);
  });

  test("DonutGauge exposes meter semantics", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "DonutGauge");
    await entry.scrollIntoViewIfNeeded();

    const gauge = entry.locator(".ui-donut-gauge").first();
    await expect(gauge).toBeVisible();
    await expect(gauge).toHaveAttribute("role", "meter");
    await expect(gauge).toHaveAttribute("aria-valuenow", "72");
    await expect(gauge).toContainText("72%");

    // The custom display value variant reports its text via aria-valuetext.
    const uptime = entry.locator(".ui-donut-gauge--success").first();
    await expect(uptime).toHaveAttribute("aria-valuetext", "99.9%");
  });

  test("Sparkline is decorative without a label and named with one", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "Sparkline");
    await entry.scrollIntoViewIfNeeded();

    const labelled = entry.locator(".ui-sparkline--success").first();
    await expect(labelled).toHaveAttribute("role", "img");
    await expect(labelled).toHaveAttribute("aria-label", /Weekly active users/);

    const decorative = entry.locator(".ui-sparkline--danger").first();
    await expect(decorative).toHaveAttribute("aria-hidden", "true");
  });
});

test.describe("Sortable surfaces", () => {
  test("SortableList reorders with the keyboard and announces moves", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "SortableList");
    await entry.scrollIntoViewIfNeeded();

    const labels = entry.locator(".ui-sortable-label");
    await expect(labels.first()).toHaveText("Triage inbox");

    // Grab the first row's handle, move it down one slot, drop it.
    // Keyboard-only: focus instead of click so no overlay can intercept.
    const handle = entry.locator(".ui-sortable-handle").first();
    await handle.focus();
    await handle.press(" ");
    await expect(handle).toHaveAttribute("aria-pressed", "true");
    await expect(entry.locator("[aria-live='assertive']")).toContainText(/grabbed/);

    await handle.press("ArrowDown");
    await expect(labels.first()).toHaveText("Review escalations");
    await expect(entry.locator("[aria-live='assertive']")).toContainText(/position 2 of 4/);
  });

  test("SortableList Escape restores the order from grab time", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "SortableList");
    await entry.scrollIntoViewIfNeeded();

    const labels = entry.locator(".ui-sortable-label");
    await expect(labels.first()).toHaveText("Triage inbox");

    // Track the grabbed row's handle by its accessible name — after the move
    // it is no longer the list's first handle.
    const handle = entry.getByRole("button", { name: /Reorder Triage inbox/ });
    await handle.focus();
    await handle.press(" ");
    await handle.press("ArrowDown");
    await expect(labels.first()).toHaveText("Review escalations");
    await handle.press("Escape");
    await expect(labels.first()).toHaveText("Triage inbox");
  });

  test("KanbanBoard moves a grabbed card across columns with arrows", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "KanbanBoard");
    await entry.scrollIntoViewIfNeeded();

    const columns = entry.locator(".ui-kanban-column");
    await expect(columns).toHaveCount(3);
    await expect(columns.nth(0).locator(".ui-kanban-card-surface")).toHaveCount(2);
    await expect(columns.nth(1).locator(".ui-kanban-card-surface")).toHaveCount(1);

    // Grab the first backlog card and send it right into "In progress".
    const card = columns.nth(0).locator(".ui-kanban-card-surface").first();
    await card.focus();
    await card.press(" ");
    await expect(card).toHaveAttribute("aria-pressed", "true");
    await card.press("ArrowRight");

    await expect(columns.nth(0).locator(".ui-kanban-card-surface")).toHaveCount(1);
    await expect(columns.nth(1).locator(".ui-kanban-card-surface")).toHaveCount(2);
    await expect(columns.nth(1)).toContainText("Audit color tokens");
    await expect(entry.locator("[aria-live='assertive']")).toContainText(/In progress/);
  });
});

test.describe("Tour", () => {
  test("walks the steps, anchors the spotlight, and dismisses on Escape", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "Tour");
    await entry.scrollIntoViewIfNeeded();

    // dispatchEvent: the gallery's fixed Toaster demo overlaps this entry's
    // header, so a pointer click never reaches the button. The component
    // logic under test is unaffected by gallery page stacking.
    await entry.getByRole("button", { name: "Start tour" }).dispatchEvent("click");

    const panel = page.locator(".ui-tour-panel");
    await expect(panel).toBeVisible();
    await expect(panel).toHaveAttribute("role", "dialog");
    await expect(panel).toContainText("Step 1 of 3");
    await expect(panel).toContainText("Compose anywhere");

    // The overlay measured the target and anchored the cutout.
    const overlay = page.locator(".ui-spotlight-overlay");
    await expect(overlay).toHaveAttribute("data-anchored", "true");
    await expect(page.locator(".ui-spotlight-cutout")).toBeVisible();

    await panel.getByRole("button", { name: "Next" }).click();
    await expect(panel).toContainText("Step 2 of 3");
    await expect(panel).toContainText("Refine the view");

    await panel.getByRole("button", { name: "Back" }).click();
    await expect(panel).toContainText("Step 1 of 3");

    await panel.press("Escape");
    await expect(panel).toHaveCount(0);
    await expect(overlay).toHaveCount(0);
  });

  test("final centered step closes with Done", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "Tour");
    await entry.scrollIntoViewIfNeeded();

    await entry.getByRole("button", { name: "Start tour" }).dispatchEvent("click");
    const panel = page.locator(".ui-tour-panel");
    await panel.getByRole("button", { name: "Next" }).click();
    await panel.getByRole("button", { name: "Next" }).click();

    await expect(panel).toContainText("Step 3 of 3");
    // Targetless step renders centered.
    await expect(panel).toHaveClass(/ui-tour-panel--center/);

    await panel.getByRole("button", { name: "Done" }).click();
    await expect(panel).toHaveCount(0);
  });
});

test.describe("Voice surfaces", () => {
  test("Waveform renders one bar per level and an active variant", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "Waveform");
    await entry.scrollIntoViewIfNeeded();

    const traces = entry.locator(".ui-waveform");
    await expect(traces).toHaveCount(2);
    await expect(traces.first().locator(".ui-waveform-bar")).toHaveCount(20);
    await expect(traces.first()).toHaveAttribute("role", "img");
    await expect(traces.nth(1)).toHaveClass(/ui-waveform--active/);
  });

  test("VoiceInput toggles recording state and announces it", async ({ page }) => {
    await prepare(page);
    const entry = entryFor(page, "VoiceInput");
    await entry.scrollIntoViewIfNeeded();

    const composer = entry.locator(".ui-voice-input").first();
    const toggle = composer.locator(".ui-voice-input-toggle");
    await expect(composer).toHaveClass(/ui-voice-input--idle/);
    await expect(composer.locator(".ui-voice-input-status")).toHaveText("Ready to record");

    // dispatchEvent for the same Toaster-overlap reason as the Tour trigger.
    await toggle.dispatchEvent("click");
    await expect(composer).toHaveClass(/ui-voice-input--recording/);
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
    await expect(composer.locator(".ui-voice-input-status")).toHaveText("Recording…");
    await expect(composer.locator(".ui-waveform--active")).toHaveCount(1);
    await expect(composer.locator(".ui-voice-input-elapsed")).toHaveText("0:07");

    await toggle.dispatchEvent("click");
    await expect(composer).toHaveClass(/ui-voice-input--idle/);

    // The error tile escalates to an alert.
    const error = entry.locator(".ui-voice-input--error").first();
    await expect(error.locator("[role='alert']")).toHaveText("Microphone permission denied");
  });
});
