import { expect, test } from "@playwright/test";

// The three SP-4+5+6 ui-blocks showcases live under the Scene category. Each
// gallery entry is rendered as an `article.gallery-entry` whose <h4> heading
// carries the entry name; we scope every locator to that card so the spec is
// not coupled to other Scene fragments that may be added later.
const SCENE_SECTION = "#scene";

test.describe("SP-4+5+6 ui-blocks catalog showcases", () => {
  test("LowerThird emits aria-label with name and role", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Lower Third Demo'))",
    );
    await expect(card).toBeVisible();
    const lowerThird = card.locator(".ui-block-lower-third").first();
    await expect(lowerThird).toHaveAttribute(
      "aria-label",
      "Ada Lovelace, Mathematician",
    );
  });

  test("Caption emits per-word SplitText spans", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Caption Reading-Pace Demo'))",
    );
    await expect(card).toBeVisible();
    const caption = card.locator(".ui-block-caption").first();
    // "Built with kinetics ui-blocks." => 4 whitespace-delimited word tokens.
    // The actual emitted CSS class is `.ui-split-text__word` (BEM
    // double-underscore), matching the SP-3 rename also used by
    // `.ui-split-text__glyph` for the character split mode.
    const wordCount = await caption.locator(".ui-split-text__word").count();
    expect(wordCount).toBe(4);
  });

  test("WipeTransition emits mask-image inline style + data-angle-deg=120", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Wipe Transition Demo'))",
    );
    await expect(card).toBeVisible();
    const wipe = card.locator(".ui-block-wipe-transition").first();
    await expect(wipe).toHaveAttribute("data-angle-deg", "120");
    const inlineStyle = (await wipe.getAttribute("style")) ?? "";
    expect(
      inlineStyle.includes("mask-image") ||
        inlineStyle.includes("-webkit-mask-image"),
    ).toBe(true);
  });
});
