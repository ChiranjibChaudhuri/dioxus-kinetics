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
        "Button::smoke::default::Button renders without error" as const,
        { outcome: "pass" as const, notes: [] },
      ],
      [
        "Button::visual::default::Button matches snapshot" as const,
        { outcome: "pass" as const, notes: [] },
      ],
    ]);
    const out = renderTable(manifest, rows as any);
    const row = out.split("\n").find((line) => line.includes("| Button |"))!;
    expect(row).toContain("ready");
  });

  it("marks regression when any layer fails", () => {
    const rows = new Map([
      [
        "Sequence::motion::default::Sequence animates correctly" as const,
        { outcome: "fail" as const, notes: ["opacity stuck at 0"] },
      ],
    ]);
    const out = renderTable(manifest, rows as any);
    const row = out.split("\n").find((line) => line.includes("| Sequence |"))!;
    expect(row).toContain("regression");
    expect(row).toContain("opacity stuck at 0");
  });

  it("does not overwrite cells when two tests share (name, layer, variant)", () => {
    const rows = new Map<string, { outcome: "pass" | "fail" | "flaky" | "skipped"; notes: string[] }>([
      [
        "Sequence::motion::default::scrubbing 0 to 560 ms animates the three children",
        { outcome: "fail" as const, notes: ["opacity stuck at 0"] },
      ],
      [
        "Sequence::motion::default::reduced motion keeps the sequence at its settled state at t=0",
        { outcome: "pass" as const, notes: [] },
      ],
    ]);
    const out = renderTable(manifest, rows as any);
    const row = out.split("\n").find((line) => line.includes("| Sequence |"))!;
    expect(row).toContain("fail");
    expect(row).toContain("opacity stuck at 0");
  });
});
