import type { TestInfo } from "@playwright/test";

export type AuditMode = "static" | "dev-loop";

/**
 * Extracts the audit mode (`"static"` or `"dev-loop"`) from the project's
 * metadata. Falls back to `"static"` if metadata is absent (e.g. when running
 * a single test outside of a configured project).
 */
export function getMode(testInfo: TestInfo): AuditMode {
  return (testInfo.project.metadata as { mode?: AuditMode }).mode ?? "static";
}
