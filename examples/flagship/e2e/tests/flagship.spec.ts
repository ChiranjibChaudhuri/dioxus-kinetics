import { test, expect } from "@playwright/test";

test.describe("flagship marketing page", () => {
  test("has zero gallery-shell markers", async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto("/");

    await expect(page.locator(".flagship-shell")).toBeVisible();

    expect(await page.locator(".gallery-rail").count()).toBe(0);
    expect(await page.locator(".gallery-controls").count()).toBe(0);
    expect(await page.locator(".gallery-entry").count()).toBe(0);
    expect(await page.locator(".gallery-code").count()).toBe(0);

    await page.waitForTimeout(500);
    expect(consoleErrors, `console errors: ${consoleErrors.join("\n")}`).toHaveLength(0);
  });

  test("hero scene root mounts", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator(".flagship-hero")).toBeVisible();
    // The Scene component in ui-dioxus/src/scene_player.rs emits
    // `data-composition-id` (not `data-scene-id`) as the attribute on the
    // rendered <section> element. The value matches the `id` prop verbatim.
    await expect(page.locator('[data-composition-id="product-intro"]').first()).toBeAttached();
  });

  test("five sections are present", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator(".flagship-hero")).toBeVisible();
    await expect(page.locator(".flagship-story")).toBeAttached();
    await expect(page.locator(".flagship-features")).toBeAttached();
    await expect(page.locator(".flagship-metrics")).toBeAttached();
    await expect(page.locator(".flagship-cta")).toBeAttached();
  });
});
