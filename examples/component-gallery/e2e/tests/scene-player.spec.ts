import { expect, test } from "@playwright/test";

// The gallery's Scene category renders the Product Intro 10s scene inside an
// `article.gallery-entry` whose <h4> heading carries the entry name. We scope
// every selector to that card so the spec does not accidentally bind to other
// Scene fragments that may be added later.
const SCENE_SECTION = "#scene";
const ENTRY_NAME = "Scene · Product Intro 10s";

test.describe("Scene player", () => {
  test("renders transport controls and scrubs to settled", async ({ page }) => {
    await page.goto("/");

    const section = page.locator(SCENE_SECTION);
    await section.scrollIntoViewIfNeeded();
    await expect(section).toBeVisible();

    const card = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator(`h4:text-is("${ENTRY_NAME}")`) })
      .first();
    await expect(card).toBeVisible();

    const scrubber = card.locator("input.ui-scene-scrubber");
    await expect(scrubber).toBeVisible();
    await expect(scrubber).toHaveAttribute("min", "0");
    await expect(scrubber).toHaveAttribute("max", "10000");

    const stage = card.locator(".ui-scene-stage");

    // Drive the scrubber to the end. We dispatch both `input` and `change`
    // because WebKit's <input type=range> does not always fire `input` from
    // a programmatic `.value=` assignment.
    await scrubber.evaluate((el: HTMLInputElement) => {
      el.value = "10000";
      el.dispatchEvent(new Event("input", { bubbles: true }));
      el.dispatchEvent(new Event("change", { bubbles: true }));
    });

    await expect(stage).toHaveAttribute("data-state", "settled");
    await expect(stage).toHaveAttribute("data-elapsed-ms", "10000");
  });

  test("reduced-motion disables scrubber and shows tag", async ({ page }) => {
    // Emulate `prefers-reduced-motion: reduce` BEFORE navigation so the Scene
    // component's one-shot `use_reduced_motion()` probe at mount picks up the
    // reduced state. Toggling the gallery preference bar after mount does not
    // re-thread the Scene's frozen `clock.reduced` signal — the OS-level media
    // query is the authoritative channel the component reads at construction.
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto("/");

    // Sanity-check that the gallery shell mirrors the OS preference into its
    // `data-ui-motion` attribute (the PreferenceBar honours the media query at
    // startup when no stored override exists).
    await expect(page.locator(".gallery-shell")).toHaveAttribute(
      "data-ui-motion",
      "reduced"
    );

    const card = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator(`h4:text-is("${ENTRY_NAME}")`) })
      .first();
    await card.scrollIntoViewIfNeeded();

    const scrubber = card.locator("input.ui-scene-scrubber");
    await expect(scrubber).toHaveAttribute("aria-disabled", "true");
    await expect(card.locator(".ui-scene-reduced-tag")).toBeVisible();

    const stage = card.locator(".ui-scene-stage");
    await expect(stage).toHaveAttribute("data-state", "settled");
    await expect(stage).toHaveAttribute("data-reduced", "true");
  });
});
