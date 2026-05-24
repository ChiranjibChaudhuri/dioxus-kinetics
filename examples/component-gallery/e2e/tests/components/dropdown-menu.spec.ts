import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("DropdownMenu", () => {
  test("menu is open by default with menuitem rows and a separator", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("DropdownMenu")') })
      .locator(".gallery-preview--ready");

    const menu = preview.locator('[role="menu"]');
    await expect(menu).toBeVisible();

    const items = menu.locator('[role="menuitem"]');
    // 4 menuitems: Rename, Duplicate, Archive (disabled), Delete.
    await expect(items).toHaveCount(4);

    // The separator divider sits between Duplicate and Archive.
    const separator = menu.locator('[role="separator"]');
    await expect(separator).toHaveCount(1);
  });

  test("disabled menuitem reports aria-disabled and the native disabled attribute", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("DropdownMenu")') })
      .locator(".gallery-preview--ready");

    const archive = preview.locator('[role="menuitem"]', { hasText: "Archive" });
    await expect(archive).toHaveAttribute("aria-disabled", "true");
    await expect(archive).toBeDisabled();
  });

  test("clicking an enabled menuitem closes the menu", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("DropdownMenu")') })
      .locator(".gallery-preview--ready");

    const rename = preview.locator('[role="menuitem"]', { hasText: "Rename" });
    await rename.click();

    // After click the panel is removed by Popover.
    await expect(preview.locator('[role="menu"]')).toHaveCount(0);
  });
});
