import { expect, test } from "@playwright/test";

// Regression suite for the "renders but doesn't animate" bug class. Each Scene
// category entry contains an animated leaf which, when wrapped in a Scene
// clock, emits `style.animationDelay = -<elapsed>ms`. The clock advances on
// rAF, so sampling the inline style at t=0 and t=250ms should yield different
// values. If both samples are equal the leaf is not being seeked — the
// SceneContext is not threaded down to that leaf and the showcase is silently
// static even though the SSR DOM looks correct.
//
// Curved Trajectory is intentionally omitted: its KineticBox uses the Sequence
// sample path (inline `transform`) rather than the cue keyframe path, and
// `gsap-tier-primitives.spec.ts` already covers its MotionPath DOM assertions.

const SCENE_SECTION = "#scene";

const SCENES: Array<{
  name: string;
  selector: string;
  description: string;
}> = [
  {
    name: "Scene · Product Intro 10s",
    selector: '[data-kinetic-id="intro-title"]',
    description: "KineticText animation-delay flows from Scene clock",
  },
  {
    name: "Scene · Split Headline",
    selector: ".ui-split-text__glyph",
    description: "SplitText glyphs animate per-stagger",
  },
  {
    name: "Scene · Lower Third Demo",
    selector: '[data-kinetic-id="lower-third-name"]',
    description: "LowerThird name animates",
  },
  {
    name: "Scene · Caption Reading-Pace Demo",
    selector: ".ui-split-text__word",
    description: "Caption per-word stagger animates",
  },
  {
    name: "Scene · Wipe Transition Demo",
    selector: ".ui-block-wipe-transition",
    description: "WipeTransition animation-delay updates",
  },
  {
    name: "Scene · Metric Counter Demo",
    selector: '[data-kinetic-id="metric-value"]',
    description: "MetricCounter value rises in",
  },
  {
    name: "Scene · Social Overlay Demo",
    selector: '[data-kinetic-id="social-overlay-handle"]',
    description: "SocialOverlay handle slides up",
  },
];

test.describe("Animation motion — leaves update animation-delay over time", () => {
  for (const scene of SCENES) {
    test(scene.name, async ({ page }) => {
      await page.goto("/");
      await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
      const card = page.locator(
        `article.gallery-entry:has(h4:has-text('${scene.name}'))`,
      );
      await expect(card).toBeVisible();

      const leaf = card.locator(scene.selector).first();
      await expect(leaf).toBeVisible();

      // WebKit's HTMLElement.style.animationDelay can return "" even when the
      // inline `style` attribute carries `animation-delay: -123ms;`. Fall back
      // to parsing the raw attribute string when the property accessor is
      // empty, so the assertion stays meaningful across both engines.
      const sample = async () =>
        await leaf.evaluate((el: HTMLElement) => {
          const direct = el.style.animationDelay;
          if (direct && direct.length > 0) return direct;
          const raw = el.getAttribute("style") ?? "";
          const match = raw.match(/animation-delay:\s*([^;]+)/i);
          return match ? match[1].trim() : "";
        });

      // Drive the clock deterministically via the transport scrubber
      // rather than relying on wall-clock autoplay timing. Wall-clock
      // sampling is fragile under WebKit's slower page-load — short-
      // duration scenes (2500-3000ms) can settle entirely before the
      // first sample, leaving both endpoint samples at the same
      // settled value even though the animation IS working.
      //
      // Setting the scrubber to two known elapsed-ms values exercises
      // the same reactivity path: clock changes → SceneContext signal
      // fires → leaf re-renders with a new animation-delay. If the
      // reactivity chain is broken, both samples are identical.
      const scrubber = card.locator("input.ui-scene-scrubber").first();
      await expect(scrubber).toBeVisible();

      const seek = async (value: string) => {
        await scrubber.evaluate(
          (el: HTMLInputElement, v: string) => {
            el.value = v;
            el.dispatchEvent(new Event("input", { bubbles: true }));
          },
          value,
        );
        // Allow Dioxus to process the Signal change + re-render.
        await page.waitForTimeout(50);
      };

      await seek("100");
      const first = await sample();
      await seek("1000");
      const second = await sample();

      expect(first).not.toBe(second);
    });
  }
});
