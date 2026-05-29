import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

// Covers the four newer primitives: Badge + Avatar (Surfaces) and
// Heading + Text (Foundations). Assertions mirror the live previews in
// examples/component-gallery/src/previews/{surfaces,foundations}.rs and the
// component sources in crates/ui-dioxus/src/{display,typography}.rs.

test.describe("Badge", () => {
  test("renders one pill per tone", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "surfaces" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Badge")') })
      .locator(".gallery-preview--ready");

    // One badge per tone: Neutral/Primary/Success/Warning/Danger/Info.
    await expect(preview.getByText("Draft", { exact: true })).toBeVisible();
    await expect(preview.getByText("New", { exact: true })).toBeVisible();
    await expect(preview.getByText("Active", { exact: true })).toBeVisible();
    await expect(preview.getByText("Degraded", { exact: true })).toBeVisible();
    await expect(preview.getByText("Down", { exact: true })).toBeVisible();
    await expect(preview.getByText("Beta", { exact: true })).toBeVisible();

    await expect(preview.locator("span.ui-badge")).toHaveCount(6);
  });

  test("toned badges carry their modifier class", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "surfaces" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Badge")') })
      .locator(".gallery-preview--ready");

    // Success tone -> "Active"; Danger tone -> "Down".
    await expect(preview.getByText("Active", { exact: true })).toHaveClass(
      /ui-badge--success/
    );
    await expect(preview.getByText("Down", { exact: true })).toHaveClass(
      /ui-badge--danger/
    );
  });
});

test.describe("Avatar", () => {
  test("initials avatar exposes aria-label = name", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "surfaces" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Avatar")') })
      .locator(".gallery-preview--ready");

    // Three size tiles, each with an initials + an image avatar.
    const initials = preview.locator(
      'span.ui-avatar-initials[aria-label="Ada Lovelace"]'
    );
    await expect(initials.first()).toBeVisible();
    await expect(initials.first()).toHaveText("AL");
    await expect(initials).toHaveCount(3);
  });

  test("image avatar renders an <img> with alt = name", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "surfaces" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Avatar")') })
      .locator(".gallery-preview--ready");

    const image = preview.getByRole("img", { name: "Ada Lovelace" });
    await expect(image.first()).toBeVisible();
    await expect(image.first()).toHaveClass(/ui-avatar-image/);
  });

  test("the three size modifier classes appear", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "surfaces" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Avatar")') })
      .locator(".gallery-preview--ready");

    await expect(preview.locator("span.ui-avatar--sm").first()).toBeVisible();
    await expect(preview.locator("span.ui-avatar--md").first()).toBeVisible();
    await expect(preview.locator("span.ui-avatar--lg").first()).toBeVisible();
  });
});

test.describe("Heading", () => {
  test("renders real semantic h1..h4 with the ui-heading class", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "foundations" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Heading")') })
      .locator(".gallery-preview--ready");

    // Each level keeps its semantic tag; copy is the ground truth from the
    // preview source.
    const h1 = preview.locator("h1.ui-heading");
    const h2 = preview.locator("h2.ui-heading");
    const h3 = preview.locator("h3.ui-heading");
    const h4 = preview.locator("h4.ui-heading");

    await expect(h1).toHaveText("Quarterly performance");
    await expect(h3).toHaveText("North America");
    await expect(h4).toHaveText("Enterprise accounts");

    // Level 2 has two tiles: the default and the Display-variant override.
    await expect(h2.filter({ hasText: "Revenue by region" })).toBeVisible();
    await expect(h2.filter({ hasText: "Display override" })).toBeVisible();
  });

  test("variant override keeps the semantic level but swaps the visual class", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "foundations" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Heading")') })
      .locator(".gallery-preview--ready");

    // Heading { level: 2, variant: Display } -> still an <h2>, carrying the
    // display type-scale class instead of title2.
    const override = preview
      .locator("h2.ui-heading")
      .filter({ hasText: "Display override" });
    await expect(override).toHaveClass(/ui-text--display/);
  });
});

test.describe("Text", () => {
  test("variant tiles render and carry their ui-text--<variant> class", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "foundations" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Text")') })
      .locator(".gallery-preview--ready");

    // Display tile renders as a <div> (as_element="div"); Body as a <p>.
    const display = preview.getByText("The optical top of the scale.", {
      exact: true,
    });
    await expect(display).toBeVisible();
    await expect(display).toHaveClass(/ui-text--display/);

    const body = preview.getByText(
      "Default reading size for paragraphs and prose.",
      { exact: true }
    );
    await expect(body).toBeVisible();
    await expect(body).toHaveClass(/ui-text--body/);
  });
});
