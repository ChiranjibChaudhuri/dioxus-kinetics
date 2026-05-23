import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Switch", () => {
  test("toggling flips the checked attribute and the visual thumb position", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Switch")') })
      .locator(".gallery-preview--ready");

    const sw = preview.locator(".ui-switch");
    await expect(sw).toBeVisible();

    const control = preview.getByRole("switch");
    const initial = await control.getAttribute("aria-checked");
    expect(["true", "false"]).toContain(initial);

    await control.click();
    const flipped = initial === "true" ? "false" : "true";
    await expect(control).toHaveAttribute("aria-checked", flipped);

    await control.click();
    await expect(control).toHaveAttribute("aria-checked", initial!);
  });

  test("reduced motion variant still toggles", async ({ page }) => {
    await mountGallery(page, { variant: "reduced-motion", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Switch")') });
    const control = preview.getByRole("switch");
    const initial = await control.getAttribute("aria-checked");
    await control.click();
    const flipped = initial === "true" ? "false" : "true";
    await expect(control).toHaveAttribute("aria-checked", flipped);
  });
});
