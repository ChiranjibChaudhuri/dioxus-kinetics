import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Toast", () => {
  test("each trigger pushes a toast onto the stage", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Toast")') })
      .locator(".gallery-preview--ready");

    const stage = preview.locator(".gallery-toast-stage");
    await expect(stage.locator(".ui-toast")).toHaveCount(0);

    await preview.getByRole("button", { name: "Trigger success" }).click();
    await expect(stage.getByText("Report exported")).toBeVisible();
    await expect(stage.getByText("The PDF is ready.")).toBeVisible();

    await preview.getByRole("button", { name: "Trigger info" }).click();
    await preview.getByRole("button", { name: "Trigger warning" }).click();
    await preview.getByRole("button", { name: "Trigger error" }).click();
    await expect(stage.locator(".ui-toast")).toHaveCount(4);
  });

  test("toasts auto-dismiss after 3 seconds", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Toast")') });
    await preview.getByRole("button", { name: "Trigger success" }).click();
    const stage = preview.locator(".gallery-toast-stage");
    await expect(stage.locator(".ui-toast")).toHaveCount(1);
    await expect(stage.locator(".ui-toast")).toHaveCount(0, { timeout: 4_000 });
  });
});
