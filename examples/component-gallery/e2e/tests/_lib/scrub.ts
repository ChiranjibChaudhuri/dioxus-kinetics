import type { Locator, Page } from "@playwright/test";
import { expect } from "@playwright/test";

/**
 * Drive a ScrubFrame's range input by setting its value and dispatching
 * `input`. Returns the elapsed value read back from the
 * `.gallery-demo-frame-elapsed` span (parsed integer ms).
 */
export async function scrubTo(
  page: Page,
  frame: Locator,
  ms: number
): Promise<number> {
  const slider = frame.locator('input[type="range"]');
  await slider.evaluate((el, value) => {
    const input = el as HTMLInputElement;
    input.value = String(value);
    input.dispatchEvent(new Event("input", { bubbles: true }));
    input.dispatchEvent(new Event("change", { bubbles: true }));
  }, Math.round(ms));

  const elapsedText = await frame
    .locator(".gallery-demo-frame-elapsed")
    .innerText();
  const match = elapsedText.match(/^(\d+(?:\.\d+)?)/);
  expect(
    match,
    `expected '.gallery-demo-frame-elapsed' to start with a number, got: ${elapsedText}`
  ).toBeTruthy();
  return Number.parseFloat(match![1]);
}

/** Click the ReplayFrame's "Replay" button to restart its RAF loop. */
export async function clickReplay(frame: Locator): Promise<void> {
  await frame.getByRole("button", { name: "Replay" }).click();
}
