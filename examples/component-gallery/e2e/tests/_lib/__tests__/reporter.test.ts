import { describe, expect, it } from "vitest";
import { renderTable } from "../../../reporters/audit-report.js";
import type { ManifestEntry } from "../component-manifest.js";

const manifest: ManifestEntry[] = [
  { name: "Button", slug: "button", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "Sequence", slug: "sequence", status: "ready", layers: { smoke: true, motion: true, visual: true } },
];

describe("renderTable", () => {
  it("emits a table header and one row per ready component", () => {
    const out = renderTable(manifest, new Map());
    expect(out).toContain("| Component | Smoke | Motion | Visual | Status | Notes |");
    expect(out).toContain("| Button |");
    expect(out).toContain("| Sequence |");
  });

  it("marks a component ready when every covered layer passes", () => {
    const rows = new Map([
      [
        "Button::smoke::default" as const,
        { outcome: "pass" as const, notes: [] },
      ],
      [
        "Button::visual::default" as const,
        { outcome: "pass" as const, notes: [] },
      ],
    ]);
    const out = renderTable(manifest, rows);
    const row = out.split("\n").find((line) => line.includes("| Button |"))!;
    expect(row).toContain("ready");
  });

  it("marks regression when any layer fails", () => {
    const rows = new Map([
      [
        "Sequence::motion::default" as const,
        { outcome: "fail" as const, notes: ["opacity stuck at 0"] },
      ],
    ]);
    const out = renderTable(manifest, rows);
    const row = out.split("\n").find((line) => line.includes("| Sequence |"))!;
    expect(row).toContain("regression");
    expect(row).toContain("opacity stuck at 0");
  });
});
