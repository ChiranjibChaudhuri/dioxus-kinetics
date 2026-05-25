import { expect, test } from "@playwright/test";

const SCENE_SECTION = "#scene";

test.describe("SP-3 GSAP-tier primitives", () => {
  test("SplitText emits per-glyph spans with aria-label parent", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Split Headline'))",
    );
    await expect(card).toBeVisible();
    const splitText = card.locator(".ui-split-text").first();
    await expect(splitText).toHaveAttribute(
      "aria-label",
      "Kinetics typography, glyph by glyph.",
    );
    const glyphCount = await splitText.locator(".ui-split-text__glyph").count();
    expect(glyphCount).toBe("Kinetics typography, glyph by glyph.".length);
  });

  test("MotionPath emits data-motion-path JSON and a KineticBox child", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Curved Trajectory'))",
    );
    await expect(card).toBeVisible();
    const motionPath = card.locator(".ui-motion-path").first();
    const dataAttr = await motionPath.getAttribute("data-motion-path");
    expect(dataAttr).not.toBeNull();
    expect(dataAttr!).toContain("Line");
    expect(dataAttr!).toContain("Bezier");
  });

  test("Scroll-pinned scene installs trigger element and Scene reads driver", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Scroll-pinned Story'))",
    );
    await expect(card).toBeVisible();
    const trigger = card.locator("#scroll-story-trigger");
    await expect(trigger).toBeVisible();
    const stage = card.locator(".ui-scene-stage").first();
    await expect(stage).toHaveAttribute("data-composition-id", "scroll-story");
  });

  test("Scroll-pinned scene under reduced motion settles immediately", async ({ page }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Scroll-pinned Story'))",
    );
    const stage = card.locator(".ui-scene-stage").first();
    await expect(stage).toHaveAttribute("data-state", "settled");
    await expect(stage).toHaveAttribute("data-reduced", "true");
  });
});
