import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("RadioGroup", () => {
  test("renders three radio inputs sharing the same name", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("RadioGroup")') })
      .locator(".gallery-preview--ready");

    const inputs = preview.locator('input[type="radio"]');
    await expect(inputs).toHaveCount(3);
    for (let i = 0; i < 3; i++) {
      await expect(inputs.nth(i)).toHaveAttribute("name", "billing-plan");
    }

    // Seeded value is "monthly" — first option checked.
    await expect(inputs.nth(0)).toBeChecked();
    await expect(inputs.nth(1)).not.toBeChecked();
    await expect(inputs.nth(2)).not.toBeChecked();
  });

  test("clicking a different option moves the selection (native mutex)", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("RadioGroup")') })
      .locator(".gallery-preview--ready");

    const inputs = preview.locator('input[type="radio"]');
    await inputs.nth(2).click();

    await expect(inputs.nth(0)).not.toBeChecked();
    await expect(inputs.nth(1)).not.toBeChecked();
    await expect(inputs.nth(2)).toBeChecked();
  });

  test("legend and descriptions render alongside each option", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    // Scope to the preview only — the code-snippet block contains identical
    // strings as Rust source, so an unscoped getByText matches twice.
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("RadioGroup")') })
      .locator(".gallery-preview--ready");

    await expect(preview.locator("legend")).toContainText("Billing plan");
    await expect(preview.getByText("Billed on the first of every month")).toBeVisible();
    await expect(preview.getByText("Save 18% versus monthly")).toBeVisible();
  });
});
