import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

// Helper: locate the live preview for a named AI component within the gallery.
function aiPreview(page: import("@playwright/test").Page, name: string) {
  return page
    .locator("article.gallery-entry")
    .filter({ has: page.locator(`h4:text-is("${name}")`) })
    .locator(".gallery-preview--ready");
}

test.describe("StreamingText", () => {
  test("settled text and the streaming caret render", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "ai" });
    const preview = aiPreview(page, "StreamingText");

    // The block is a polite live region (role=status).
    await expect(preview.getByRole("status").first()).toBeVisible();

    // The settled tile renders the complete sentence (with the period).
    await expect(
      preview.getByText(
        "Revenue grew 18% quarter over quarter, driven mostly by enterprise renewals.",
      ),
    ).toBeVisible();

    // The streaming tile carries the blinking caret and a faded tail token.
    await expect(preview.locator(".ui-stream-caret")).toHaveCount(1);
    await expect(preview.locator(".ui-stream-token").first()).toBeVisible();
  });
});

test.describe("AiStatus", () => {
  test("renders one pill per state and the Done check", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "ai" });
    const preview = aiPreview(page, "AiStatus");

    // Five states: Idle, Thinking, Searching, Generating, Done.
    await expect(preview.locator(".ui-ai-status")).toHaveCount(5);

    // Each pill is a polite live region.
    await expect(preview.getByRole("status").first()).toBeVisible();

    // The Done tile swaps the dots for a check glyph.
    const done = preview.locator('.ui-ai-status[data-ai-state="done"]');
    await expect(done).toBeVisible();
    await expect(done.locator(".ui-ai-status-check")).toHaveCount(1);
    await expect(done.getByText("Done", { exact: true })).toBeVisible();
  });
});

test.describe("CitationChip", () => {
  test("linked chip is an anchor and no-href chip is a button", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "ai" });
    const preview = aiPreview(page, "CitationChip");

    // Linked chip -> real <a> with href + the "Citation N: <title>" name.
    const linked = preview.getByRole("link", {
      name: "Citation 1: The Rust Reference",
    });
    await expect(linked).toBeVisible();
    await expect(linked).toHaveAttribute(
      "href",
      "https://doc.rust-lang.org/reference/",
    );
    await expect(linked).toHaveClass(/ui-citation-chip/);

    // No-href chip -> <button> (the rank ai-2 fix), not a link.
    const buttonChip = preview.getByRole("button", {
      name: "Citation 3: Tokio · Internal scheduler",
    });
    await expect(buttonChip).toBeVisible();
    await expect(buttonChip).toHaveClass(/ui-citation-chip/);
  });
});

test.describe("SourceCard / SourceRail", () => {
  test("rail is an ARIA list and cards render titles and domains", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "ai" });
    const preview = aiPreview(page, "SourceCard");

    // The rail exposes role=list.
    const rail = preview.getByRole("list");
    await expect(rail).toBeVisible();

    // Card titles and the index+domain line render.
    await expect(preview.getByText("Understanding Ownership")).toBeVisible();
    await expect(preview.getByText("1 · doc.rust-lang.org")).toBeVisible();
    await expect(preview.getByText("Fearless Concurrency")).toBeVisible();

    // A linked card preserves the link role (wrapped in a listitem).
    const linkedCard = preview.getByRole("link", { name: /Understanding Ownership/ });
    await expect(linkedCard).toHaveAttribute(
      "href",
      "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html",
    );
    await expect(linkedCard).toHaveClass(/ui-source-card/);
  });
});

test.describe("PromptInput", () => {
  test("renders the textarea, a Send button, and a Stop button while streaming", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "ai" });
    const preview = aiPreview(page, "PromptInput");

    // Both composers expose a textarea labelled by the placeholder.
    const textareas = preview.locator("textarea.ui-prompt-textarea");
    await expect(textareas.first()).toBeVisible();
    await expect(textareas).toHaveCount(2);

    // The idle composer renders the send affordance.
    await expect(preview.getByRole("button", { name: "Send" })).toBeVisible();

    // The streaming composer flips the action to a Stop control.
    await expect(preview.getByRole("button", { name: "Stop" })).toBeVisible();
  });
});

test.describe("AssistantPanel", () => {
  test("renders a complementary panel hosting status, stream, and a source card", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "ai" });
    const preview = aiPreview(page, "AssistantPanel");

    // The panel is role=complementary, named by its title.
    const panel = preview.getByRole("complementary", {
      name: "Workspace assistant",
    });
    await expect(panel).toBeVisible();
    await expect(panel.locator(".ui-assistant-panel-title")).toHaveText(
      "Workspace assistant",
    );

    // It nests the Ai-native surfaces: status pill, streaming text, a source card.
    await expect(panel.locator(".ui-ai-status")).toBeVisible();
    await expect(panel.locator(".ui-stream")).toBeVisible();
    await expect(panel.getByText("Release notes · 0.7")).toBeVisible();
  });
});

test.describe("AgentTimeline", () => {
  test("renders an ordered list and marks the active step", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "ai" });
    const preview = aiPreview(page, "AgentTimeline");

    // The timeline is an ordered list of steps.
    const list = preview.locator("ol.ui-agent-timeline");
    await expect(list).toBeVisible();
    await expect(list.locator("li.ui-agent-timeline-step")).toHaveCount(5);

    // The single active step carries aria-current="step".
    const active = list.locator('li[aria-current="step"]');
    await expect(active).toHaveCount(1);
    await expect(active.locator(".ui-agent-timeline-label")).toHaveText(
      "Synthesise an answer",
    );

    // Each step mirrors its state in visually-hidden text.
    await expect(list.locator(".visually-hidden").first()).toHaveText(
      /— (pending|in progress|completed)/,
    );
  });
});
