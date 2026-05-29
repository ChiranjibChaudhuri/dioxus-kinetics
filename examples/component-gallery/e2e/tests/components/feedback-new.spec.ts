import { test, expect } from "@playwright/test";
import type { Page, Locator } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

const entry = (page: Page, name: string): Locator =>
  page
    .locator("article.gallery-entry")
    .filter({ has: page.locator(`h4:text-is("${name}")`) })
    .locator(".gallery-preview--ready");

test.describe("Toaster", () => {
  test("renders the fixed toast region with the seeded toasts", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });

    const preview = entry(page, "Toaster");
    const region = preview.locator(".ui-toast-region");
    await expect(region).toBeVisible();

    // The Toaster pre-seeds one entry per tone; assert seeded titles render.
    await expect(region.getByText("Report exported", { exact: true })).toBeVisible();
    await expect(region.getByText("Sync started", { exact: true })).toBeVisible();
    await expect(region.getByText("Quota close", { exact: true })).toBeVisible();
  });

  test("the push trigger appends a new toast to the region", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });

    const preview = entry(page, "Toaster");
    await preview.getByRole("button", { name: "Push success" }).click();

    // "Push success" enqueues a toast titled "Saved" into the region.
    await expect(preview.locator(".ui-toast-region").getByText("Saved", { exact: true })).toBeVisible();
  });
});

test.describe("Spinner", () => {
  test("renders a status spinner with an aria-label", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });

    const preview = entry(page, "Spinner");
    const spinner = preview.locator(".ui-spinner").first();
    await expect(spinner).toBeVisible();
    await expect(spinner).toHaveAttribute("role", "status");
    await expect(spinner).toHaveAttribute("aria-label", "Loading workspace");

    // The status role + accessible name make it reachable via getByRole.
    await expect(preview.getByRole("status", { name: "Loading workspace" })).toBeVisible();
  });
});

test.describe("Sheet", () => {
  test("opens as a modal dialog showing its title", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });

    const preview = entry(page, "Sheet");
    const sheet = preview.locator("aside.ui-sheet");
    await expect(sheet).toBeVisible();
    await expect(sheet).toHaveAttribute("role", "dialog");
    await expect(sheet).toHaveAttribute("aria-modal", "true");
    await expect(sheet).toHaveAttribute("aria-label", "Edit filters");
    await expect(sheet.locator("h2.ui-sheet-title")).toHaveText("Edit filters");
  });

  test("Escape dismisses the sheet and the reopen trigger restores it", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });

    const preview = entry(page, "Sheet");
    const sheet = preview.locator("aside.ui-sheet");
    await expect(sheet).toBeVisible();

    // The panel pulls focus on mount and traps Tab; Escape fires on_dismiss,
    // which toggles `open` to false so the sheet returns an empty tree.
    await sheet.press("Escape");
    await expect(sheet).toHaveCount(0);

    // The reopen trigger flips `open` back to true and re-renders the panel.
    await preview.getByRole("button", { name: "Reopen" }).click();
    await expect(preview.locator("aside.ui-sheet")).toBeVisible();
  });

  test("the close button dismisses the sheet", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });

    const preview = entry(page, "Sheet");
    await expect(preview.locator("aside.ui-sheet")).toBeVisible();

    await preview.getByRole("button", { name: "Close" }).click();
    await expect(preview.locator("aside.ui-sheet")).toHaveCount(0);
  });
});
