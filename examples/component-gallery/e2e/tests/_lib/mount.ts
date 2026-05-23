import type { Page, Locator } from "@playwright/test";
import { expect } from "@playwright/test";

export type Variant = "default" | "dark" | "reduced-motion" | "solid-glass";

export type MountOptions = {
  variant?: Variant;
  /** If provided, navigate to `#<slug>` after the shell is ready. */
  hash?: string;
};

const VARIANT_MEDIA: Record<
  Variant,
  { colorScheme: "light" | "dark"; reducedMotion: "no-preference" | "reduce" }
> = {
  default: { colorScheme: "light", reducedMotion: "no-preference" },
  dark: { colorScheme: "dark", reducedMotion: "no-preference" },
  "reduced-motion": { colorScheme: "light", reducedMotion: "reduce" },
  "solid-glass": { colorScheme: "light", reducedMotion: "no-preference" },
};

/**
 * Navigate to the gallery, emulate the requested media + drive the preference
 * bar so the gallery shell carries the right data-attributes. Waits for the
 * shell to be visible before returning.
 */
export async function mountGallery(
  page: Page,
  opts: MountOptions = {}
): Promise<{ shell: Locator }> {
  const variant: Variant = opts.variant ?? "default";
  const media = VARIANT_MEDIA[variant];
  await page.emulateMedia({
    colorScheme: media.colorScheme,
    reducedMotion: media.reducedMotion,
  });

  await page.goto(opts.hash ? `/#${opts.hash}` : "/");
  const shell = page.locator(".gallery-shell");
  await expect(shell).toBeVisible({ timeout: 15_000 });

  // Drive the preference bar. The four toggle groups are radio groups labelled
  // by "Theme", "Density", "Motion", "Glass". We click the radio whose
  // accessible name matches the variant.
  switch (variant) {
    case "default":
      // Defaults are already correct.
      break;
    case "dark":
      await selectRadio(page, "Theme", "Dark");
      await expect(shell).toHaveAttribute("data-ui-theme", "dark");
      break;
    case "reduced-motion":
      await selectRadio(page, "Motion", "Reduced");
      await expect(shell).toHaveAttribute("data-ui-motion", "reduced");
      break;
    case "solid-glass":
      await selectRadio(page, "Glass", "Solid");
      await expect(shell).toHaveAttribute("data-ui-glass-policy", "solid");
      break;
  }

  return { shell };
}

async function selectRadio(page: Page, groupLabel: string, optionLabel: string) {
  const group = page.getByRole("radiogroup", { name: groupLabel });
  const radio = group.getByRole("radio", { name: optionLabel });
  // The gallery's toggle groups render `<button role="radio" onclick>` —
  // NOT `<input type="radio" onchange>`. We dispatch a real click via
  // .click({ force: true }) which bypasses Playwright's actionability
  // checks (sibling tile prose can intercept the pointer in WebKit).
  await radio.click({ force: true });
}
