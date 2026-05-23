import { test, expect } from "@playwright/test";
import { mountGallery, type Variant } from "./_lib/mount.js";
import { readyComponents } from "./_lib/component-manifest.js";

const VARIANTS: Variant[] = ["default", "dark", "reduced-motion", "solid-glass"];

for (const variant of VARIANTS) {
  test.describe(`visual @${variant}`, () => {
    for (const entry of readyComponents()) {
      if (!entry.layers.visual) continue;
      test(`${entry.name} preview matches snapshot`, async ({ page }) => {
        await mountGallery(page, { variant });
        const preview = page
          .locator("article.gallery-entry")
          .filter({ has: page.locator(`h4:text-is("${entry.name}")`) })
          .locator(".gallery-preview");

        await preview.scrollIntoViewIfNeeded();
        await expect(preview).toBeVisible();

        // Mask LiquidSurface canvas because WebGPU driver pixels are non-
        // deterministic across machines.
        const masks =
          entry.name === "LiquidSurface" ? [preview.locator("canvas")] : [];

        await expect(preview).toHaveScreenshot(
          `${entry.slug}/${variant}.png`,
          { mask: masks }
        );
      });
    }
  });
}
