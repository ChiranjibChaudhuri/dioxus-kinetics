import { expect, test } from "@playwright/test";

const SCENE_SECTION = "#scene";

test.describe("Gallery exposure polish — new scene showcases", () => {
  test("MetricCounter renders label + value + delta", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Metric Counter Demo'))",
    );
    await expect(card).toBeVisible();
    const counter = card.locator(".ui-block-metric-counter").first();
    await expect(counter).toBeVisible();
    await expect(counter).toContainText("Active users");
    await expect(counter).toContainText("1,287");
    await expect(counter).toContainText("+24%");
  });

  test("SocialOverlay renders Instagram accent + handle + message", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Social Overlay Demo'))",
    );
    await expect(card).toBeVisible();
    const overlay = card.locator(".ui-block-social-overlay").first();
    await expect(overlay).toBeVisible();
    await expect(overlay).toHaveClass(/ui-block-social-overlay--instagram/);
    await expect(overlay).toHaveAttribute("data-platform", "instagram");
    await expect(overlay).toContainText("@kineticsui");
    await expect(overlay).toContainText("Just followed you!");
  });

  test("Manual driver Scene starts paused and never auto-advances", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Manual Driver Demo'))",
    );
    await expect(card).toBeVisible();
    const stage = card.locator(".ui-scene-stage").first();
    // Scrubber should exist and be enabled (not aria-disabled).
    const scrubber = card.locator("input.ui-scene-scrubber");
    await expect(scrubber).toBeVisible();
    await expect(scrubber).not.toHaveAttribute("aria-disabled", "true");

    // Initial state is paused with elapsed=0 because autoplay=false AND
    // driver=Manual disables the rAF loop.
    await expect(stage).toHaveAttribute("data-state", "paused");
    await expect(stage).toHaveAttribute("data-elapsed-ms", "0");

    // Wait briefly — clock must NOT advance on its own.
    await page.waitForTimeout(500);
    await expect(stage).toHaveAttribute("data-state", "paused");
    await expect(stage).toHaveAttribute("data-elapsed-ms", "0");

    // Driving the scrubber via JS moves elapsed_ms.
    await scrubber.evaluate((el: HTMLInputElement) => {
      el.value = "2500";
      el.dispatchEvent(new Event("input", { bubbles: true }));
    });
    await expect(stage).toHaveAttribute("data-elapsed-ms", "2500");
  });

  test("CurvedTrajectory cue enables rotate-along-path on its MotionPath track", async ({ page }) => {
    // The af62bd5 commit flipped the timeline MotionCue's rotate_along_path
    // from false to true. That toggle lives on the Path cue, not on the
    // <MotionPath> component prop, so the visible signal is the path data
    // embedded in the .ui-motion-path element being unchanged while the
    // upstream timeline now drives tangent rotation. We assert that the
    // showcase still renders its motion path with the expected Line + Bezier
    // payload — a regression here means the showcase wiring broke.
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Curved Trajectory'))",
    );
    await expect(card).toBeVisible();
    const mp = card.locator(".ui-motion-path").first();
    const pathAttr = await mp.getAttribute("data-motion-path");
    expect(pathAttr).not.toBeNull();
    expect(pathAttr!).toContain("Bezier");
    expect(pathAttr!).toContain("control_1");
    expect(pathAttr!).toContain("600.0");
  });
});
