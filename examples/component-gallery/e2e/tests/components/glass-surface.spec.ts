import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("GlassSurface", () => {
  test("renders the revenue operations card", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "surfaces" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("GlassSurface")') })
      .locator(".gallery-preview--ready");
    await expect(preview.getByRole("heading", { name: "Revenue operations" })).toBeVisible();
    // GlassSurface dispatches on detected GPU tier: WgpuWebGpu/WgpuWebGl2 renders via
    // LiquidSurface (canvas, no .ui-glass-surface class); SvgFilter/SolidCss/Off uses
    // the CSS fallback (with .ui-glass-surface). Either is a valid mount; assert that
    // at least one is present.
    const fallbackCount = await preview.locator(".ui-glass-surface").count();
    const canvasCount = await preview.locator("canvas").count();
    expect(fallbackCount + canvasCount).toBeGreaterThan(0);
  });

  test("solid-glass variant routes through the solid fallback", async ({ page }) => {
    await mountGallery(page, { variant: "solid-glass", hash: "surfaces" });
    const shell = page.locator(".gallery-shell");
    await expect(shell).toHaveAttribute("data-ui-glass-policy", "solid");
  });
});
