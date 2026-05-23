import type { Page } from "@playwright/test";

const DEFAULT_ALLOWLIST: RegExp[] = [];

const DEV_LOOP_ALLOWLIST: RegExp[] = [
  // dx serve hot-reload WebSocket reconnect noise; harmless in dev-loop mode.
  /WebSocket connection to 'ws:\/\/localhost:\d+\/_dioxus/,
];

export type ConsoleGuard = {
  errors: string[];
  /** Throws if any unexpected console.error fired. */
  assertClean(): void;
};

export function expectNoConsoleErrors(
  page: Page,
  opts?: { mode?: "static" | "dev-loop"; allowlist?: RegExp[] }
): ConsoleGuard {
  const errors: string[] = [];
  const allowlist = [
    ...DEFAULT_ALLOWLIST,
    ...(opts?.mode === "dev-loop" ? DEV_LOOP_ALLOWLIST : []),
    ...(opts?.allowlist ?? []),
  ];

  // Warns are intentionally tolerated — the audit's smoke layer only catches
  // hard errors (console.error + pageerror). Animation regressions emit no
  // errors and are caught by the motion + visual layers instead.
  page.on("console", (msg) => {
    if (msg.type() !== "error") return;
    const text = msg.text();
    if (allowlist.some((re) => re.test(text))) return;
    errors.push(text);
  });

  page.on("pageerror", (err) => {
    errors.push(`pageerror: ${err.message}`);
  });

  return {
    errors,
    assertClean() {
      if (errors.length > 0) {
        throw new Error(
          `Expected no console errors, got ${errors.length}:\n  - ${errors.join(
            "\n  - "
          )}`
        );
      }
    },
  };
}
