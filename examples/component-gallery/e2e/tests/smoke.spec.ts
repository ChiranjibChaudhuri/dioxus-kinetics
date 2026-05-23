import { test, expect } from "@playwright/test";
import { mountGallery, type Variant } from "./_lib/mount.js";
import { expectNoConsoleErrors } from "./_lib/console-guard.js";
import { COMPONENT_MANIFEST } from "./_lib/component-manifest.js";

const VARIANTS: Variant[] = ["default", "dark", "reduced-motion", "solid-glass"];

for (const variant of VARIANTS) {
  test.describe(`smoke @${variant}`, () => {
    for (const entry of COMPONENT_MANIFEST) {
      test(`${entry.name} renders without console errors`, async ({ page }, testInfo) => {
        const mode = (testInfo.project.metadata as { mode?: "static" | "dev-loop" }).mode ?? "static";
        const guard = expectNoConsoleErrors(page, { mode });
        await mountGallery(page, { variant });

        const entryNode = page
          .locator("article.gallery-entry")
          .filter({ has: page.locator(`h4:text-is("${entry.name}")`) })
          .first();

        await entryNode.scrollIntoViewIfNeeded();
        await expect(entryNode).toBeVisible();

        if (entry.status === "ready") {
          await expect(entryNode.locator(".gallery-preview--ready")).toBeVisible();
        } else {
          await expect(entryNode.locator(".gallery-preview--soon")).toBeVisible();
        }

        const code = entryNode.locator("pre.gallery-code code");
        await expect(code).toBeVisible();
        const codeText = await code.innerText();
        expect(codeText.trim().length).toBeGreaterThan(0);

        // Let the entry settle for a beat to surface any post-mount errors.
        await page.waitForTimeout(250);
        guard.assertClean();
      });
    }
  });
}
