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

        // Mask any canvas in the preview — WebGPU/WebGL driver pixels are
        // non-deterministic across machines, and `GlassSurface` may also
        // render via `LiquidSurface` internally on WgpuWebGl2 tiers.
        // Locators that resolve to zero elements contribute nothing.
        const masks = [preview.locator("canvas")];

        // 15s timeout accommodates first-paint of canvas-backed previews
        // (LiquidSurface, GlassSurface in WgpuWebGl2 tier) on webkit when
        // wasm-opt failed at build time and the bundle is unoptimized.
        await expect(preview).toHaveScreenshot(
          `${entry.slug}/${variant}.png`,
          { mask: masks, timeout: 15_000 }
        );
      });
    }
  });
}
