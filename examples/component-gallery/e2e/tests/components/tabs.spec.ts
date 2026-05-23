import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Tabs", () => {
  test("clicking a tab updates the selected panel", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "layout" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tabs")') })
      .locator(".gallery-preview--ready");

    await expect(preview.getByText("Payment method active")).toBeVisible();

    await preview.getByRole("tab", { name: "Overview" }).click();
    await expect(preview.getByText("Account summary")).toBeVisible();

    await preview.getByRole("tab", { name: "Usage" }).click();
    await expect(preview.getByText("92% of monthly quota used")).toBeVisible();
  });

  test("aria-selected toggles across tabs", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "layout" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tabs")') });
    const billing = preview.getByRole("tab", { name: "Billing" });
    const overview = preview.getByRole("tab", { name: "Overview" });

    await expect(billing).toHaveAttribute("aria-selected", "true");
    await overview.click();
    await expect(overview).toHaveAttribute("aria-selected", "true");
    await expect(billing).toHaveAttribute("aria-selected", "false");
  });
});
