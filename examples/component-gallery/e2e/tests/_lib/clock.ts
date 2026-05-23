import type { Page } from "@playwright/test";

/**
 * Install a deterministic clock so RAF-driven previews step in known
 * increments. Returns helpers for time advancement.
 */
export async function installClock(page: Page) {
  await page.clock.install({ time: 0 });
  return {
    tickMs: async (ms: number) => {
      await page.clock.runFor(ms);
    },
    fastForward: async (ms: number) => {
      await page.clock.fastForward(ms);
    },
  };
}
